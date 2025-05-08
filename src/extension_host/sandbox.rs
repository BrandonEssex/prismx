use wasmtime::{Engine, Store, Module, Instance, Config};
use crate::extension_host::plugin::Plugin;
use crate::extension_host::capability::{PermissionSet};
use crate::extension_host::errors::{ExtensionHostError};
use crate::extension_host::profiler::ResourceProfileReport;

pub struct Sandbox {
    engine: Engine,
    memory_limit: usize,
    cpu_cycles: u64,
    permissions: PermissionSet,
}

impl Sandbox {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.consume_fuel(true);
        Self {
            engine: Engine::new(&config).expect("Wasmtime engine initialization failed"),
            memory_limit: 16 * 1024 * 1024,
            cpu_cycles: 50_000_000,
            permissions: PermissionSet::default(),
        }
    }

    pub fn adjust_limits(&mut self, report: &ResourceProfileReport) {
        if report.estimated_memory_bytes > self.memory_limit {
            self.memory_limit = report.estimated_memory_bytes;
        }

        if report.estimated_cpu_cycles > self.cpu_cycles {
            self.cpu_cycles = report.estimated_cpu_cycles;
        }
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

        func.call(&mut store, &[], &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        Ok(())
    }
}