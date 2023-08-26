use rand::Rng;
use rusqlite::{named_params, params, Connection, Error, Transaction};
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::Hash;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use tauri::AppHandle;

use crate::file_reader::{Data, KeyValueRecord, Stats, TilesRecords};
use crate::{emit_tauri_event, Payload};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub directory_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scenario {
    pub id: u64,
    pub name: String,
    pub difficulty: String,
    pub created_at: String,
    pub games_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: u64,
    pub name: String,
    pub hash: i64,
    pub scenario_id: u64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyScenarioData {
    pub date: String,
    pub avg_accuracy: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioChartStats {
    pub name: String,
    pub data: Vec<DailyScenarioData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioGeneralStats {
    pub name: String,
    pub games_count: u32,
    pub shots: Option<f32>,
    pub hits: Option<f32>,
    pub accuracy: Option<f32>,
    pub damage_done: Option<f32>,
    pub damage_possible: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioPlaylist {
    pub scenario_id: u64,
    pub reps: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertPlaylist {
    pub name: String,
    pub description: String,
    pub duration: u8,
    pub scenarios: Vec<ScenarioPlaylist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchedPlaylistData {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub duration: u8,
    pub scenario_name: String,
    pub scenario_difficulty: String,
    pub games_played: u8,
    pub reps: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupedPlaylist {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub duration: u8,
    pub state: String,
    pub scenarios: Vec<GroupedPlaylistScenario>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupedPlaylistScenario {
    pub scenario_name: String,
    pub scenario_difficulty: String,
    pub reps: u8,
    pub days: Vec<GroupedPlaylistGamesByDay>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GroupedPlaylistGamesByDay {
    pub games_count: u8,
}

const CURRENT_DB_VERSION: u32 = 1;

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
                PRAGMA foreign_keys = ON;

                DROP TABLE IF EXISTS game;
                DROP TABLE IF EXISTS scenario;
                DROP TABLE IF EXISTS state;
                DROP TABLE IF EXISTS stats;
                DROP TABLE IF EXISTS key_value;
                DROP TABLE IF EXISTS tile;
                DROP TABLE IF EXISTS playlist;
                DROP TABLE IF EXISTS scenario_playlist;
                DROP TABLE IF EXISTS playlist_collection;
                DROP TABLE IF EXISTS playlist_playlist_collection;
                DROP TABLE IF EXISTS setting;

                CREATE TABLE game(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    hash INTEGER NOT NULL,
                    scenario_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(scenario_id) REFERENCES scenario(id) ON DELETE CASCADE
                );

                CREATE TABLE stats(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    game_id INTEGER NOT NULL,
                    weapon TEXT NOT NULL,
                    shots INTEGER NOT NULL,
                    hits INTEGER NOT NULL,
                    damage_done REAL NOT NULL,
                    damage_possible REAL NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(game_id) REFERENCES game(id) ON DELETE CASCADE
                );

                CREATE TABLE key_value(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    game_id INTEGER NOT NULL,
                    key TEXT DEFAULT '_',
                    value TEXT DEFAULT '_',
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(game_id) REFERENCES game(id) ON DELETE CASCADE
                );

                CREATE TABLE tile(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    game_id INTEGER NOT NULL,
                    kill INTEGER,
                    timestamp TEXT,
                    bot TEXT,
                    weapon TEXT,
                    ttk TEXT,
                    shots INTEGER,
                    accuracy REAL,
                    damage_done INTEGER,
                    damage_possible INTEGER,
                    efficiency REAL,
                    cheated BOOLEAN NOT NULL CHECK(cheated IN (0, 1, '0', '1')),
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(game_id) REFERENCES game(id) ON DELETE CASCADE
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
                    FOREIGN KEY(scenario_id) REFERENCES scenario(id) ON DELETE CASCADE,
                    FOREIGN KEY(playlist_id) REFERENCES playlist(id) ON DELETE CASCADE
                );

                CREATE TABLE playlist (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    duration INTEGER NOT NULL,
                    started_at date DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+20 years')),
                    ended_at date DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+22 years')),
                    state TEXT DEFAULT 'INACTIVE' CHECK(state in ('ACTIVE', 'INACTIVE')),
                    created_at date DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE playlist_playlist_collection(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    playlist_id INTEGER NOT NULL,
                    playlist_collection_id INTEGER NOT NULL,
                    reps INTEGER NOT NULL,
                    created_at date DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(playlist_id) REFERENCES playlist(id) ON DELETE CASCADE,
                    FOREIGN KEY(playlist_collection_id) REFERENCES playlist_collection(id) ON DELETE CASCADE
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

                INSERT INTO setting (directory_path) values ('C:/Program Files (x86)/Steam/SteamApps/common/FPSAimTrainer/FPSAimTrainer/Saved/SaveGames/scenarios');

                CREATE TRIGGER update_playlist_state 
                AFTER UPDATE OF state ON playlist
                BEGIN
                    UPDATE playlist
                    SET 
                        started_at = CASE WHEN new.state = 'ACTIVE' THEN CURRENT_DATE ELSE started_at END,
                        ended_at = CASE WHEN new.state = 'ACTIVE' THEN DATE(CURRENT_TIMESTAMP, '+22 years') ELSE CURRENT_DATE END
                    WHERE id = new.id;
                END;
            ",
        )?;

        seed_database(&tx)?;

        tx.commit()?;

        println!("[Database]::database updated");
    }

    Ok(())
}

pub fn get_settings(db: &Connection) -> Result<Settings, rusqlite::Error> {
    let query = "SELECT * FROM setting WHERE id = 1";

    let result = db.query_row(query, [], |row| {
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
            println!("[Database]::(get_settings)Error : {:?}", err);
            Err(err)
        }
    }
}

pub fn update_settings(settings: Settings, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement =
        db.prepare("INSERT INTO setting (id, directory_path) VALUES (1, @directory_path) ON CONFLICT(id) DO UPDATE SET directory_path = @directory_path")?;

    statement.execute(named_params! { "@directory_path": settings.directory_path })?;

    println!("[Database]::Update settings : {:?}", settings);

    emit_tauri_event(crate::TauriEvent::Warning(Payload {
        message: "Settings updated".to_owned(),
        data: "App restart is needed".to_owned(),
    }));

    Ok(())
}

fn game_exists(hash: i64, db: &Connection) -> Result<bool, rusqlite::Error> {
    let query = "SELECT * FROM game WHERE hash = ?";

    let result = db.query_row(query, [hash], |row| {
        Ok(Game {
            id: row.get(0)?,
            hash: row.get(1)?,
            scenario_id: row.get(2)?,
            name: row.get(3)?,
            created_at: row.get(4)?,
        })
    });

    match result {
        Ok(_game) => Ok(true),
        Err(_err) => Ok(false),
    }
}

fn scenario_exists(name: &String, db: &Connection) -> Result<Option<Scenario>, rusqlite::Error> {
    let query = "SELECT * FROM scenario WHERE name = ?";
    let result: Result<Scenario, Error> = db.query_row(query, &[name], |row| {
        Ok(Scenario {
            id: row.get(0)?,
            name: row.get(1)?,
            difficulty: row.get(2)?,
            created_at: row.get(3)?,
            games_count: 0,
        })
    });

    match result {
        Ok(game) => Ok(Some(game)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

fn insert_game_data(data: &Data, game_id: i64, transaction: &Transaction) -> Result<(), Error> {
    let query = "INSERT INTO stats (
                            weapon,
                            shots,
                            hits,
                            damage_done,
                            damage_possible,
                            game_id
                        ) VALUES (?, ?, ?, ?, ?, ?)";
    transaction.prepare(query)?.execute(params![
        data.stats.weapon,
        data.stats.shots,
        data.stats.hits,
        data.stats.damage_done,
        data.stats.damage_possible,
        game_id,
    ])?;

    let query = "INSERT INTO tile (
                            kill,
                            timestamp,
                            bot,
                            weapon,
                            ttk,
                            shots,
                            accuracy,
                            damage_done,
                            damage_possible,
                            efficiency,
                            cheated,
                            game_id
                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    for tile in &data.tiles {
        transaction.prepare(query)?.execute(params![
            tile.kill,
            tile.timestamp,
            tile.bot,
            tile.weapon,
            tile.ttk,
            tile.shots,
            tile.accuracy,
            tile.damage_done,
            tile.damage_taken,
            tile.efficiency,
            0,
            game_id,
        ])?;
    }

    let query = "INSERT INTO key_value (key, value, game_id) VALUES (?, ?, ?)";
    for key_value in &data.key_value {
        transaction
            .prepare(query)?
            .execute(params![&key_value.key, &key_value.value, game_id])?;
    }

    Ok(())
}

pub fn insert_game(data: &Data, db: &mut Connection) -> Result<(), rusqlite::Error> {
    let hash = calculate_hash(&data);

    match game_exists(hash, db) {
        Ok(exists) => {
            if !exists {
                let key_value_vec = data
                    .key_value
                    .iter()
                    .filter(|key_value| key_value.key == "Scenario:")
                    .cloned()
                    .collect::<Vec<KeyValueRecord>>();

                let scenario_name = &key_value_vec[0].value;

                match scenario_exists(scenario_name, db) {
                    Ok(Some(scenario)) => {
                        let transaction = db.transaction()?;

                        let query =
                            "INSERT INTO game (name, hash, scenario_id) VALUES ('name', ?, ?)";
                        transaction
                            .prepare(query)?
                            .execute(params![hash, scenario.id])?;
                        let game_id = transaction.last_insert_rowid();

                        let _inserted_result = insert_game_data(data, game_id, &transaction)?;

                        transaction.commit()?;
                    }
                    Ok(None) => {
                        let transaction = db.transaction()?;

                        let query = "INSERT INTO scenario (name, difficulty) VALUES (?, 'EASY')";
                        transaction.prepare(query)?.execute([scenario_name])?;
                        let scenario_id = transaction.last_insert_rowid();

                        let query =
                            "INSERT INTO game (name, hash, scenario_id) VALUES ('name', ?, ?)";
                        transaction
                            .prepare(query)?
                            .execute(params![hash, scenario_id])?;
                        let game_id = transaction.last_insert_rowid();

                        let _inserted_result = insert_game_data(data, game_id, &transaction)?;

                        transaction.commit()?;
                    }
                    Err(err) => eprintln!("[Database]::(scenario exists check)Error: {}", err),
                }
            }
        }
        Err(err) => eprintln!("[Database]::(game exists check)Error: {}", err),
    }

    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> i64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() as i64
}

pub fn clear_database(db: &mut Connection) -> Result<(), rusqlite::Error> {
    let query = "
        DELETE FROM key_value;
        DELETE FROM tile;
        DELETE FROM stats;
        DELETE FROM game;
        DELETE FROM scenario;
    ";

    let tx = db.transaction()?;

    let _ = tx.execute_batch(query);
    let _ = tx.commit();

    emit_tauri_event(crate::TauriEvent::Info(Payload {
        message: "".to_owned(),
        data: "Database cleared successfully".to_owned(),
    }));

    Ok(())
}

pub fn fetch_key_value(
    game_id: u64,
    db: &Connection,
) -> Result<Vec<KeyValueRecord>, rusqlite::Error> {
    let mut vec: Vec<KeyValueRecord> = Vec::new();

    let query = "SELECT * FROM key_value WHERE game_id = ?";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([game_id])?;

    while let Some(row) = rows.next()? {
        let key_value = KeyValueRecord {
            key: row.get(2)?,
            value: row.get(3)?,
        };

        vec.push(key_value);
    }

    Ok(vec)
}

pub fn fetch_tiles(game_id: u64, db: &Connection) -> Result<Vec<TilesRecords>, rusqlite::Error> {
    let mut vec: Vec<TilesRecords> = Vec::new();

    let query = "SELECT * FROM tile WHERE game_id = ?";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([game_id])?;

    while let Some(row) = rows.next()? {
        let tile = TilesRecords {
            kill: row.get(2)?,
            timestamp: row.get(3)?,
            bot: row.get(4)?,
            weapon: row.get(5)?,
            ttk: row.get(6)?,
            shots: row.get(7)?,
            accuracy: row.get(8)?,
            damage_done: row.get(9)?,
            damage_taken: row.get(10)?,
            efficiency: row.get(11)?,
            cheated: row.get(12)?,
        };

        vec.push(tile);
    }

    Ok(vec)
}

pub fn fetch_stats(game_id: u64, db: &Connection) -> Result<Stats, rusqlite::Error> {
    let query = "SELECT * FROM stats WHERE game_id = ?";
    let result = db.query_row(query, [game_id], |row| {
        Ok(Stats {
            weapon: row.get(2)?,
            shots: row.get(3)?,
            hits: row.get(4)?,
            damage_done: row.get(5)?,
            damage_possible: row.get(6)?,
        })
    });

    result
}

pub fn fetch_game_page(page: u8, limit: u8, db: &Connection) -> Result<Vec<Data>, rusqlite::Error> {
    let mut vec: Vec<Data> = Vec::new();

    let query = "SELECT * FROM game LIMIT ? OFFSET ?";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([limit, (page - 1) * limit])?;

    while let Some(row) = rows.next()? {
        let game = Game {
            id: row.get(0)?,
            hash: row.get(1)?,
            scenario_id: row.get(2)?,
            name: row.get(3)?,
            created_at: row.get(4)?,
        };

        let key_value = fetch_key_value(game.id, db)?;
        let tiles = fetch_tiles(game.id, db)?;
        let stats = fetch_stats(game.id, db)?;

        let data = Data {
            key_value,
            tiles,
            stats,
        };

        vec.push(data)
    }

    Ok(vec)
}

pub fn fetch_gamed_with_scenario_id(
    game_id: u64,
    db: &Connection,
) -> Result<Vec<Data>, rusqlite::Error> {
    let mut vec: Vec<Data> = Vec::new();

    let query = "SELECT * FROM game WHERE id = ?";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([game_id])?;

    while let Some(row) = rows.next()? {
        let game = Game {
            id: row.get(0)?,
            hash: row.get(1)?,
            scenario_id: row.get(2)?,
            name: row.get(3)?,
            created_at: row.get(4)?,
        };

        let key_value = fetch_key_value(game.id, db)?;
        let tiles = fetch_tiles(game.id, db)?;
        let stats = fetch_stats(game.id, db)?;

        let data = Data {
            key_value,
            tiles,
            stats,
        };

        vec.push(data)
    }

    Ok(vec)
}

pub fn fetch_scenarios(db: &Connection) -> Result<Vec<Scenario>, rusqlite::Error> {
    let mut vec: Vec<Scenario> = Vec::new();

    let query = "SELECT s.*, COUNT(g.id) AS game_count FROM scenario AS s LEFT JOIN game AS g ON s.id = g.scenario_id GROUP BY s.id, s.name, s.difficulty ORDER BY s.id;";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let scenario = Scenario {
            id: row.get(0)?,
            name: row.get(1)?,
            difficulty: row.get(2)?,
            created_at: row.get(3)?,
            games_count: row.get(4)?,
        };

        vec.push(scenario)
    }

    Ok(vec)
}

pub fn fetch_scenarios_games(
    scenario_id: u64,
    db: &Connection,
) -> Result<Vec<Data>, rusqlite::Error> {
    let query = "SELECT * FROM scenario WHERE id = ?";

    let result = db.query_row(query, [scenario_id], |row| {
        Ok(Scenario {
            id: row.get(0)?,
            name: row.get(1)?,
            difficulty: row.get(2)?,
            created_at: row.get(3)?,
            games_count: 0,
        })
    });

    match result {
        Ok(scenario) => {
            let games = fetch_gamed_with_scenario_id(scenario.id, db)?;

            Ok(games)
        }
        Err(err) => Err(err),
    }
}

pub fn fetch_general_scenario_stats(
    db: &Connection,
) -> Result<Vec<ScenarioGeneralStats>, rusqlite::Error> {
    let mut vec: Vec<ScenarioGeneralStats> = Vec::new();

    let query = "
        SELECT s.name,
            COUNT(g.id),
            AVG(st.shots),
            AVG(st.hits),
            AVG(st.hits) / AVG(st.shots) * 100  AS accuracy,
            AVG(st.damage_done),
            AVG(st.damage_possible) 
        FROM scenario s 
        LEFT JOIN game g 
        ON s.id = g.scenario_id 
        LEFT JOIN stats st 
        ON g.id = st.game_id 
        GROUP BY s.id;
    ";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let scenario = ScenarioGeneralStats {
            name: row.get(0)?,
            games_count: row.get(1)?,
            shots: row.get(2)?,
            hits: row.get(3)?,
            accuracy: row.get(4)?,
            damage_done: row.get(5)?,
            damage_possible: row.get(6)?,
        };

        vec.push(scenario)
    }

    Ok(vec)
}

pub fn fetch_chart_scenario_stats(
    db: &Connection,
) -> Result<Vec<ScenarioChartStats>, rusqlite::Error> {
    let mut vec: Vec<ScenarioChartStats> = Vec::new();

    let query = "
        SELECT s.name,
            DATE(g.created_at),
            AVG(st.hits) / AVG(st.shots) * 100 AS accuracy 
        FROM scenario s 
        LEFT JOIN game g 
        ON s.id = g.scenario_id 
        LEFT JOIN stats st 
        ON g.id = st.game_id 
        GROUP BY s.id, 
            DATE(g.created_at);";
    let mut statement = db.prepare(query)?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<f64>>(2)?,
        ))
    })?;

    for result in rows {
        let (name, date, avg_accuracy) = result?;
        let daily_data = DailyScenarioData { date, avg_accuracy };

        let existing_index = vec.iter().position(|stats| stats.name == name);

        match existing_index {
            Some(index) => {
                vec[index].data.push(daily_data);
            }
            None => {
                let new_stats = ScenarioChartStats {
                    name: name.clone(),
                    data: vec![daily_data],
                };
                vec.push(new_stats);
            }
        }
    }

    Ok(vec)
}

pub fn insert_playlist(
    playlist_data: InsertPlaylist,
    db: &mut Connection,
) -> Result<(), rusqlite::Error> {
    let transaction = db.transaction()?;

    let query = "INSERT INTO playlist (name, description, duration) VALUES (?, ?, ?);";
    transaction.prepare(query)?.execute(params![
        playlist_data.name,
        playlist_data.description,
        playlist_data.duration
    ])?;
    let playlist_id = transaction.last_insert_rowid();

    let query = "INSERT INTO scenario_playlist (scenario_id, playlist_id, reps) VALUES (?, ?, ?)";
    for scenario_playlist in playlist_data.scenarios {
        transaction.prepare(query)?.execute(params![
            scenario_playlist.scenario_id,
            playlist_id,
            scenario_playlist.reps
        ])?;
    }

    transaction.commit()?;

    emit_tauri_event(crate::TauriEvent::Info(Payload {
        message: "Insert playlist".to_owned(),
        data: "Playlist was saved successfully".to_owned(),
    }));

    Ok(())
}

pub fn fetch_playlist_with_data(db: &Connection) -> Result<Vec<GroupedPlaylist>, rusqlite::Error> {
    let query = "
        SELECT p.id,
            p.name,
            p.description,
            p.duration,
            p.state,
            s.name,
            s.difficulty,
            count(g.id),
            sp.reps 
        FROM playlist p 
        LEFT JOIN scenario_playlist sp 
        ON p.id = sp.playlist_id 
        LEFT JOIN scenario s 
        ON s.id = sp.scenario_id 
        LEFT JOIN game g 
        ON g.scenario_id = s.id 
        AND g.created_at 
            BETWEEN p.started_at 
            AND CASE 
                    WHEN p.state = 'ACTIVE' 
                    THEN date(p.started_at, '+' || p.duration || ' days') 
                    ELSE p.ended_at 
                END 
        GROUP BY p.id, s.id, date(g.created_at)
        ORDER BY p.id, sp.id ,date(g.created_at);";
    let mut statement = db.prepare(query)?;
    let mut rows = statement.query([])?;

    let mut playlist_data_vec: Vec<GroupedPlaylist> = Vec::new();

    while let Some(row) = rows.next()? {
        let playlist_id = row.get(0)?;
        let scenario_name: String = row.get(5)?;
        let scenario_difficulty: String = row.get(6)?;
        let games_played = row.get(7)?;
        let reps = row.get(8)?;

        let games_data = GroupedPlaylistGamesByDay {
            games_count: games_played,
        };

        let scenario_data = GroupedPlaylistScenario {
            scenario_name: scenario_name.clone(),
            scenario_difficulty: scenario_difficulty.clone(),
            reps,
            days: vec![games_data],
        };

        if let Some(existing_playlist) = playlist_data_vec.iter_mut().find(|p| p.id == playlist_id)
        {
            if let Some(existing_scenario) = existing_playlist
                .scenarios
                .iter_mut()
                .find(|s| s.scenario_name == scenario_name)
            {
                existing_scenario.days.push(games_data);
            } else {
                existing_playlist.scenarios.push(scenario_data);
            }
        } else {
            let playlist_data = GroupedPlaylist {
                id: playlist_id,
                name: row.get(1)?,
                description: row.get(2)?,
                duration: row.get(3)?,
                state: row.get(4)?,
                scenarios: vec![scenario_data],
            };
            playlist_data_vec.push(playlist_data);
        }
    }

    Ok(playlist_data_vec)
}

fn seed_database(db: &Transaction) -> Result<(), rusqlite::Error> {
    let mut rng = rand::thread_rng();

    for playlist_num in 1..=10 {
        db.execute(
            "INSERT INTO playlist (name, description, duration, state, started_at) VALUES (?, ?, ?, ?, ?)",
            params![
                format!("Playlist {}", playlist_num),
                "Description",
                60,
                "ACTIVE",
                "2023-07-26 02:52:28",
            ],
        )?;
        let playlist_id = db.last_insert_rowid() as i32;

        let scenarios_count = rng.gen_range(1..=10);

        for scenario_num in 1..=scenarios_count {
            db.execute(
                "INSERT INTO scenario (name, difficulty) VALUES (?, ?)",
                params![
                    format!("Scenario {}-{}", playlist_num, scenario_num),
                    "EASY".to_owned()
                ],
            )?;
            let scenario_id = db.last_insert_rowid() as i32;

            db.execute(
                "INSERT INTO scenario_playlist (scenario_id, playlist_id, reps) VALUES (?, ?, ?)",
                params![scenario_id, playlist_id, 5],
            )?;

            for day in 1..=9 {
                let games_count = rng.gen_range(1..=9);

                for game_num in 0..=games_count {
                    let game_name = format!("Game {}", game_num);
                    let game_id = db.last_insert_rowid() as i32;

                    let shots = rng.gen_range(50..=200);
                    let hits = rng.gen_range(20..=shots);
                    let damage_done = rng.gen_range(50.0..=300.0);
                    let damage_possible = rng.gen_range(damage_done..=400.0);

                    db.execute(
                        "INSERT INTO game (hash, scenario_id, name, created_at) VALUES (?, ?, ?, ?)",
                        params![0, scenario_id, &game_name, format!("2023-08-0{} 02:52:28", day)],
                    )?;

                    db.execute(
                        "INSERT INTO stats (game_id, weapon, shots, hits, damage_done, damage_possible) VALUES (?, ?, ?, ?, ?, ?)",
                        params![game_id, "Weapon", shots, hits, damage_done, damage_possible],
                    )?;

                    for _ in 1..=7 {
                        db.execute(
                            "INSERT INTO key_value (game_id, key, value) VALUES (?, ?, ?)",
                            params![game_id, "Key", "Value"],
                        )?;
                    }

                    db.execute(
                    "INSERT INTO tile (game_id, kill, timestamp, bot, weapon, ttk, shots, accuracy, damage_done, damage_possible, efficiency, cheated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    params![game_id, Some(1), "2023-08-26", "Bot", "Weapon", "0.5s", 10, 0.85, 100, 120, 0.75, false],
                )?;
                }
            }
        }
    }

    Ok(())
}
