use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch_config_changes(path: &str) {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Config::default()).expect("Failed to create watcher");

    watcher
        .watch(path, RecursiveMode::NonRecursive)
        .expect("Failed to watch config");

    std::thread::spawn(move || {
        for event in rx {
            println!("Config change detected: {:?}", event);
        }
    });
}