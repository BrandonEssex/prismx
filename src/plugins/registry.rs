use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, path::Path, sync::Mutex};

use super::settings::{self, PluginSettingsTab};

#[derive(Debug, Deserialize, Clone)]
pub struct PluginModule {
    #[serde(default)]
    pub dock_icon: Option<String>,
    #[serde(default)]
    pub hover_title: Option<String>,
    pub route_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginInfo {
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    pub entry: String,
    pub version: String,
    #[serde(default)]
    pub modules: Vec<PluginModule>,
}

#[derive(Debug, Deserialize)]
struct ManifestFile {
    #[serde(default)]
    plugins: Vec<PluginInfo>,
}

static MANIFEST: OnceCell<Mutex<Vec<PluginInfo>>> = OnceCell::new();

pub fn load_manifest(path: &Path) -> Vec<PluginInfo> {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("[PLUGIN] failed to read manifest {}: {}", path.display(), err);
            return Vec::new();
        }
    };
    let parsed: ManifestFile = serde_json::from_str(&data).unwrap_or(ManifestFile { plugins: Vec::new() });
    let lock = MANIFEST.get_or_init(|| Mutex::new(Vec::new()));
    *lock.lock().unwrap() = parsed.plugins.clone();
    parsed.plugins
}

pub fn entries() -> Vec<PluginInfo> {
    MANIFEST.get().map(|m| m.lock().unwrap().clone()).unwrap_or_default()
}

#[derive(Debug, Clone)]
pub struct DockEntry {
    pub icon: String,
    pub title: String,
    pub route: String,
}

pub fn dock_entries() -> Vec<DockEntry> {
    let mut entries_out = Vec::new();
    for plugin in entries() {
        if plugin.modules.is_empty() {
            let icon = plugin
                .icon
                .clone()
                .unwrap_or_else(|| "🔌".to_string());
            entries_out.push(DockEntry {
                icon,
                title: plugin.name.clone(),
                route: format!("/{}", plugin.entry),
            });
        } else {
            for m in plugin.modules {
                let icon = m
                    .dock_icon
                    .or_else(|| plugin.icon.clone())
                    .unwrap_or_else(|| "🔌".to_string());
                let title = m.hover_title.unwrap_or_else(|| plugin.name.clone());
                entries_out.push(DockEntry {
                    icon,
                    title,
                    route: format!("/{}", m.route_id),
                });
            }
        }
    }
    entries_out
}

pub fn module_entries() -> Vec<(String, String)> {
    let mut mods = Vec::new();
    for plugin in entries() {
        if plugin.modules.is_empty() {
            mods.push((plugin.icon.clone().unwrap_or_else(|| "🔌".to_string()), plugin.name.clone()));
        } else {
            for m in plugin.modules {
                let icon = m
                    .dock_icon
                    .or_else(|| plugin.icon.clone())
                    .unwrap_or_else(|| "🔌".to_string());
                let title = m.hover_title.unwrap_or_else(|| plugin.name.clone());
                mods.push((icon, title));
            }
        }
    }
    mods
}

/// Retrieve all plugin-defined settings tabs.
pub fn plugin_tabs() -> Vec<PluginSettingsTab> {
    settings::drain_tabs()
}
