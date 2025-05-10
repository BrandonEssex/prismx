// FINAL FULL FILE DELIVERY
// Filename: /src/logger.rs

use std::{fs::OpenOptions, io::Write};
use simplelog::{WriteLogger, Config as LogConfig, LevelFilter};
use crate::config::Config;
use std::fs;

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