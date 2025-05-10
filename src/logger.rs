use std::{fs::OpenOptions, fs};
use simplelog::{WriteLogger, Config as LogConfig, LevelFilter};
use crate::config::Config;

pub fn init_logger(config: &Config) {
    let _ = fs::create_dir_all("logs");

    let log_path = "logs/runtime.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");

    WriteLogger::init(LevelFilter::Info, LogConfig::default(), file)
        .expect("Failed to initialize file logger");
}