use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use chrono::Utc;

pub fn start_journal() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ZEN] Journaling active. Type below:");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let log_dir = Path::new("logs");
    fs::create_dir_all(log_dir)?;
    let filename = format!("logs/journal_{}.txt", Utc::now().format("%Y%m%d_%H%M%S"));
    let mut file = OpenOptions::new().create(true).write(true).open(&filename)?;
    file.write_all(buffer.as_bytes())?;

    println!("[ZEN] Saved to {}", filename);
    Ok(())
}
