use crate::config::Config;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn init_logger(_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = "logs/prismx.log";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let now = SystemTime::now();
    writeln!(file, "=== PrismX Session Start @ {:?} ===", now)?;
    Ok(())
}