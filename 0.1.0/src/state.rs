use std::path::PathBuf;
use crate::inbox::InboxState;
use crate::storage::inbox_storage::{load_inbox_from_disk, save_inbox_to_disk};

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
        let _ = save_inbox_to_disk(&self.inbox_path, &self.inbox);
    }

    pub fn load_inbox(&mut self) {
        if let Ok(loaded) = load_inbox_from_disk(&self.inbox_path) {
            self.inbox = loaded;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(PathBuf::from("data"))
    }
}