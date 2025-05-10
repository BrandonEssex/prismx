use crate::mindmap_state::MindmapState;
use crate::tag::TagEntry;

#[derive(Debug)]
pub struct ExportSummary {
    pub format: String,
    pub tags: Vec<String>,
    pub trust_summary: String,
}

#[derive(Debug)]
pub struct AppState {
    pub running: bool,
    pub mindmap: MindmapState,
    pub tag_glossary: Vec<TagEntry>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: true,
            mindmap: MindmapState::default(),
            tag_glossary: vec![],
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}