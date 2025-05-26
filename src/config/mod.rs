use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::state::serialize::PersistedLayout;

pub mod theme;

pub const CONFIG_VERSION: u32 = 1;

pub mod theme;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PrismConfig {
    pub version: u32,
    pub layout: Option<PersistedLayout>,
}

/// Return path to `~/.config/prismx/prismx.toml`
fn config_file_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("prismx")
        .join("prismx.toml")
}

pub fn load_config() -> PrismConfig {
    let path = config_file_path();
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(cfg) = toml::from_str::<PrismConfig>(&data) {
            if cfg.version == CONFIG_VERSION {
                return cfg;
            }
        }
    }
    PrismConfig { version: CONFIG_VERSION, ..Default::default() }
}

pub fn save_config(cfg: &PrismConfig) {
    if let Ok(data) = toml::to_string(cfg) {
        let path = config_file_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, data);
    }
}

//
// PLUGIN REGISTRY (JSON) SUPPORT
//

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

/// Reads `config/plugin.json` and validates lock status
pub fn load_locked_registry() -> Result<PluginRegistry, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("config/plugin.json")?;
    let registry: PluginRegistry = serde_json::from_str(&data)?;
    if !registry.locked {
        return Err("Plugin registry must be locked".into());
    }
    Ok(registry)
}
