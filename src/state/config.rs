use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub version: u32,
    pub ui: Ui,
    pub theme: Theme,
    pub behavior: Behavior,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ui {
    pub show_logs: bool,
    pub dock_layout: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    pub dark_mode: bool,
    pub accent_color: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Behavior {
    pub auto_arrange: bool,
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(include_str!("../../config/default.toml")).expect("valid default config")
    }
}

fn config_file_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("prismx")
        .join("config.toml")
}

impl Config {
    pub fn load() -> Self {
        let path = config_file_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str::<Config>(&data).or_else(|_| serde_json::from_str(&data)) {
                if cfg.version == 1 {
                    return cfg;
                }
            }
        }
        Config::default()
    }
}
