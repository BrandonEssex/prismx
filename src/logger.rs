use log::{LevelFilter};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: Option<String>,
}

pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn Error>> {
    let level = LevelFilter::from_str(&config.level).unwrap_or(LevelFilter::Info);

    if let Some(ref log_file) = config.file_output {
        let file = File::create(Path::new(log_file))?;
        simplelog::WriteLogger::init(level, simplelog::Config::default(), file)?;
    } else {
        simplelog::TermLogger::init(
            level,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        )?;
    }

    Ok(())
}