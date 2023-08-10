use notify::{event::CreateKind, *};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{
    emit_tauri_event,
    file_reader::{read_file, Data},
    DIRECTORY_PATH,
};

pub fn file_watcher_thread() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let binding = "/home/linuxlolrandomxd/Desktop/scenarios".to_owned();
        let dir_path = DIRECTORY_PATH.get().unwrap_or(&binding);

        let mut watcher: Box<dyn Watcher> =
            if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                let config = Config::default().with_poll_interval(Duration::from_secs(1));
                Box::new(PollWatcher::new(tx, config).unwrap())
            } else {
                Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
            };

        watcher
            .watch(Path::new(dir_path), RecursiveMode::Recursive)
            .unwrap();

        for event in rx {
            match event {
                Ok(event) => match event.kind {
                    notify::event::EventKind::Create(CreateKind::File) => {
                        if !is_csv_file(&event.paths.as_slice()[0]) {
                            continue;
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
                                eprintln!("Error reading file: {}", err);
                            }
                        }
                    }
                    _ => (),
                },
                Err(err) => {
                    eprintln!("Error receiving event: {}", err);
                }
            }
        }
    });
}

fn is_csv_file(file_name: &PathBuf) -> bool {
    let file_extension = Path::new(file_name).extension().and_then(OsStr::to_str);

    if file_name.is_file() && file_extension == Some("csv") {
        return true;
    }
    return false;
}
