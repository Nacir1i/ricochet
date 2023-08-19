use notify::{event::CreateKind, *};
use std::{ffi::OsStr, path::PathBuf, sync::mpsc::Sender};

use crate::{
    emit_tauri_event,
    file_reader::{read_file, Data},
    Payload, TauriEvent,
};

pub fn file_watcher_thread(path: &String) -> Sender<PathBuf> {
    let initial_path = PathBuf::from(path);

    println!("{}", initial_path.display());

    let mut watcher =
        notify::recommended_watcher(|res: std::result::Result<Event, Error>| match res {
            Ok(event) => match event.kind {
                notify::event::EventKind::Create(CreateKind::File) => {
                    if !is_csv_file(&event.paths.as_slice()[0]) {
                        return;
                    };
                    match read_file(&event.paths.as_slice()[0]) {
                        Ok((tiles, key_value, stats)) => {
                            let data = Data {
                                tiles,
                                key_value,
                                stats,
                            };
                            emit_tauri_event(TauriEvent::NewRun(Payload {
                                message: "New run was found".to_owned(),
                                data,
                            }));
                        }
                        Err(err) => {
                            emit_tauri_event(TauriEvent::Error(Payload {
                                message: "Error while parsing the file".to_owned(),
                                data: err.to_string(),
                            }));
                            eprintln!("[File_watcher]::Error reading file: {}", err);
                        }
                    }
                }
                _ => (),
            },
            Err(err) => {
                emit_tauri_event(TauriEvent::Error(Payload {
                    message: "Error while catching event".to_owned(),
                    data: err.to_string(),
                }));
                println!("[File_watcher]::watch error: {:?}", err)
            }
        })
        .unwrap();

    let (sender, receiver) = std::sync::mpsc::channel::<PathBuf>();

    match watcher.watch(&initial_path, RecursiveMode::NonRecursive) {
        Ok(notify_watcher) => {
            emit_tauri_event(crate::TauriEvent::Info(Payload {
                message: "".to_owned(),
                data: "File watcher initiated successfully".to_owned(),
            }));

            println!("[File_watcher]::File watcher initiated");
            notify_watcher
        }
        Err(err) => {
            emit_tauri_event(TauriEvent::Error(Payload {
                message: "Error while starting file watcher".to_owned(),
                data: err.to_string(),
            }));
            eprintln!("[File_watcher]::Error : {}", err)
        }
    };

    std::thread::spawn(move || loop {
        if let Ok(new_path) = receiver.recv() {
            println!("[File_watcher]::updating path to: {:?}", new_path);
            match watcher.watch(&initial_path, RecursiveMode::NonRecursive) {
                Ok(notify_watcher) => {
                    emit_tauri_event(crate::TauriEvent::Info(Payload {
                        message: "".to_owned(),
                        data: "File watcher stopped successfully".to_owned(),
                    }));
                    notify_watcher
                }
                Err(err) => {
                    emit_tauri_event(TauriEvent::Error(Payload {
                        message: "Error while stopping file watcher".to_owned(),
                        data: err.to_string(),
                    }));
                    eprintln!("[File_watcher]::Error : {}", err)
                }
            };
            match watcher.watch(&new_path, RecursiveMode::NonRecursive) {
                Ok(notify_watcher) => {
                    emit_tauri_event(crate::TauriEvent::Info(Payload {
                        message: "".to_owned(),
                        data: "File watcher re-initiated successfully".to_owned(),
                    }));
                    notify_watcher
                }
                Err(err) => {
                    emit_tauri_event(TauriEvent::Error(Payload {
                        message: "Error while starting file watcher".to_owned(),
                        data: err.to_string(),
                    }));
                    eprintln!("[File_watcher]::Error : {}", err)
                }
            };
        }
    });

    sender
}

fn is_csv_file(file_name: &PathBuf) -> bool {
    let file_extension = file_name.extension().and_then(OsStr::to_str);

    if file_name.is_file() && file_extension == Some("csv") {
        return true;
    }
    return false;
}
