use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportSummary {
    pub node_count: usize,
    pub export_time: String,
}