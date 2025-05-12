// src/state/app_state.rs

use crate::plugin::registry::PluginRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub view: View,
    pub sidebar: SidebarView,
    pub active_plugin: Option<String>,
    pub plugin_registry: PluginRegistry,
    pub focused_node: Option<Uuid>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
    Dashboard,
    Zen,
    Log,
    Mindmap,
    Export,
}

impl Default for View {
    fn default() -> Self {
        View::Dashboard
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarView {
    Hidden,
    Help,
    Triage,
    Scratchpad,
    Plugins,
    Config,
}

impl Default for SidebarView {
    fn default() -> Self {
        SidebarView::Hidden
    }
}