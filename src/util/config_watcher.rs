use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use std::thread;
use crate::util::logger::{init_logging, LoggingConfig};
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub logging: LoggingConfig,
}

pub fn watch_config_changes(path: &str) -> notify::Result<Receiver<()>> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.send(());
        }
    })?;

    watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;

    Ok(rx)
}

pub fn reload_config_on_change(path: &str) {
    if let Ok(rx) = watch_config_changes(path) {
        thread::spawn(move || {
            for _ in rx.iter() {
                if let Err(e) = init_logging() {
                    eprintln!("Failed to reload logger config: {}", e);
                }
            }
        });
    }
}