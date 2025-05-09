use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub level: String,
    pub theme: String,
    pub zen_mode: ZenConfig,
    pub extension_host: ExtensionHostConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZenConfig {
    pub title_fade_delay_secs: u64,
    pub autosave_interval_secs: u64,
    pub scratchpad_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionHostConfig {
    pub plugin_dir: String,
    pub log_level: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}