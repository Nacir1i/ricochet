use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

const CURRENT_DB_VERSION: u32 = 2;

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
            ",
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO items (title) VALUES (@title)")?;
    statement.execute(named_params! { "@title": title })?;

    Ok(())
}

pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let title: String = row.get("title")?;

        items.push(title);
    }

    Ok(items)
}
