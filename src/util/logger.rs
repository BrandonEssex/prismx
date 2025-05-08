use log::{LevelFilter};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: Option<String>,
}

pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn Error>> {
    let level = LevelFilter::from_str(&config.level).unwrap_or(LevelFilter::Info);
    let mut builder = env_logger::Builder::new();
    builder.filter_level(level);

    if let Some(file_path) = &config.file_output {
        let file = File::create(file_path)?;
        builder.target(env_logger::Target::Pipe(Box::new(file)));
    }

    builder.init();
    Ok(())
}

pub fn load_logging_config(path: &Path) -> Option<LoggingConfig> {
    let mut content = String::new();
    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut content).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}