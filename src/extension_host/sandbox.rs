use wasmtime::{Engine, Store, Module, Instance, Config, Val};

use crate::extension_host::plugin::Plugin;
use crate::extension_host::capability::{Capability, PermissionSet};
use crate::extension_host::errors::{ExtensionHostError, Result};
use crate::extension_host::profiler::ResourceProfileReport;

pub struct Sandbox {
    engine: Engine,
    permissions: PermissionSet,
}

impl Sandbox {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config).expect("Failed to initialize WASM engine");
        Sandbox {
            engine,
            permissions: PermissionSet::default(),
        }
    }

    pub fn adjust_limits(&mut self, _report: &ResourceProfileReport) {
        // Placeholder for future limit logic
    }

    pub fn set_permissions(&mut self, permissions: PermissionSet) {
        self.permissions = permissions;
    }

    pub fn execute_plugin(&self, plugin: Plugin) -> Result<()> {
        let module = Module::from_file(&self.engine, &plugin.wasm_path)
            .map_err(|e| ExtensionHostError::WasmBinaryNotFound(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        store.add_fuel(10_000).map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let func = instance.get_func(&mut store, "run")
            .ok_or_else(|| ExtensionHostError::EntrypointNotFound("run".to_string()))?;

        func.call(&mut store, &mut [], &mut [])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        Ok(())
    }
}