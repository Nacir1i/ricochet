use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

#[derive(Debug)]
pub struct Settings {
    pub directory_path: String,
}

const CURRENT_DB_VERSION: u32 = 4;

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");

    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("db.sqlite");

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    println!("[Database]::database initialized");

    Ok(db)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
            "
                DROP TABLE IF EXISTS game;
                DROP TABLE IF EXISTS scenario;
                DROP TABLE IF EXISTS state;
                DROP TABLE IF EXISTS key_value;
                DROP TABLE IF EXISTS tile;
                DROP TABLE IF EXISTS playlist;
                DROP TABLE IF EXISTS scenario_playlist;
                DROP TABLE IF EXISTS playlist_collection;
                DROP TABLE IF EXISTS playlist_playlist_collection;
                DROP TABLE IF EXISTS setting;

                CREATE TABLE game(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    scenario_id INTEGER,
                    name TEXT NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(scenario_id) REFERENCES scenario(id)
                );

                CREATE TABLE state(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    game_id INTEGER,
                    weapon TEXT NOT NULL,
                    shots INTEGER NOT NULL,
                    hits INTEGER NOT NULL,
                    damage_done REAL NOT NULL,
                    damage_possible REAL NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(game_id) REFERENCES game(id)
                );

                CREATE TABLE key_value(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    game_id INTEGER,
                    key TEXT DEFAULT '_',
                    value TEXT DEFAULT '_',
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(game_id) REFERENCES game(id)
                );

                CREATE TABLE tile(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    game_id INTEGER,
                    kill INTEGER,
                    timestamp TEXT,
                    bot TEXT,
                    weapon TEXT,
                    tkt TEXT,
                    shots INTEGER,
                    accuracy REAL,
                    damage_done INTEGER,
                    damage_possible INTEGER,
                    efficiency REAL,
                    cheated BOOLEAN NOT NULL CHECK(cheated IN (0, 1, '0', '1')),
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(game_id) REFERENCES game(id)
                ); 

                CREATE TABLE scenario (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    difficulty TEXT DEFAULT 'NONE' CHECK(difficulty IN ('EASY', 'MEDIUM', 'HARD')),
                    created_at date DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE scenario_playlist(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    scenario_id INTEGER NOT NULL,
                    playlist_id INTEGER NOT NULL,
                    reps INTEGER NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(scenario_id) REFERENCES scenario(id),
                    FOREIGN KEY(playlist_id) REFERENCES playlist(id)
                );

                CREATE TABLE playlist (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE playlist_playlist_collection(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    playlist_id INTEGER NOT NULL,
                    playlist_collection_id INTEGER NOT NULL,
                    reps INTEGER NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(playlist_id) REFERENCES playlist(id),
                    FOREIGN KEY(playlist_collection_id) REFERENCES playlist_collection(id)
                );

                CREATE TABLE playlist_collection (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE setting (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    directory_path TEXT NOT NULL
                );
            ",
        )?;

        tx.commit()?;
    }

    println!("[Database]::database updated");

    Ok(())
}

pub fn get_settings(db: &Connection) -> Result<Settings, rusqlite::Error> {
    let query = "SELECT * FROM setting";

    let result: Result<Settings, rusqlite::Error> = db.query_row(query, [], |row| {
        Ok(Settings {
            directory_path: row.get(1)?,
        })
    });

    match result {
        Ok(settings) => {
            println!("[Database]::Get settings : {:?}", settings);
            Ok(settings)
        }
        Err(err) => {
            println!("Error : {:?}", err);
            Err(err)
        }
    }
}

pub fn update_settings(settings: Settings, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement =
        db.prepare("UPDATE setting SET directory_path = @directory_path WHERE id = 1")?;

    statement.execute(named_params! { "@directory_path": settings.directory_path })?;

    println!("[Database]::Update settings : {:?}", settings);

    Ok(())
}
