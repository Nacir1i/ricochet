use serde::{Deserialize, Serialize};
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub struct Data {
    pub tiles: Vec<TilesRecords>,
    pub key_value: Vec<KeyValueRecord>,
    pub stats: Stats,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
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
    pub kill: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub timestamp: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub bot: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub weapon: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub ttk: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub shots: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub accuracy: Option<f32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub damage_done: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub damage_taken: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub efficiency: Option<f32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub cheated: Option<bool>,
}

impl Hash for TilesRecords {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.kill.hash(hasher);
        self.timestamp.hash(hasher);
        self.bot.hash(hasher);
        self.weapon.hash(hasher);
        self.ttk.hash(hasher);
        self.shots.hash(hasher);
        self.damage_done.hash(hasher);
        self.damage_taken.hash(hasher);
        self.cheated.hash(hasher);

        match self.efficiency {
            Some(value) => {
                let bits = value.to_bits();
                bits.hash(hasher);
            }
            None => ().hash(hasher),
        }
        match self.accuracy {
            Some(value) => {
                let bits = value.to_bits();
                bits.hash(hasher);
            }
            None => ().hash(hasher),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct KeyValueRecord {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Stats {
    pub weapon: String,
    pub shots: u16,
    pub hits: u16,
    pub damage_done: f32,
    pub damage_possible: f32,
}

impl Hash for Stats {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.weapon.hash(hasher);
        self.shots.hash(hasher);
        self.hits.hash(hasher);

        let damage_done_bits = self.damage_done.to_bits();
        damage_done_bits.hash(hasher);

        let damage_possible_bits = self.damage_possible.to_bits();
        damage_possible_bits.hash(hasher);
    }
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

pub fn read_existing_files(path: &String) -> Vec<Data> {
    let mut data_vec = Vec::new();
    let dir_path = path.to_owned();
    let dir_entries = std::fs::read_dir(dir_path).unwrap();

    for file in dir_entries {
        match file {
            Ok(file_entry) => {
                if let Ok((tiles, key_value, stats)) = read_file(&file_entry.path()) {
                    let data = Data {
                        tiles,
                        key_value,
                        stats,
                    };

                    let data_size = size_of::<Data>()
                        + size_of::<TilesRecords>() * data.tiles.len()
                        + size_of::<KeyValueRecord>() * data.key_value.len()
                        + size_of::<Stats>()
                        + size_of::<Timestamp>() * data.tiles.len();

                    data_vec.push(data);

                    println!("Estimated size of Data struct: {} bytes", data_size);
                } else {
                    eprintln!("[File_reader]::Error reading file: {:?}", file_entry.path());
                }
            }
            Err(err) => {
                eprintln!("[File_reader]::Error reading directory entry: {}", err);
            }
        }
    }

    data_vec
}
