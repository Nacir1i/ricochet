use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Data {
    pub(crate) tiles: Vec<TilesRecords>,
    pub(crate) key_value: Vec<KeyValueRecord>,
    pub(crate) stats: Stats,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TilesRecords {
    #[serde(deserialize_with = "csv::invalid_option")]
    kill: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option")]
    timestamp: Option<Timestamp>,
    #[serde(deserialize_with = "csv::invalid_option")]
    bot: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    weapon: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ttk: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    shots: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option")]
    accuracy: Option<f32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    damage_done: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    damage_taken: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    efficiency: Option<f32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    cheated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyValueRecord {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Stats {
    weapon: String,
    shots: u16,
    hits: u16,
    damage_done: f32,
    damage_possible: f32,
}

pub fn read_file(
    path: &PathBuf,
) -> core::result::Result<(Vec<TilesRecords>, Vec<KeyValueRecord>, Stats), Box<dyn Error>> {
    let mut tile_record_list: Vec<TilesRecords> = Vec::new();
    let mut key_value_record_list: Vec<KeyValueRecord> = Vec::new();
    let mut stats_record_list: Stats = Stats::default();
    let validation_vec = vec![
        "Kills:",
        "Score:",
        "Hash:",
        "Deaths:",
        "Scenario:",
        "FOV:",
        "Horiz Sens:",
    ];

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .double_quote(false)
        .flexible(true)
        .from_path(path)?;

    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => return Err(err.into()),
        };

        if record.len() == 12 {
            let tile_record: TilesRecords = record.deserialize(None)?;
            tile_record_list.push(tile_record);
        } else if record.len() == 2 && validation_vec.contains(&&record[0]) {
            let key_value_record: KeyValueRecord = record.deserialize(None)?;
            key_value_record_list.push(key_value_record);
        } else if record.len() == 6 {
            let stats_record: Stats = record.deserialize(None)?;
            stats_record_list = stats_record;
        }
    }

    Ok((tile_record_list, key_value_record_list, stats_record_list))
}

pub fn read_existing_files(path: &String) {
    let dir_path = path.to_owned();
    let dir_entries = std::fs::read_dir(dir_path).unwrap();

    for file in dir_entries {
        match file {
            Ok(file_entry) => {
                if let Ok((tiles, key_value, stats)) = read_file(&file_entry.path()) {
                    let _data = Data {
                        tiles,
                        key_value,
                        stats,
                    };
                } else {
                    eprintln!("[File_reader]::Error reading file: {:?}", file_entry.path());
                }
            }
            Err(err) => {
                eprintln!("[File_reader]::Error reading directory entry: {}", err);
            }
        }
    }
}
