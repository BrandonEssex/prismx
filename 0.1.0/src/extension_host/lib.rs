use super::sandbox::Sandbox;
use super::plugin::Plugin;
use super::profiler::ResourceProfiler;
use super::capability::PermissionSet;
use super::errors::Result;
use tracing::{debug, info};

pub struct ExtensionHost {
    sandbox: Sandbox,
    profiler: ResourceProfiler,
}

impl ExtensionHost {
    pub fn new() -> Self {
        debug!("Initializing PrismX ExtensionHost");
        Self {
            sandbox: Sandbox::default(),
            profiler: ResourceProfiler::default(),
        }
    }

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