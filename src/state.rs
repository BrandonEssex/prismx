use crate::mindmap_state::MindmapState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportSummary {
    pub node_count: usize,
    pub export_time: String,
}

#[derive(Default)]
pub struct AppState {
    pub mindmap: MindmapState,
}