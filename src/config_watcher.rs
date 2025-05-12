// src/config_watcher.rs

use std::path::Path;
use notify::{RecursiveMode, RecommendedWatcher, Result, Watcher, Event, EventKind};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

pub fn watch_config<P: AsRef<Path>>(path: P) -> Result<()> {
    let (tx, rx): (Sender<Result<Event>>, Receiver<Result<Event>>) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event>| {
            match res {
                Ok(event) => {
                    if let EventKind::Modify(_) = event.kind {
                        println!("Configuration changed: {:?}", event.paths);
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        },
        notify::Config::default(),
    )?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(10));
    });

    Ok(())
}