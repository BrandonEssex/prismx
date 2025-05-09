use std::fs::{create_dir_all, OpenOptions};
use simplelog::*;

pub fn init_logger() {
    let log_dir = "logs";
    let log_file = "logs/qa_runtime.log";

    let _ = create_dir_all(log_dir);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), file),
    ])
    .unwrap();

    log::info!("Logger initialized");
}