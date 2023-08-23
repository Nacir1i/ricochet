use notify::{event::CreateKind, *};

use crate::{
    emit_tauri_event,
    file_reader::{read_file, Data},
    Payload, TauriEvent,
};

pub fn file_watcher_thread(path: &String) {
    let (sender, receiver) = std::sync::mpsc::channel();

    let initial_path = std::path::PathBuf::from(path);

    std::thread::spawn(move || {
        let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind()
            == WatcherKind::PollWatcher
        {
            let config = Config::default().with_poll_interval(std::time::Duration::from_secs(1));
            Box::new(PollWatcher::new(sender, config).unwrap())
        } else {
            Box::new(RecommendedWatcher::new(sender, Config::default()).unwrap())
        };

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

        for event in receiver {
            if let Ok(event) = event {
                if !is_valid_file(&event.paths.as_slice()[0]) {
                    return;
                }

                match event.kind {
                    EventKind::Create(CreateKind::Any) => {
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
                }
            }
        }
    });
}

fn is_valid_file(path: &std::path::PathBuf) -> bool {
    let metadata = std::fs::metadata(path).unwrap();

    if metadata.is_file() && path.extension().map(|s| s == "csv").unwrap_or(false) {
        true
    } else {
        false
    }
}
