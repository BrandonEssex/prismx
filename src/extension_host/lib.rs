use super::plugin::Plugin;
use super::profiler::ResourceProfiler;
use super::capability::PermissionSet;
use super::sandbox::Sandbox;
use super::errors::ExtensionHostError;

pub struct ExtensionHost {
    sandbox: Sandbox,
    profiler: ResourceProfiler,
}

impl ExtensionHost {
    pub fn new() -> Self {
        Self {
            sandbox: Sandbox::new(),
            profiler: ResourceProfiler::new(),
        }
    }

    pub fn load_plugin(&mut self, plugin_path: &str, permissions: PermissionSet) -> Result<(), ExtensionHostError> {
        // Load plugin and validate manifest
        let plugin = Plugin::from_path(plugin_path)?;

        // Profile and configure
        let profile_report = self.profiler.profile_plugin(&plugin.wasm_bytes)?;
        self.sandbox.configure_limits(profile_report);
        self.sandbox.set_permissions(permissions);
        self.sandbox.execute_plugin(plugin)
    }
}