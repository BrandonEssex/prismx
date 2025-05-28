use crate::plugin::loader;
use crate::plugin::registry;

/// Initialize runtime features on startup.
pub fn init() {
    loader::load_plugins();
    registry::init();
}

/// Reload all plugins in the `plugins/` directory. Only `.so` and `.dylib`
/// files are loaded.
pub fn reload_plugins() {
    loader::load_plugins();
}
