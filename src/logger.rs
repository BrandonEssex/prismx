use log::{LevelFilter, debug, error, info, warn};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fs;
use crate::util::logger::LoggingConfig;
use thiserror::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: Option<String>,
}

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("config.toml");
    let mut config_content = String::new();

    if config_path.exists() {
        let mut file = File::open(config_path)?;
        file.read_to_string(&mut config_content)?;

        let value: toml::Value = toml::from_str(&config_content)?;
        if let Ok(config) = value.try_into::<LoggingConfig>() {
            let level = match config.level.as_str() {
                "debug" => LevelFilter::Debug,
                "warn" => LevelFilter::Warn,
                "error" => LevelFilter::Error,
                _ => LevelFilter::Info,
            };

            let loggers: Vec<Box<dyn simplelog::SharedLogger>> = if let Some(path) = config.file_output {
                let file = File::create(path)?;
                vec![WriteLogger::new(level, Config::default(), file)]
            } else {
                vec![TermLogger::new(level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]
            };

            simplelog::CombinedLogger::init(loggers)?;
        }
    }

    Ok(())
}