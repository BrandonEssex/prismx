// FINAL FULL FILE DELIVERY
// Filename: /src/state.rs
// File Delivery Progress: 2/2 FINAL FILES delivered

use crate::mindmap_state::MindmapState;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportSummary {
    pub node_count: usize,
    pub export_time: String,
}

#[derive(Default)]
pub struct AppState {
    pub mindmap: MindmapState,
    pub focused_node: Option<Uuid>,
    pub bookmarked_node_ids: HashSet<Uuid>,
    pub breadcrumbs: Vec<Uuid>,
    pub sidebar_visible: bool,
    pub active_sidebar_tab: usize,
    pub editing_node_id: Option<Uuid>,
    pub cursor_position: usize,
    pub layout_profile: String,
    pub activity_log: Vec<ActivityEvent>,
    pub show_prism_panel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub node_id: Uuid,
    pub timestamp: String,
    pub event_type: ActivityType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    Created,
    Edited,
    Focused,
    Commented,
    Bookmarked,
}