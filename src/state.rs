use crate::plugin::sandbox::replay::ReplayEngine;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportSummary {
    pub format: String,
    pub tags: Vec<String>,
    pub trust_summary: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    pub tag_glossary: Vec<TagEntry>,
    pub export: ExportSummary,
    pub replay_engine: ReplayEngine,
    running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tag_glossary: Vec::new(),
            export: ExportSummary::default(),
            replay_engine: ReplayEngine::new(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagEntry {
    pub name: String,
    pub role: String,
    pub source: String,
}