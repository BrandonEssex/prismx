pub mod export_summary;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum View {
    #[default]
    Mindmap,
    Help,
    Inbox,
    Dashboard,
    HelpOverlay,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum SidebarView {
    #[default]
    None,
    Meta,
    Outline,
    Tags,
}

#[derive(Default)]
pub struct AppState {
    pub view: View,
    pub sidebar_view: SidebarView,
    pub show_help: bool,
    pub in_command_mode: bool,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub command_index: Option<usize>,
    pub layout_profile: String,
    pub breadcrumbs: Vec<uuid::Uuid>,
    pub focused_node: Option<uuid::Uuid>,
    pub sidebar_visible: bool,
    pub active_sidebar_tab: usize,
    pub editing_node_id: Option<uuid::Uuid>,
    pub cursor_position: usize,
    pub activity_log: Vec<ActivityEvent>,
    pub bookmarked_node_ids: std::collections::HashSet<uuid::Uuid>,
    pub mindmap: crate::mindmap_state::MindmapState,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityEvent {
    pub node_id: uuid::Uuid,
    pub timestamp: String,
    pub event_type: ActivityType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ActivityType {
    Created,
    Edited,
    Focused,
    Commented,
    Bookmarked,
}