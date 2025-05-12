use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::plugins::manifest::{PluginManifest, PluginInfo};

#[derive(Debug, Default)]
pub struct PluginRegistry {
    pub plugins: HashMap<String, PluginManifest>,
}

impl PluginRegistry {
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Self {
        let mut registry = PluginRegistry::default();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path().join("manifest.toml");
                if let Some(manifest) = PluginManifest::from_path(path) {
                    registry.plugins.insert(manifest.plugin.id.clone(), manifest);
                }
            }
        }
        registry
    }

    pub fn get_launcher_matches(&self, query: &str) -> Vec<&PluginInfo> {
        self.plugins
            .values()
            .filter_map(|p| {
                p.plugin
                    .launcher_keywords
                    .as_ref()
                    .and_then(|keys| {
                        if keys.iter().any(|k| k.contains(query)) {
                            Some(&p.plugin)
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    pub fn is_enabled(&self, id: &str) -> bool {
        self.plugins
            .get(id)
            .map(|p| p.plugin.enabled)
            .unwrap_or(false)
    }
}
