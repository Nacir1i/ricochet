#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use diesel::prelude::*;
use std::env;
use std::str;
use tauri::{Manager, Window};

mod file_reader;
mod file_watcher;
mod models;
mod schema;

// const DIRECTORY_PATH: &str = "C:/Users/LolRandomXD/Desktop/dev/rustTest/csv/test";
const DIRECTORY_PATH: &str = "C:/Users/LolRandomXD/Desktop/KovaaKs.FPS.Aim.Trainer/KovaaKs.FPS.Aim.Trainer/FPSAimTrainer/stats";

#[derive(Debug)]
pub struct FileData(std::sync::Mutex<Vec<file_reader::Data>>);

impl FileData {
    pub fn new() -> Self {
        FileData(std::sync::Mutex::new(Vec::new()))
    }

    pub fn add_file_data(&mut self, data: file_reader::Data) {
        let mut file_data = self.0.lock().unwrap();
        file_data.push(data);
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    data: file_reader::Data,
}

pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn emit_tauri_event(window: &Window, data: file_reader::Data) {
    window
        .emit(
            "event-name",
            Payload {
                message: "event sent".to_owned(),
                data,
            },
        )
        .unwrap()
}

#[tauri::command]
fn fetch_data(
    page: u8,
    limit: u8,
    file_data: tauri::State<FileData>,
) -> Option<Vec<file_reader::Data>> {
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

    let mut data: Vec<file_reader::Data> = Vec::new();
    data.extend_from_slice(data_slice);

    return Some(data);
}

fn main() {
    let mut file_data = FileData::new();

    let start = std::time::Instant::now();
    file_reader::read_existing_files(&mut file_data);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    tauri::Builder::default()
        .setup(|app| {
            Ok({
                let main_window = app.get_window("main").unwrap();
                file_watcher::file_watcher_thread(&main_window);
            })
        })
        .manage(file_data)
        .invoke_handler(tauri::generate_handler![fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
