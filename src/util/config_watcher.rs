// FINAL VERSION — File Delivery Progress: 3/6
// File: src/util/config_watcher.rs

use crate::util::logger::LoggingConfig;
use serde::Deserialize;
use std::fs::read_to_string;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::task;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub logging: LoggingConfig,
}

pub async fn watch_config_changes<P: AsRef<Path>>(path: P) -> Receiver<()> {
    let (tx, rx) = channel(1);
    let path_buf = path.as_ref().to_owned();

    task::spawn(async move {
        loop {
            if let Ok(content) = read_to_string(&path_buf) {
                if let Ok(updated_config): Result<Config, _> = toml::from_str(&content) {
                    let _ = tx.send(()).await;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });

    rx
}