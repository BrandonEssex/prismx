use crate::inbox::InboxState;
use std::path::PathBuf;

pub struct AppState {
    pub inbox: InboxState,
    pub inbox_path: PathBuf,
}

impl Default for AppState {
    fn default() -> Self {
        let path = PathBuf::from("data/inbox.json");
        Self {
            inbox: InboxState::default(),
            inbox_path: path,
        }
    }
}