// FINAL FULL FILE DELIVERY
// Filename: /src/plugin.rs
// File Delivery Progress: 11/∞ FINAL FILES delivered

use std::collections::HashMap;

pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub capabilities: PluginCapabilities,
}

#[derive(Default)]
pub struct PluginCapabilities {
    pub read_state: bool,
    pub write_state: bool,
    pub inject_ui: bool,
    pub register_shortcuts: bool,
    pub access_filesystem: bool,
}

pub fn load_plugins() -> HashMap<String, PluginManifest> {
    // Stub: would scan /plugins and load plugin.toml files
    HashMap::new()
}