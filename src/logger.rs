// FINAL VERSION — File Delivery Progress: 2/6
// File: src/logger.rs

use log::{LevelFilter, Record};
use simplelog::{Config as SimpleLogConfig, TermLogger, TerminalMode, ColorChoice, WriteLogger};
use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::util::logger::LoggingConfig;
use std::error::Error;
use std::env;

pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn Error>> {
    let log_level = match config.level.as_str() {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let simplelog_config = SimpleLogConfig::default();

    if let Some(path) = &config.file_output {
        let file = File::create(Path::new(path))?;
        WriteLogger::init(log_level, simplelog_config, file)?;
    } else {
        TermLogger::init(log_level, simplelog_config, TerminalMode::Mixed, ColorChoice::Auto)?;
    }

    Ok(())
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: Option<String>,
}