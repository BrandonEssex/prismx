use crate::plugin::loader;

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
}
