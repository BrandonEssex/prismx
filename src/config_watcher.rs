// src/config_watcher.rs

use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

pub fn watch_config<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2))?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Write(path)) => {
                    println!("Configuration changed: {:?}", path);
                    // Future: Reload configuration
                }
                Ok(_) => {}
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }
    });

    Ok(())
}