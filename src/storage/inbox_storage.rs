use crate::inbox::InboxState;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use chrono::Utc;

pub fn load_inbox_from_disk<P: AsRef<Path>>(path: P) -> Result<InboxState, String> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(InboxState::default());
    }

    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    match serde_json::from_str(&contents) {
        Ok(state) => Ok(state),
        Err(e) => {
            let backup_path = make_backup(path)?;
            eprintln!(
                "[ERROR] Failed to parse inbox.json. Created backup at {:?}. Error: {}",
                backup_path, e
            );
            Ok(InboxState::default())
        }
    }
}

pub fn save_inbox_to_disk<P: AsRef<Path>>(path: P, state: &InboxState) -> Result<(), String> {
    let serialized =
        serde_json::to_string_pretty(state).map_err(|e| format!("Failed to serialize: {}", e))?;

    let mut file = File::create(path.as_ref())
        .map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(serialized.as_bytes())
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

fn make_backup(original_path: &Path) -> Result<PathBuf, String> {
    let timestamp = Utc::now().format("bak.%Y%m%dT%H%M%S").to_string();
    let backup_path = original_path.with_extension(timestamp);
    fs::copy(original_path, &backup_path)
        .map_err(|e| format!("Failed to create backup: {}", e))?;
    Ok(backup_path)
}