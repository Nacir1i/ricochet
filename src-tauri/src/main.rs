#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify::{event::CreateKind, *};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path, path::PathBuf, str, time::Duration};
use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct Data {
    tiles: Vec<String>,
    key_value: Vec<String>,
    stats: Vec<String>,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    data: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Timestamp {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
struct KeyValueRecord {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
    weapon: String,
    shots: u16,
    hits: u16,
    damage_done: f32,
    damage_possible: f32,
}

fn read_file(
    path: &PathBuf,
) -> core::result::Result<(Vec<String>, Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut tile_record_list: Vec<String> = Vec::new();
    let mut key_value_record_list: Vec<String> = Vec::new();
    let mut stats_record_list: Vec<String> = Vec::new();
    let validation_vec = vec!["Kills:", "Score:", "Hash:", "Deaths:", "Scenario:"];

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .double_quote(false)
        .flexible(true)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        if record.len() == 12 {
            let tile_record: TilesRecords = record.deserialize(None)?;
            let tile_record_serialized = serde_json::to_string(&tile_record).unwrap();
            tile_record_list.push(tile_record_serialized);
        } else if record.len() == 2 && validation_vec.contains(&&record[0]) {
            let key_value_record: KeyValueRecord = record.deserialize(None)?;
            let key_vale_serialized = serde_json::to_string(&key_value_record).unwrap();
            key_value_record_list.push(key_vale_serialized);
        } else if record.len() == 6 {
            let stats_record: Stats = record.deserialize(None)?;
            let stats_serialized = serde_json::to_string(&stats_record).unwrap();
            stats_record_list.push(stats_serialized);
        }
    }

    Ok((tile_record_list, key_value_record_list, stats_record_list))
}

#[tauri::command]
fn read_existing_files(window: Window) {
    for file in std::fs::read_dir("C:/Users/LolRandomXD/Desktop/dev/rustTest/csv/test").unwrap() {
        if let Ok((tiles, key_value, stats)) = read_file(&file.unwrap().path()) {
            let data = Data {
                tiles,
                key_value,
                stats,
            };
            emit_tauri_event(&window, data);
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
            .watch(
                Path::new("C:/Users/LolRandomXD/Desktop/dev/rustTest/csv/test"),
                RecursiveMode::Recursive,
            )
            .unwrap();

        for event in rx {
            if let Ok(event) = event {
                match event.kind {
                    EventKind::Create(CreateKind::Any) => {
                        if let Ok((tiles, key_value, stats)) = read_file(&event.paths.as_slice()[0])
                        {
                            let data = Data {
                                tiles,
                                key_value,
                                stats,
                            };
                            emit_tauri_event(&main_window, data);
                        }
                    }
                    _ => (),
                }
            }
        }
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            Ok({
                let main_window = app.get_window("main").unwrap();
                file_watcher_thread(&main_window);
            })
        })
        .invoke_handler(tauri::generate_handler![read_existing_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
