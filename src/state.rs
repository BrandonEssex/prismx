use std::path::PathBuf;
use crate::inbox::{InboxState, Priority};
use crate::storage::inbox_storage::{load_inbox_from_disk, save_inbox_to_disk};

#[derive(Debug)]
pub struct AppState {
    pub inbox: InboxState,
    pub inbox_path: PathBuf,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let inbox_path = data_dir.join("inbox.json");
        let inbox = load_inbox_from_disk(&inbox_path).unwrap_or_default();
        AppState {
            inbox,
            inbox_path,
        }
    }

    pub fn save_inbox(&self) {
        if let Err(e) = save_inbox_to_disk(&self.inbox_path, &self.inbox) {
            eprintln!("Failed to save inbox state: {:?}", e);
        }
    }

    pub fn load_inbox(&mut self) {
        if let Ok(loaded) = load_inbox_from_disk(&self.inbox_path) {
            self.inbox = loaded;
        }
    }
}