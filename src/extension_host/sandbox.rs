use crate::extension_host::capability::{Capability, PermissionSet};
use crate::extension_host::errors::ExtensionHostError;
use crate::extension_host::plugin::Plugin;
use crate::extension_host::profiler::ResourceProfileReport;
use wasmtime::{Engine, Store, Module, Instance};

pub struct Sandbox {
    engine: Engine,
    memory_limit: usize,
    cpu_cycles: u64,
    permissions: PermissionSet,
}

impl Sandbox {
    pub fn new() -> Self {
        let mut config = wasmtime::Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config).unwrap();
        Sandbox {
            engine,
            memory_limit: 16 * 1024 * 1024,
            cpu_cycles: 50_000_000,
            permissions: PermissionSet::default(),
        }
    }

    pub fn adjust_limits(&mut self, report: &ResourceProfileReport) {
        self.memory_limit = report.estimated_memory_bytes;
        self.cpu_cycles = report.estimated_cpu_cycles;
    }

    pub fn set_permissions(&mut self, perms: PermissionSet) {
        self.permissions = perms;
    }

    pub fn execute_plugin(&self, plugin: Plugin) -> Result<(), ExtensionHostError> {
        let module = Module::from_binary(&self.engine, &plugin.wasm_bytes)
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        store.add_fuel(self.cpu_cycles)
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let func = instance
            .get_func(&mut store, &plugin.manifest.entrypoint)
            .ok_or_else(|| ExtensionHostError::EntrypointNotFound(plugin.manifest.entrypoint.clone()))?;

        func.call(&mut store, &[], &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        Ok(())
    }
}