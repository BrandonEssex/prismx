use log::{debug, error, info, warn, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use env_logger::Env;

pub fn init_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Logger initialized successfully.");
}