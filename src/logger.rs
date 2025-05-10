// FINAL FULL FILE DELIVERY
// Filename: /src/logger.rs
// File Delivery Progress: 5/∞ FINAL FILES delivered

use crate::config::Config;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn init_logger(_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = "prismx.log";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let now = SystemTime::now();
    writeln!(file, "=== PrismX Log Start @ {:?} ===", now)?;
    Ok(())
}