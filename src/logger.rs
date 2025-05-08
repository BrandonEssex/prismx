use log::{info};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub file: Option<String>,
    pub level: Option<String>,
}

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("config.toml");

    if path.exists() {
        let mut file = File::open(path)?;
        let mut config_content = String::new();
        file.read_to_string(&mut config_content)?;

        let config: LoggingConfig = toml::from_str(&config_content)?;
        if let Some(log_file) = &config.file {
            info!("Logging to file: {}", log_file);
        }
    }

    Ok(())
}