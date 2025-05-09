use crate::extension_host::capability::Capability;
use std::fs;
use std::collections::HashSet;
use serde::Deserialize;
use log::info;

#[derive(Deserialize)]
pub struct PluginManifestV2 {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub entry: String,
}

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_all_plugins(&self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir("extensions")? {
            let path = entry?.path();
            if path.extension().map_or(false, |e| e == "json") {
                let content = fs::read_to_string(&path)?;
                let manifest: PluginManifestV2 = serde_json::from_str(&content)?;
                info!("Loaded plugin: {}", manifest.name);

                let declared: HashSet<_> = manifest.capabilities.iter().collect();

                if declared.contains(&"network:fetch".to_string()) {
                    info!("[{}] requesting network access", manifest.name);
                }
            }
        }

        Ok(())
    }
}