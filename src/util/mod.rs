// src/util/mod.rs

use std::fs;
use std::path::Path;

pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn timestamp() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}
