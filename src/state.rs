use crate::mindmap_state::MindmapState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum View {
    #[default]
    Mindmap,
    Help,
    Inbox,
    Dashboard,
    HelpOverlay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SidebarView {
    #[default]
    None,
    Meta,
    Outline,
    Tags,
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
    pub view: View,
    pub sidebar_view: SidebarView,
    pub show_help: bool,
    pub in_command_mode: bool,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub command_index: Option<usize>,
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