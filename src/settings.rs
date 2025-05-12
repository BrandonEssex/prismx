// src/settings.rs

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    pub autosave: bool,
    pub plugin_autoload: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            autosave: true,
            plugin_autoload: true,
        }
    }
}

pub fn load_settings<P: AsRef<Path>>(path: P) -> Settings {
    if let Ok(data) = fs::read_to_string(path) {
        toml::from_str(&data).unwrap_or_default()
    } else {
        Settings::default()
    }
}
