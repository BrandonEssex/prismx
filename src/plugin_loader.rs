// src/plugin_loader.rs

use crate::plugin_manifest::{load_manifest, PluginManifest};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn discover_plugins<P: AsRef<Path>>(directory: P) -> HashMap<String, PluginManifest> {
    let mut plugins = HashMap::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                if let Some(manifest) = load_manifest(&path) {
                    plugins.insert(manifest.name.clone(), manifest);
                }
            }
        }
    }

    plugins
}