use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, path::Path, sync::Mutex};

#[derive(Debug, Deserialize, Clone)]
pub struct PluginInfo {
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    pub entry: String,
    pub version: String,
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

pub fn dock_entries() -> Vec<(String, String)> {
    entries()
        .into_iter()
        .map(|e| {
            let icon = e.icon.unwrap_or_else(|| "🔌".to_string());
            (icon, format!("/{}", e.entry))
        })
        .collect()
}

pub fn module_entries() -> Vec<(String, String)> {
    entries()
        .into_iter()
        .map(|e| (e.icon.unwrap_or_else(|| "🔌".to_string()), e.name))
        .collect()
}
