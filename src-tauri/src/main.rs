#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod file_reader;
mod file_watcher;
mod state;

use database::Scenario;
use file_reader::Data;
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

// TODO: future common error solution
// pub type CommonResult<T> = std::result::Result<T, CommonError>;

// #[derive(Debug)]
// pub enum CommonError {
//     RusqliteError(rusqlite::Error),
//     CoreError(Box<dyn std::error::Error>),
// }

// impl serde::Serialize for CommonError {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             CommonError::RusqliteError(rusqlite_error) => {
//                 serializer.serialize_str(rusqlite_error.to_string().as_ref())
//             }
//             CommonError::CoreError(core_error) => {
//                 serializer.serialize_str(core_error.to_string().as_ref())
//             }
//         }
//     }
// }

// impl Display for CommonError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CommonError::RusqliteError(rusqlite_error) => write!(f, "{}", rusqlite_error),
//             CommonError::CoreError(core_error) => write!(f, "{}", core_error),
//         }
//     }
// }

fn emit_tauri_event(data: file_reader::Data, event: &str) {
    let window = WINDOW.get().expect("Window is not available");

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
fn insert_game(data: Data, app_handle: AppHandle) {
    app_handle.db_mut(|db| match database::insert_game(&data, db) {
        Ok(()) => println!("[Main]::game saved"),
        Err(err) => eprintln!("[Main]::insert game Error : {:?}", err),
    })
}

#[tauri::command]
fn fetch_game_page(page: u8, limit: u8, app_handle: AppHandle) -> Vec<Data> {
    let mut data: Vec<Data> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_game_page(page, limit, db) {
            Ok(fetched_data) => data = fetched_data,
            Err(err) => eprintln!("[Main]::fetch data Error : {}", err),
        };
    });

    data
}

#[tauri::command]
fn fetch_scenarios(app_handle: AppHandle) -> Vec<Scenario> {
    let mut vec: Vec<Scenario> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_scenarios(db) {
            Ok(fetched_scenarios) => vec = fetched_scenarios,
            Err(err) => eprintln!("[Main]::fetch scenarios Error : {}", err),
        };
    });

    vec
}

#[tauri::command]
fn fetch_scenarios_games(scenario_id: u64, app_handle: AppHandle) -> Vec<Data> {
    let mut vec: Vec<Data> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_scenarios_games(scenario_id, db) {
            Ok(fetched_games) => vec = fetched_games,
            Err(err) => eprintln!("[Main]::fetch scenarios Error : {}", err),
        };
    });

    vec
}

#[tauri::command]
fn clear_database(app_handle: AppHandle) {
    match app_handle.db_mut(|mut db| database::clear_database(&mut db)) {
        Ok(()) => println!("[Main]::clear_database was successful"),
        Err(err) => eprintln!("[Main]::clear_database Error : {:?}", err),
    }
}

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
                directory_path: "C:/Program Files (x86)/Steam/SteamApps/common/FPSAimTrainer/FPSAimTrainer/Saved/SaveGames/scenarios".to_owned(),
            });

            let start = std::time::Instant::now();
            let data_vec = file_reader::read_existing_files(&settings.directory_path);
            println!(
                "[Main]::(reading files: {})time elapsed is: {:?}",
                data_vec.len(),
                start.elapsed()
            );

            let start = std::time::Instant::now();
            for data in data_vec {
                match database::insert_game(&data, &mut db) {
                    Ok(()) => println!("[Main]::Game saved successfully:"),
                    Err(err) => println!("[Main]::Error while saving game: {:?}", err),
                }
            }
            println!(
                "[Main]::(running queries)time elapsed is: {:?}",
                start.elapsed()
            );

            let file_watcher_handler = file_watcher::file_watcher_thread(&settings.directory_path);

            let mut file_watcher = app_state.file_watcher_handler.lock().unwrap();

            *app_state.db.lock().unwrap() = Some(db);
            *file_watcher = Some(file_watcher_handler);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_game_page,
            update_dir_path,
            clear_database,
            insert_game,
            fetch_scenarios,
            fetch_scenarios_games
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
