use log::LevelFilter;
use simplelog::*;
use std::fs::{self, File};
use std::path::Path;
use toml;
use env_logger::Env;

#[derive(Debug)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub log_level: String,
    pub log_to_file: bool,
    pub log_file_path: String,
}

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("config.toml")?;
    let config: toml::Value = toml::from_str(&config_content)?;
    let logging_cfg: LoggingConfig = config
        .get("logging")
        .ok_or("Missing [logging] section")?
        .clone()
        .try_into()?;

    if !logging_cfg.enabled {
        return Ok(());
    }

    let level = match logging_cfg.log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    if logging_cfg.log_to_file {
        let path = Path::new(&logging_cfg.log_file_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        loggers.push(WriteLogger::new(
            level,
            Config::default(),
            File::create(path)?,
        ));
    }

    CombinedLogger::init(loggers)?;

    Ok(())
}

pub fn log_zen(msg: &str) {
    println!("[ZenLog] {}", msg);
}