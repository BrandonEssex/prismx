use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PrismXConfig {
    pub theme: Option<String>,
    pub default_module: Option<String>,
    pub log_level: Option<String>,
}

pub fn load_config() -> PrismXConfig {
    fs::read_to_string("config.toml")
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}
