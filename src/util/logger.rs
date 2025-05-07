use chrono::Local;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

pub fn log_zen(msg: &str) {
    let log_dir = "logs";
    let log_file = format!("{}/zen_debug.log", log_dir);
    let _ = create_dir_all(log_dir);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, msg);
    }
}