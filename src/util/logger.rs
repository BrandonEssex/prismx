use log::info;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use env_logger::Env;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: Option<String>,
}

pub fn init_logging() {
    if let Some(config_path) = get_config_path() {
        if let Ok(mut file) = File::open(&config_path) {
            let mut config_content = String::new();
            if file.read_to_string(&mut config_content).is_ok() {
                if let Ok(config): Result<LoggingConfig, _> = toml::from_str(&config_content) {
                    let env = Env::default().default_filter_or(config.level);
                    env_logger::Builder::from_env(env).init();
                    if let Some(log_file) = config.file_output {
                        info!("Logging to file: {}", log_file);
                    }
                    return;
                }
            }
        }
    }

    let env = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();
}

fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("prismx");
        path.push("logging.toml");
        path
    })
}