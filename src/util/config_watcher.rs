use crate::util::logger;
use crate::util::logger::LoggingConfig;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc::{channel, Receiver};
use log::{error, info};
use tokio::fs;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub logging: LoggingConfig,
}

pub async fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path).await?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub async fn watch_config_changes(path: &str) -> notify::Result<()> {
    let (tx, mut rx) = channel(1);

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let _ = tx.blocking_send(());
                }
            }
        },
        notify::Config::default(),
    )?;

    watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;

    info!("Configuration watcher initialized for '{}'.", path);

    while rx.recv().await.is_some() {
        match load_config(path).await {
            Ok(updated_config) => {
                info!("Configuration file '{}' reloaded.", path);
                if let Err(e) = logger::init_logging() {
                    error!("Failed to update logger settings: {}", e);
                } else {
                    info!(
                        "Logger settings updated: level '{}'",
                        updated_config.logging.log_level
                    );
                }
            }
            Err(e) => error!("Failed to reload config '{}': {}", path, e),
        }
    }

    Ok(())
}