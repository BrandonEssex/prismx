// FINAL VERSION — File Delivery Progress: 5/6  
// File: src/util/logger.rs

use log::{LevelFilter};
use simplelog::{ColorChoice, Config as SimpleLogConfig, TermLogger, TerminalMode};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use std::{fs, io};

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

        let config: LoggingConfig = toml::from_str(&config_content)?;

        let level = match config.level.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        };

        if let Some(log_file) = config.file_output {
            let file = File::create(log_file)?;
            simplelog::WriteLogger::init(level, SimpleLogConfig::default(), file)?;
        } else {
            TermLogger::init(level, SimpleLogConfig::default(), TerminalMode::Mixed, ColorChoice::Auto)?;
        }
    } else {
        TermLogger::init(LevelFilter::Info, SimpleLogConfig::default(), TerminalMode::Mixed, ColorChoice::Auto)?;
    }

    Ok(())
}