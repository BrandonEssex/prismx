use crate::mindmap_state::MindmapState;

pub struct AppState {
    pub mindmap: MindmapState,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mindmap: MindmapState::new(),
            running: true,
        }
    }
}