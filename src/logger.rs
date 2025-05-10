use crate::config::Config;
use chrono::Local;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
};

pub fn init_logger(_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all("logs")?;
    let log_path = "logs/qa_runtime.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let now = Local::now();
    writeln!(file, "{} [INFO] Logger initialized.", now.format("%Y-%m-%d %H:%M:%S"))?;

    Ok(())
}