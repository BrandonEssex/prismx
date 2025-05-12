// src/plugin/registry.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::plugin_manifest::{PluginManifest, PluginInfo};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginRegistry {
    pub registered_plugins: HashMap<String, PluginInfo>,
}

impl PluginRegistry {
    pub fn all(&self) -> Vec<&String> {
        self.registered_plugins.keys().collect()
    }
}
