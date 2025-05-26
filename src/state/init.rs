use crate::plugin::loader;
use crate::plugin::registry;

/// Initialize runtime features on startup.
pub fn init() {
    let plugin_dir = std::path::Path::new("plugins");
    let plugins = loader::discover_plugins(plugin_dir);
    if plugins.is_empty() {
        tracing::info!("[INIT] no dynamic plugins found");
    } else {
        tracing::info!("[INIT] {} plugins discovered", plugins.len());
        for plug in &plugins {
            tracing::debug!("[INIT] plugin available: {}", plug.path.display());
        }
    }
    registry::init();
}

/// Reload all plugins in the `plugins/` directory. Only `.so` and `.dylib`
/// files are loaded.
pub fn reload_plugins() {
    let plugin_dir = std::path::Path::new("plugins");
    if let Ok(entries) = std::fs::read_dir(plugin_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if matches!(path.extension().and_then(|e| e.to_str()), Some("so") | Some("dylib")) {
                if let Some(p) = path.to_str() {
                    let _ = loader::load_plugin(p);
                }
            }
        }
    }
}
