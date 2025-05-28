pub mod init;
pub mod level;
pub mod sinks;

pub use init::init_logger;

use chrono::Local;
use serde::Serialize;
use std::path::PathBuf;

pub fn write_snapshot<T: Serialize>(snapshot: &T) -> std::io::Result<PathBuf> {
    use std::fs;
    let _ = fs::create_dir_all("logs/snapshots");
    let filename = format!("{}.json", Local::now().format("%Y%m%d-%H%M"));
    let path = PathBuf::from("logs/snapshots").join(filename);
    let json = serde_json::to_string_pretty(snapshot).unwrap();
    fs::write(&path, json)?;
    Ok(path)
}

#[macro_export]
macro_rules! log_debug {
    ($state:expr, $($arg:tt)*) => {
        if $state.debug_input_mode {
            tracing::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*);
    };
}
