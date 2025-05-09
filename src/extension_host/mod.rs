pub mod capability;
pub mod errors;
pub mod extension;
pub mod plugin_loader;
pub mod profiler;
pub mod sandbox;
pub mod sandbox_host;

use self::plugin_loader::PluginLoader;

pub struct ExtensionHost {
    loader: PluginLoader,
}

impl ExtensionHost {
    pub fn new() -> Self {
        Self {
            loader: PluginLoader::new(),
        }
    }

    pub fn load_all(&mut self) -> Result<(), self::errors::PluginError> {
        self.loader.load_all_plugins()
    }
}