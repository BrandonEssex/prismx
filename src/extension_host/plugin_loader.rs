use super::errors::PluginError;
use std::fs;

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        PluginLoader
    }

    pub fn load_all_plugins(&self) -> Result<(), PluginError> {
        let entries = fs::read_dir("extensions").map_err(|_| PluginError::LoadFailure)?;
        for entry in entries {
            let path = entry?.path();
            if path.extension().map_or(false, |ext| ext == "prismx-ext") {
                log::info!("Loading plugin from {:?}", path);
                // Simulated load
            }
        }
        Ok(())
    }
}
