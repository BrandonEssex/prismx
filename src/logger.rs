use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use simplelog::*;

pub fn init_logger() {
    let _ = create_dir_all("logs");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/qa_runtime.log")
        .unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), file),
    ])
    .unwrap();

    log::info!("Logger initialized");
}