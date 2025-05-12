use crate::mindmap_state::MindmapState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone)]
pub struct AppState {
    pub mindmap: MindmapState,
    pub focused_node: Option<Uuid>,
    pub sidebar_visible: bool,
    pub editing_node_id: Option<Uuid>,
    pub layout_profile: String,
    pub activity_log: Vec<ActivityEvent>,
    pub view: View,
    pub sidebar_view: SidebarView,
}

#[derive(Debug, Clone, Copy)]
pub enum View {
    Mindmap,
    Zen,
    Log,
    Export,
}

impl Default for View {
    fn default() -> Self {
        View::Mindmap
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SidebarView {
    Meta,
    Outline,
    Triage,
}

impl Default for SidebarView {
    fn default() -> Self {
        SidebarView::Meta
    }
}