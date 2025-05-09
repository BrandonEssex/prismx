use simplelog::*;
use std::fs::{create_dir_all, OpenOptions};

pub fn init_logger() {
    let _ = create_dir_all("logs");

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/qa_runtime.log")
        .expect("Failed to create log file");

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), file),
    ])
    .expect("Failed to initialize logger");

    log::info!("Logger initialized.");
}