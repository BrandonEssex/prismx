use super::errors::PluginError;
use std::fs;
use std::path::PathBuf;

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        PluginLoader
    }

    pub fn load_all_plugins(&self) -> Result<(), PluginError> {
        let entries = fs::read_dir("extensions").map_err(|_| PluginError::LoadFailure)?;
        for entry in entries {
            let entry = entry.map_err(|_| PluginError::LoadFailure)?;
            let path: PathBuf = entry.path();
            if path.extension().map_or(false, |ext| ext == "prismx-ext") {
                log::info!("Loading plugin from {:?}", path);
                // Simulated loading logic
                // TODO: Validate manifest, run sandbox, etc.
            }
        }
        Ok(())
    }
}
