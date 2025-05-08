use crate::inbox::InboxState;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppState {
    pub inbox: InboxState,
    pub inbox_path: PathBuf,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let inbox_path = data_dir.join("inbox.json");
        let inbox = InboxState::default();
        AppState {
            inbox,
            inbox_path,
        }
    }

    pub fn save_inbox(&self) {
        // placeholder
    }

    pub fn load_inbox(&mut self) {
        // placeholder
    }
}