// src/plugin_manifest.rs

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub capabilities: Vec<String>,
}

pub fn load_manifest<P: AsRef<Path>>(path: P) -> Option<PluginManifest> {
    fs::read_to_string(path)
        .ok()
        .and_then(|data| toml::from_str(&data).ok())
}
