use crate::plugin::status::PluginStatus;
use crate::plugin::sandbox::replay::ReplayEngine;

#[derive(Debug, Clone, Default)]
pub struct ExportSummary {
    pub format: String,
    pub tags: Vec<String>,
    pub trust_summary: String,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub tag_glossary: Vec<TagEntry>,
    pub export: ExportSummary,
    pub plugins: Vec<PluginStatus>,
    pub replay_engine: ReplayEngine,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tag_glossary: Vec::new(),
            export: ExportSummary::default(),
            plugins: Vec::new(),
            replay_engine: ReplayEngine::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagEntry {
    pub name: String,
    pub role: String,
    pub source: String,
}