// src/config_watcher.rs

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

pub fn watch_config<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    std::thread::spawn(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        println!("Configuration modified: {:?}", event.paths);
                        // Future: reload or re-apply config
                    }
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        }
    });

    Ok(())
}