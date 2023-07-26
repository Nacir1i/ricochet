#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify::{event::CreateKind, *};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path, path::PathBuf, str, time::Duration};
use tauri::{Manager, Window};

// const DIRECTORY_PATH: &str = "C:/Users/LolRandomXD/Desktop/dev/rustTest/csv/test";
const DIRECTORY_PATH: &str = "C:/Users/LolRandomXD/Desktop/KovaaKs.FPS.Aim.Trainer/KovaaKs.FPS.Aim.Trainer/FPSAimTrainer/stats";

#[derive(Debug)]
struct FileData(std::sync::Mutex<Vec<Data>>);

impl FileData {
    pub fn new() -> Self {
        FileData(std::sync::Mutex::new(Vec::new()))
    }

    pub fn add_file_data(&mut self, data: Data) {
        let mut file_data = self.0.lock().unwrap();
        file_data.push(data);
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct Data {
    tiles: Vec<TilesRecords>,
    key_value: Vec<KeyValueRecord>,
    stats: Stats,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Timestamp {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TilesRecords {
    #[serde(deserialize_with = "csv::invalid_option")]
    kill: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option")]
    timestamp: Option<Timestamp>,
    #[serde(deserialize_with = "csv::invalid_option")]
    bot: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    weapon: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ttk: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    shots: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option")]
    accuracy: Option<f32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    damage_done: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    damage_taken: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    efficiency: Option<f32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    cheated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct KeyValueRecord {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct Stats {
    weapon: String,
    shots: u16,
    hits: u16,
    damage_done: f32,
    damage_possible: f32,
}

fn read_file(
    path: &PathBuf,
) -> core::result::Result<(Vec<TilesRecords>, Vec<KeyValueRecord>, Stats), Box<dyn Error>> {
    let mut tile_record_list: Vec<TilesRecords> = Vec::new();
    let mut key_value_record_list: Vec<KeyValueRecord> = Vec::new();
    let mut stats_record_list: Stats = Stats::default();
    let validation_vec = vec![
        "Kills:",
        "Score:",
        "Hash:",
        "Deaths:",
        "Scenario:",
        "FOV:",
        "Horiz Sens:",
    ];

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .double_quote(false)
        .flexible(true)
        .from_path(path)?;

    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => return Err(err.into()),
        };

        if record.len() == 12 {
            let tile_record: TilesRecords = record.deserialize(None)?;
            tile_record_list.push(tile_record);
        } else if record.len() == 2 && validation_vec.contains(&&record[0]) {
            let key_value_record: KeyValueRecord = record.deserialize(None)?;
            key_value_record_list.push(key_value_record);
        } else if record.len() == 6 {
            let stats_record: Stats = record.deserialize(None)?;
            stats_record_list = stats_record;
        }
    }

    Ok((tile_record_list, key_value_record_list, stats_record_list))
}

// #[tauri::command]
fn read_existing_files(file_data: &mut FileData) {
    let dir_entries = std::fs::read_dir(DIRECTORY_PATH).unwrap();

    for file in dir_entries {
        match file {
            Ok(file_entry) => {
                if let Ok((tiles, key_value, stats)) = read_file(&file_entry.path()) {
                    let data = Data {
                        tiles,
                        key_value,
                        stats,
                    };

                    file_data.add_file_data(data);
                } else {
                    eprintln!("Error reading file: {:?}", file_entry.path());
                }
            }
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
            }
        }
    }
}

fn emit_tauri_event(window: &Window, data: Data) {
    let string_data = serde_json::to_string(&data).unwrap();

    window
        .emit(
            "event-name",
            Payload {
                message: "event sent".to_owned(),
                data: string_data,
            },
        )
        .unwrap()
}

fn file_watcher_thread(window: &Window) {
    let (tx, rx) = std::sync::mpsc::channel();
    let main_window = window.clone();

    std::thread::spawn(move || {
        let mut watcher: Box<dyn Watcher> =
            if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                let config = Config::default().with_poll_interval(Duration::from_secs(1));
                Box::new(PollWatcher::new(tx, config).unwrap())
            } else {
                Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
            };

        watcher
            .watch(Path::new(DIRECTORY_PATH), RecursiveMode::Recursive)
            .unwrap();

        for event in rx {
            match event {
                Ok(event) => match event.kind {
                    EventKind::Create(CreateKind::Any) => {
                        println!("{:?}", event.kind);

                        match read_file(&event.paths.as_slice()[0]) {
                            Ok((tiles, key_value, stats)) => {
                                let data = Data {
                                    tiles,
                                    key_value,
                                    stats,
                                };
                                emit_tauri_event(&main_window, data);
                            }
                            Err(err) => {
                                eprintln!("Error reading file: {}", err);
                            }
                        }
                    }
                    _ => (),
                },
                Err(err) => {
                    eprintln!("Error receiving event: {}", err);
                }
            }
        }
    });
}

#[tauri::command]
fn fetch_data(page: u8, limit: u8, file_data: tauri::State<FileData>) -> Option<Vec<Data>> {
    let locked_file_data = file_data.0.lock().unwrap();

    let start_index = (page - 1) as usize * limit as usize;

    let end_index = std::cmp::min(
        start_index + limit as usize,
        locked_file_data.len() as usize,
    );

    if start_index >= locked_file_data.len() {
        return None;
    }

    let data_slice = &locked_file_data[start_index..end_index];

    let mut data: Vec<Data> = Vec::new();
    data.extend_from_slice(data_slice);

    return Some(data);
}

fn main() {
    let mut file_data = FileData::new();

    let start = std::time::Instant::now();
    read_existing_files(&mut file_data);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    tauri::Builder::default()
        .setup(|app| {
            Ok({
                let main_window = app.get_window("main").unwrap();
                file_watcher_thread(&main_window);
            })
        })
        .manage(file_data)
        .invoke_handler(tauri::generate_handler![fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
