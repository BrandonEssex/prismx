pub mod plugin_loader;

use self::plugin_loader::PluginLoader;

pub fn initialize_plugins() {
    let _loader = PluginLoader::new();
    // Plugin load flow to be expanded
}