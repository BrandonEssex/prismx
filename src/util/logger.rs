// FINAL VERSION — File Delivery Progress: 1/1  
// File: src/util/logger.rs

use log::{info};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: Option<String>,
}

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("config.toml");

    if config_path.exists() {
        let mut config_content = String::new();
        File::open(config_path)?.read_to_string(&mut config_content)?;

        if let Ok(config): Result<LoggingConfig, _> = toml::from_str(&config_content) {
            let level = LevelFilter::from_str(&config.level).unwrap_or(LevelFilter::Info);

            if let Some(log_file) = config.file_output {
                let file = File::create(log_file)?;
                WriteLogger::init(level, Config::default(), file)?;
            } else {
                TermLogger::init(level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)?;
            }
            return Ok(());
        }
    }

    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)?;
    Ok(())
}