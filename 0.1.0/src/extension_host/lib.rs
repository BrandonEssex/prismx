//! PrismX Extension Host Library
//! Main orchestrator for plugin sandboxing, profiling, and capability management.

mod sandbox;
mod plugin;
mod profiler;
mod capability;
mod errors;

pub use sandbox::Sandbox;
pub use plugin::{Plugin, PluginManifest};
pub use profiler::{ResourceProfiler, ResourceProfileReport};
pub use capability::{Capability, PermissionSet};
pub use errors::{ExtensionHostError, Result};

use tracing::{debug, info};

/// Central entry point orchestrating plugin loading, sandboxing, profiling, and capability enforcement.
pub struct ExtensionHost {
    sandbox: Sandbox,
    profiler: ResourceProfiler,
}

impl ExtensionHost {
    /// Creates a new ExtensionHost instance with default settings.
    pub fn new() -> Self {
        debug!("Initializing PrismX ExtensionHost");
        Self {
            sandbox: Sandbox::default(),
            profiler: ResourceProfiler::default(),
        }
    }

    /// Loads, profiles, and safely executes a plugin at the given path.
    pub fn load_plugin(&mut self, plugin_path: &str, permissions: PermissionSet) -> Result<()> {
        info!("Loading plugin: {}", plugin_path);

        let plugin = Plugin::from_path(plugin_path)?;
        let profile_report = self.profiler.profile_plugin(&plugin.wasm_bytes)?;
        profile_report.log_warnings();
        self.sandbox.adjust_limits(&profile_report)?;
        self.sandbox.set_permissions(permissions)?;
        self.sandbox.execute_plugin(plugin)?;

        info!("Plugin executed successfully: {}", plugin_path);
        Ok(())
    }
}