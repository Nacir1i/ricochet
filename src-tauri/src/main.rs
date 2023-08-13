#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod file_reader;
mod file_watcher;
mod state;

use state::AppState;
use std::sync::{Mutex, OnceLock};
use std::{env, path::PathBuf};
use tauri::{Manager, State, Window};

static DIRECTORY_PATH: OnceLock<String> = OnceLock::new();
static WINDOW: OnceLock<Window> = OnceLock::new();

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

fn emit_tauri_event(data: file_reader::Data, event: &str) {
    let window = WINDOW.get().expect("Window is un available");

    window
        .emit(
            event,
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

#[tauri::command]
fn update_dir_path(path: String, state: State<AppState>) {
    let state_file_watcher = state.file_watcher_handler.lock().unwrap().clone().unwrap();

    let _ = state_file_watcher.send(PathBuf::from(path));
}

fn main() {
    _ = DIRECTORY_PATH.set("/home/linuxlolrandomxd/Desktop/scenarios".to_owned());
    let mut file_data = FileData::new();

    let start = std::time::Instant::now();
    file_reader::read_existing_files(&mut file_data);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            file_watcher_handler: Mutex::new(None),
        })
        .setup(|app| {
            let handle = app.handle();
            let window = app.get_window("main").unwrap();
            let app_state: State<AppState> = handle.state();

            _ = WINDOW.set(window);

            let db =
                database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            let file_watcher_handler = file_watcher::file_watcher_thread();

            let mut file_watcher = app_state.file_watcher_handler.lock().unwrap();

            *file_watcher = Some(file_watcher_handler);

            Ok(())
        })
        .manage(file_data)
        .invoke_handler(tauri::generate_handler![fetch_data, update_dir_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
