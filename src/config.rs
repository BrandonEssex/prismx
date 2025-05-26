use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PluginRegistry {
    pub locked: bool,
    pub plugins: Vec<PluginEntry>,
}

#[derive(Deserialize, Debug)]
pub struct PluginEntry {
    pub id: String,
    pub version: String,
    pub trusted: bool,
    pub trust_chain: Vec<String>,
}

pub fn load_locked_registry() -> Result<PluginRegistry, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("config/plugin.json")?;
    let registry: PluginRegistry = serde_json::from_str(&data)?;
    if !registry.locked {
        return Err("Plugin registry must be locked".into());
    }
    Ok(registry)
}

pub mod theme;
