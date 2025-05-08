use crate::extension_host::capability::PermissionSet;
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
            sandbox: Sandbox::new(),
            profiler: ResourceProfiler::new(),
        }
    }

    pub fn load_plugin(&mut self, plugin: Plugin, permissions: PermissionSet) {
        let profile_report: ResourceProfileReport = self.profiler.profile_plugin(&plugin.wasm_bytes);
        self.sandbox.adjust_limits(&profile_report);
        self.sandbox.set_permissions(permissions);
        self.sandbox.execute_plugin(plugin);
    }
}