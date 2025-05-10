use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub level: String,
    pub theme: String,
    pub zen_mode: ZenConfig,
    pub extension_host: ExtensionHostConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenConfig {
    pub title_fade_delay_secs: u64,
    pub autosave_interval_secs: u64,
    pub scratchpad_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionHostConfig {
    pub plugin_dir: String,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            theme: "default".to_string(),
            zen_mode: ZenConfig {
                title_fade_delay_secs: 2,
                autosave_interval_secs: 10,
                scratchpad_path: "~/.config/prismx/zen_scratchpad.md".to_string(),
            },
            extension_host: ExtensionHostConfig {
                plugin_dir: "extensions/".to_string(),
                log_level: "info".to_string(),
            },
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}