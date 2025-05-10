use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Option<String>,
    pub enable_plugins: bool,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new("config/prismx.toml");

    if path.exists() {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config {
            theme: Some("default".to_string()),
            enable_plugins: true,
        })
    }
}