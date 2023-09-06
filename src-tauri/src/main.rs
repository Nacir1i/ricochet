#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod file_reader;
mod file_watcher;
mod state;

use database::{GroupedPlaylist, InsertPlaylist, Scenario, ScenarioData, Settings};
use file_reader::Data;
use state::{AppState, ServiceAccess};
use std::env;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Manager, State, Window};

static WINDOW: OnceLock<Window> = OnceLock::new();

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

#[derive(Clone, serde::Serialize)]
pub struct Payload<T> {
    message: String,
    data: T,
}

pub enum TauriEvent {
    NewRun(Payload<Data>),
    Error(Payload<String>),
    Info(Payload<String>),
    Warning(Payload<String>),
}

fn emit_tauri_event(event: TauriEvent) {
    let window = WINDOW.get().expect("Window is not available");

    match event {
        TauriEvent::NewRun(payload) => window.emit("new_run", payload).unwrap(),
        TauriEvent::Error(payload) => window.emit("error", payload).unwrap(),
        TauriEvent::Info(payload) => window.emit("info", payload).unwrap(),
        TauriEvent::Warning(payload) => window.emit("warning", payload).unwrap(),
    };
}

#[tauri::command]
fn insert_game(data: Data, app_handle: AppHandle) {
    app_handle.db_mut(|db| match database::insert_game(&data, db) {
        Ok(()) => println!("[Main]::game saved"),
        Err(err) => eprintln!("[Main]::insert game Error : {:?}", err),
    })
}

#[tauri::command]
fn insert_playlist(data: InsertPlaylist, app_handle: AppHandle) {
    app_handle.db_mut(|db| match database::insert_playlist(data, db) {
        Ok(()) => println!("[Main]::playlist Saved"),
        Err(err) => eprintln!("[Main]::insert playlist Error : {:?}", err),
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
fn fetch_playlist_with_data(app_handle: AppHandle) -> Vec<GroupedPlaylist> {
    let mut vec: Vec<GroupedPlaylist> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_playlist_with_data(db, false) {
            Ok(fetched_games) => vec = fetched_games,
            Err(err) => eprintln!("[Main]::fetch playlist with data Error : {}", err),
        };
    });

    vec
}

#[tauri::command]
fn fetch_active_playlist_with_data(app_handle: AppHandle) -> Vec<GroupedPlaylist> {
    let mut vec: Vec<GroupedPlaylist> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_playlist_with_data(db, true) {
            Ok(fetched_games) => vec = fetched_games,
            Err(err) => eprintln!("[Main]::fetch playlist with data Error : {}", err),
        };
    });

    vec
}

#[tauri::command]
fn fetch_settings(app_handle: AppHandle) -> Settings {
    let mut settings: Option<Settings> = None;

    app_handle.db(|db| {
        match database::get_settings(db) {
            Ok(fetched_data) => settings = Some(fetched_data),
            Err(err) => eprintln!("[Main]::fetch settings Error : {}", err),
        };
    });

    settings.unwrap()
}

#[tauri::command]
fn fetch_scenario_data(app_handle: AppHandle) -> Vec<ScenarioData> {
    let mut data: Vec<ScenarioData> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_scenario_data(db) {
            Ok(fetched_data) => data = fetched_data,
            Err(err) => eprintln!("[Main]::fetch scenario data Error : {}", err),
        };
    });

    data
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
}

#[tauri::command]
fn update_playlist_state(playlist_id: u64, state: String, app_handle: AppHandle) {
    app_handle
        .db(|db| database::update_playlist_state(playlist_id, state, db))
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
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

            let settings = database::get_settings(&db).unwrap();

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
                    Ok(()) => {}
                    Err(err) => {
                        emit_tauri_event(TauriEvent::Error(Payload {
                            message: "Error while saving a game".to_owned(),
                            data: err.to_string(),
                        }));
                        println!("[Main]::Error while saving game: {:?}", err)
                    }
                }
            }
            println!(
                "[Main]::(running queries)time elapsed is: {:?}",
                start.elapsed()
            );

            file_watcher::file_watcher_thread(&settings.directory_path);

            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_game_page,
            update_dir_path,
            clear_database,
            insert_game,
            fetch_scenarios,
            fetch_scenarios_games,
            fetch_settings,
            insert_playlist,
            fetch_playlist_with_data,
            update_playlist_state,
            fetch_active_playlist_with_data,
            fetch_scenario_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
