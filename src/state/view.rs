use super::core::AppState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZenViewMode {
    Journal,
    Classic,
    Split,
    Summary,
}

impl Default for ZenViewMode {
    fn default() -> Self { Self::Journal }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriageViewMode {
    Feed,
    Actions,
}

impl Default for TriageViewMode {
    fn default() -> Self { Self::Feed }
}

/// Visibility state for optional overlays such as debugging HUDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OverlayState {
    Hidden,
    Visible,
}

impl Default for OverlayState {
    fn default() -> Self { Self::Hidden }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PluginViewMode {
    Registry,
}

impl Default for PluginViewMode {
    fn default() -> Self { Self::Registry }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PluginTagFilter {
    All,
    Trusted,
    Debug,
}

impl Default for PluginTagFilter {
    fn default() -> Self { Self::All }
}

impl PluginTagFilter {
    pub fn next(self) -> Self {
        match self {
            PluginTagFilter::All => PluginTagFilter::Trusted,
            PluginTagFilter::Trusted => PluginTagFilter::Debug,
            PluginTagFilter::Debug => PluginTagFilter::All,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct DebugSnapshot {
    pub active_module: String,
    pub auto_arrange: bool,
    pub scroll_x: i16,
    pub scroll_y: i16,
    pub zoom_level: f32,
    pub selected_node: Option<crate::node::NodeID>,
    pub viewport_width: u16,
    pub viewport_height: u16,
    pub plugins_active: usize,
}

impl DebugSnapshot {
    pub fn from_state(state: &AppState) -> Self {
        let (viewport_width, viewport_height) = crossterm::terminal::size().unwrap_or((0, 0));
        Self {
            active_module: state.mode.clone(),
            auto_arrange: state.auto_arrange,
            scroll_x: state.scroll_x,
            scroll_y: state.scroll_y,
            zoom_level: state.zoom_scale,
            selected_node: state.selected,
            viewport_width,
            viewport_height,
            plugins_active: state.plugin_host.active.len(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogViewState {
    Hidden,
    Visible,
}

impl Default for LogViewState {
    fn default() -> Self { Self::Hidden }
}

#[derive(Clone, Debug)]
pub struct TriageSummary {
    pub now: usize,
    pub triton: usize,
    pub done: usize,
    pub last_action: Option<String>,
    pub highlight_frames: u8,
}

impl Default for TriageSummary {
    fn default() -> Self {
        Self {
            now: 0,
            triton: 0,
            done: 0,
            last_action: None,
            highlight_frames: 0,
        }
    }
}
