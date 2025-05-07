use super::plugin::Plugin;
use super::profiler::{ResourceProfiler, ResourceProfileReport};
use super::capability::{PermissionSet};
use super::errors::Result;
use super::sandbox::Sandbox;

pub struct ExtensionHost {
    sandbox: Sandbox,
    profiler: ResourceProfiler,
}

impl ExtensionHost {
    pub fn new() -> Self {
        Self {
            sandbox: Sandbox::default(),
            profiler: ResourceProfiler::default(),
        }
    }

    pub fn load_plugin(&mut self, plugin_path: &str, permissions: PermissionSet) -> Result<()> {
        let plugin = Plugin::from_path(plugin_path)?;

        let profile_report: ResourceProfileReport =
            self.profiler.profile_plugin(&plugin.wasm_bytes)?;

        self.sandbox.adjust_limits(&profile_report)?;
        self.sandbox.set_permissions(permissions)?;
        self.sandbox.execute_plugin(plugin)?;

        Ok(())
    }
}