use crate::plugin::PluginStatus;
use crate::plugin::sandbox::replay::ReplayEngine;

#[derive(Debug, Clone)]
pub struct TagEntry {
    pub name: String,
    pub role: String,
    pub source: String,
}

#[derive(Debug, Clone)]
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