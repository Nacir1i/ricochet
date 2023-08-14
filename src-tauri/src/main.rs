#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod file_reader;
mod file_watcher;
mod state;

use state::{AppState, ServiceAccess};
use std::sync::{Mutex, OnceLock};
use std::{env, path::PathBuf};
use tauri::{AppHandle, Manager, State, Window};

static WINDOW: OnceLock<Window> = OnceLock::new();

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

// page: u8, limit: u8, state: State<AppState>
#[tauri::command]
fn fetch_data() {}

#[tauri::command]
fn update_dir_path(path: String, app_handle: AppHandle) {
    app_handle
        .db(|db| {
            database::update_settings(
                database::Settings {
                    directory_path: path.to_owned(),
                },
                db,
            )
        })
        .unwrap();

    app_handle.file_watcher_handler(|file_watcher| {
        let _ = file_watcher.send(PathBuf::from(path));
    })
}

fn main() {
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

            let mut db = database::initialize_database(&handle)
                .expect("[Main]::Database initialize should succeed");

            let settings = database::get_settings(&db).unwrap_or(database::Settings {
                directory_path: "/home/linuxlolrandomxd/Desktop/test".to_owned(),
            });

            let start = std::time::Instant::now();
            let data_vec = file_reader::read_existing_files(&settings.directory_path);
            println!("[Main]::time elapsed is: {:?}", start.elapsed());

            for data in data_vec {
                match database::insert_game(&data, &mut db) {
                    Ok(()) => println!("[Main]::Game saved successfully:"),
                    Err(err) => println!("[Main]::Error while saving game: {:?}", err),
                }
            }

            let file_watcher_handler = file_watcher::file_watcher_thread(&settings.directory_path);

            let mut file_watcher = app_state.file_watcher_handler.lock().unwrap();

            *app_state.db.lock().unwrap() = Some(db);
            *file_watcher = Some(file_watcher_handler);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch_data, update_dir_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
