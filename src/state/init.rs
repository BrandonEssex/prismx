use crate::plugin::loader;
use crate::plugin::registry;

/// Initialize runtime features on startup.
pub fn init() {
    let plugin_dir = std::path::Path::new("plugins");
    let plugins = loader::load_plugins(plugin_dir);
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
    let _ = loader::load_plugins(plugin_dir);
}
