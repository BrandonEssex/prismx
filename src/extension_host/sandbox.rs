use super::plugin::Plugin;
use super::capability::{PermissionSet};
use super::errors::ExtensionHostError;
use log::{info, debug};
use wasmtime::{Engine, Store, Module, Instance};

pub struct Sandbox {
    engine: Engine,
    memory_limit: usize,
    cpu_cycles: u64,
    permissions: PermissionSet,
}

impl Sandbox {
    pub fn new() -> Self {
        let engine = Engine::default();
        Sandbox {
            engine,
            memory_limit: 32 * 1024 * 1024,
            cpu_cycles: 1_000_000_000,
            permissions: PermissionSet::default(),
        }
    }

    pub fn configure_limits(&mut self, profile_report: super::profiler::ResourceProfileReport) {
        debug!("Adjusting sandbox limits");
        self.memory_limit = profile_report.estimated_memory_bytes;
        self.cpu_cycles = profile_report.estimated_cpu_cycles;
    }

    pub fn set_permissions(&mut self, permissions: PermissionSet) {
        self.permissions = permissions;
    }

    pub fn execute_plugin(&self, plugin: Plugin) -> Result<(), ExtensionHostError> {
        let module = Module::from_binary(&self.engine, &plugin.wasm_bytes)
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        store.add_fuel(self.cpu_cycles)
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let func = instance.get_func(&mut store, &plugin.manifest.entrypoint)
            .ok_or_else(|| ExtensionHostError::EntrypointNotFound(plugin.manifest.entrypoint.clone()))?;

        func.call(&mut store, &[], &mut [])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        info!("Plugin executed successfully.");
        Ok(())
    }
}