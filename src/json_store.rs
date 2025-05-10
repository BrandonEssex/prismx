use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};

pub fn load_json<T: DeserializeOwned>(path: &str) -> Option<T> {
    if Path::new(path).exists() {
        let mut file = File::open(path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        serde_json::from_str(&contents).ok()
    } else {
        None
    }
}

pub fn save_json<T: Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    file.write_all(json.as_bytes())
}