// src/audit.rs

use std::fs::OpenOptions;
use std::io::Write;
use crate::util::{ensure_directory_exists, timestamp};

pub fn record_audit_event(log_dir: &str, message: &str) -> std::io::Result<()> {
    ensure_directory_exists(log_dir)?;
    let filename = format!("{}/session_{}.log", log_dir, timestamp());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;
    writeln!(file, "[{}] {}", timestamp(), message)?;
    Ok(())
}