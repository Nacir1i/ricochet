#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod file_reader;
mod file_watcher;
mod state;

use database::{
    GroupedPlaylist, InsertPlaylist, Scenario, ScenarioChartStats, ScenarioGeneralStats, Settings,
};
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
        Ok(()) => (),
        Err(_err) => (),
    })
}

#[tauri::command]
fn insert_playlist(data: InsertPlaylist, app_handle: AppHandle) {
    app_handle.db_mut(|db| match database::insert_playlist(data, db) {
        Ok(()) => (),
        Err(_err) => (),
    })
}

#[tauri::command]
fn fetch_game_page(page: u8, limit: u8, app_handle: AppHandle) -> Vec<Data> {
    let mut data: Vec<Data> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_game_page(page, limit, db) {
            Ok(fetched_data) => data = fetched_data,
            Err(_err) => (),
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
            Err(_err) => (),
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
            Err(_err) => (),
        };
    });

    vec
}

#[tauri::command]
fn fetch_playlist_with_data(app_handle: AppHandle) -> Vec<GroupedPlaylist> {
    let mut vec: Vec<GroupedPlaylist> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_playlist_with_data(db) {
            Ok(fetched_games) => vec = fetched_games,
            Err(_err) => (),
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
            Err(_err) => (),
        };
    });

    settings.unwrap()
}

#[tauri::command]
fn fetch_general_scenario_stats(app_handle: AppHandle) -> Vec<ScenarioGeneralStats> {
    let mut data: Vec<ScenarioGeneralStats> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_general_scenario_stats(db) {
            Ok(fetched_data) => data = fetched_data,
            Err(_err) => (),
        };
    });

    data
}

#[tauri::command]
fn fetch_chart_scenario_stats(app_handle: AppHandle) -> Vec<ScenarioChartStats> {
    let mut data: Vec<ScenarioChartStats> = Vec::new();

    app_handle.db(|db| {
        match database::fetch_chart_scenario_stats(db) {
            Ok(fetched_data) => data = fetched_data,
            Err(_err) => (),
        };
    });

    data
}

#[tauri::command]
fn clear_database(app_handle: AppHandle) {
    match app_handle.db_mut(|mut db| database::clear_database(&mut db)) {
        Ok(()) => (),
        Err(_err) => (),
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

            let data_vec = file_reader::read_existing_files(&settings.directory_path);

            for data in data_vec {
                match database::insert_game(&data, &mut db) {
                    Ok(()) => {}
                    Err(err) => {
                        emit_tauri_event(TauriEvent::Error(Payload {
                            message: "Error while saving a game".to_owned(),
                            data: err.to_string(),
                        }));
                    }
                }
            }

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
            fetch_chart_scenario_stats,
            fetch_general_scenario_stats,
            insert_playlist,
            fetch_playlist_with_data,
            update_playlist_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
