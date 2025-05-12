// src/plugin_manifest.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub enabled: bool,
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub can_render: bool,
    pub can_modify_state: bool,
    pub can_access_files: bool,
}
