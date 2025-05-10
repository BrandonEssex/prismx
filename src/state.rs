use crate::mindmap_state::MindmapState;
use crate::storage::inbox_storage::InboxState;
use crate::tag::TagEntry;
use crate::config::Config;
use crate::scratchpad::Scratchpad;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportSummary {
    pub format: String,
    pub tags: Vec<String>,
    pub trust_summary: String,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub config: Config,
    pub scratchpad: Scratchpad,
    pub inbox: InboxState,
    pub mindmap: MindmapState,
    pub tag_glossary: Vec<TagEntry>,
    pub export: ExportSummary,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            scratchpad: Scratchpad::default(),
            inbox: InboxState::default(),
            mindmap: MindmapState::new(),
            tag_glossary: vec![],
            export: ExportSummary::default(),
            running: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}