use notify::{event::CreateKind, *};
use std::{path::Path, time::Duration};
use tauri::Window;

use crate::{
    emit_tauri_event,
    file_reader::{read_file, Data},
    DIRECTORY_PATH,
};

pub fn file_watcher_thread(window: &Window) {
    let (tx, rx) = std::sync::mpsc::channel();
    let main_window = window.clone();

    std::thread::spawn(move || {
        let mut watcher: Box<dyn Watcher> =
            if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                let config = Config::default().with_poll_interval(Duration::from_secs(1));
                Box::new(PollWatcher::new(tx, config).unwrap())
            } else {
                Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
            };

        watcher
            .watch(Path::new(DIRECTORY_PATH), RecursiveMode::Recursive)
            .unwrap();

        for event in rx {
            match event {
                Ok(event) => match event.kind {
                    EventKind::Create(CreateKind::Any) => {
                        println!("{:?}", event.kind);

                        match read_file(&event.paths.as_slice()[0]) {
                            Ok((tiles, key_value, stats)) => {
                                let data = Data {
                                    tiles,
                                    key_value,
                                    stats,
                                };
                                emit_tauri_event(&main_window, data);
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
