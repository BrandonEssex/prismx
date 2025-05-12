use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct PluginManifest {
    pub plugin: PluginInfo,
    pub ui: Option<PluginUI>,
    pub chain: Option<PluginChain>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub id: String,
    pub enabled: bool,
    pub entry: String,
    pub launcher_keywords: Option<Vec<String>>,
    pub node_action_trigger: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PluginUI {
    pub tabbed: Option<bool>,
    pub panel: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PluginChain {
    pub next: Option<Vec<String>>,
}

impl PluginManifest {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let manifest_data = fs::read_to_string(&path).ok()?;
        toml::from_str(&manifest_data).ok()
    }
}
