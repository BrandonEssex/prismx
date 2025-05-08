use crate::util::logger::LoggingConfig;
use log::info;
use std::{fs, path::Path};

pub fn watch_config_changes(config_path: &Path) -> Option<LoggingConfig> {
    if let Ok(content) = fs::read_to_string(config_path) {
        let parsed: Result<LoggingConfig, _> = toml::from_str(&content);
        if let Ok(config) = parsed {
            info!("Loaded config from file.");
            return Some(config);
        } else {
            eprintln!("Failed to parse config file.");
        }
    } else {
        eprintln!("Failed to read config file.");
    }

    None
}