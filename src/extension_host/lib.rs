use crate::extension_host::capability::{PermissionSet};
use crate::extension_host::errors::ExtensionHostError;
use crate::extension_host::plugin::Plugin;
use crate::extension_host::profiler::{ResourceProfiler, ResourceProfileReport};
use crate::extension_host::sandbox::Sandbox;

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

    pub fn load_plugin(&mut self, plugin_path: &str, permissions: PermissionSet) -> Result<(), ExtensionHostError> {
        let plugin = Plugin::from_path(plugin_path)?;
        let profile_report = self.profiler.profile_plugin(&plugin.wasm_bytes)?;
        self.sandbox.adjust_limits(profile_report);
        self.sandbox.set_permissions(permissions);
        self.sandbox.execute_plugin(plugin)
    }
}