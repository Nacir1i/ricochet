use notify::{event::CreateKind, *};
use std::{ffi::OsStr, path::PathBuf, sync::mpsc::Sender};

use crate::{
    emit_tauri_event,
    file_reader::{read_file, Data},
};

pub fn file_watcher_thread(path: &String) -> Sender<PathBuf> {
    let initial_path = PathBuf::from(path);

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
                            emit_tauri_event(data, &"new_run");
                        }
                        Err(err) => {
                            eprintln!("[File_watcher]::Error reading file: {}", err);
                        }
                    }
                }
                _ => (),
            },
            Err(e) => println!("[File_watcher]::watch error: {:?}", e),
        })
        .unwrap();

    let (sender, receiver) = std::sync::mpsc::channel::<PathBuf>();

    println!("[File_watcher]::File watcher initiated");

    match watcher.watch(&initial_path, RecursiveMode::NonRecursive) {
        Ok(notify_watcher) => notify_watcher,
        Err(err) => eprintln!("[File_watcher]::Error : {}", err),
    };

    std::thread::spawn(move || loop {
        if let Ok(new_path) = receiver.recv() {
            println!("[File_watcher]::updating path to: {:?}", new_path);
            match watcher.watch(&initial_path, RecursiveMode::NonRecursive) {
                Ok(notify_watcher) => notify_watcher,
                Err(err) => eprintln!("[File_watcher]::Unwatch Error : {}", err),
            };
            match watcher.watch(&new_path, RecursiveMode::NonRecursive) {
                Ok(notify_watcher) => notify_watcher,
                Err(err) => eprintln!("[File_watcher]::New watch Error : {}", err),
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
