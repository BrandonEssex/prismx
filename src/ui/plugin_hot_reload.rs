use crate::plugins::registry::{PluginManifest, PluginRegistry};
use std::path::PathBuf;

pub fn reload_plugin(registry: &mut PluginRegistry, plugin_id: &str, plugin_path: &str) -> bool {
    let manifest_path = PathBuf::from(plugin_path).join("manifest.toml");
    if let Some(updated_manifest) = PluginManifest::from_path(manifest_path) {
        registry.plugins.insert(plugin_id.to_string(), updated_manifest);
        return true;
    }
    false
}
