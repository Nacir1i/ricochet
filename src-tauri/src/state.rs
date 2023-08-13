use std::{
    path::PathBuf,
    sync::{mpsc::Sender, Mutex},
};

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
    pub file_watcher_handler: Mutex<Option<Sender<PathBuf>>>,
}

pub trait ServiceAccess {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult;

    fn file_watcher_handler<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Sender<PathBuf>) -> TResult;
    fn file_watcher_handler_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Sender<PathBuf>) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_ref().unwrap();

        operation(db)
    }
    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
    }

    fn file_watcher_handler<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Sender<PathBuf>) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let file_watcher_handler_guard = app_state.file_watcher_handler.lock().unwrap();
        let file_watcher_handler = file_watcher_handler_guard.as_ref().unwrap();

        operation(file_watcher_handler)
    }
    fn file_watcher_handler_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Sender<PathBuf>) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut file_watcher_handler_guard = app_state.file_watcher_handler.lock().unwrap();
        let file_watcher_handler = file_watcher_handler_guard.as_mut().unwrap();

        operation(file_watcher_handler)
    }
}
