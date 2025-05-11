use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSummary {
    pub node_count: usize,
    pub export_time: String,
}