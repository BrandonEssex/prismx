use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeProfile {
    pub name: String,
    pub colors: HashMap<String, String>,
}

impl ThemeProfile {
    pub fn load_from_file(path: &str) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn get_color(&self, key: &str) -> Option<String> {
        self.colors.get(key).cloned()
    }
}