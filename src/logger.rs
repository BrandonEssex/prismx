// src/logger.rs

use std::fs::OpenOptions;
use std::io::Write;
use crate::util::{ensure_directory_exists, timestamp};

pub fn init_logger(log_dir: &str) -> std::io::Result<()> {
    ensure_directory_exists(log_dir)?;

    let log_file = format!("{}/runtime_{}.log", log_dir, timestamp());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)?;

    writeln!(file, "Logger initialized at {}", timestamp())?;
    Ok(())
}
