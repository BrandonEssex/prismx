mod mode;
mod app;
mod config;
mod error_handling;
mod input;
mod logger;
mod screen;
mod state;

mod zen_mode;
mod scratchpad;
mod inbox;
mod view_triage;

mod spotlight;
mod tui;
mod api;
mod storage;
mod extension_host;
mod actions;
mod util;

use crate::app::run_app;
use crate::util::logger::{init_logging, LoggingConfig};
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let config_path = Path::new("config.toml");

    let config_data = fs::read_to_string(config_path)
        .expect("Failed to read config.toml for logging");

    let config: LoggingConfig = toml::from_str(&config_data)
        .expect("Failed to parse logging configuration");

    init_logging(&config).expect("Failed to initialize logging");

    run_app()
}