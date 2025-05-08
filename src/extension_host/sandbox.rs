use wasmtime::{Engine, Store, Module, Instance, Config};
use super::plugin::Plugin;
use super::capability::{PermissionSet};
use super::errors::ExtensionHostError;
use log::{info, warn, debug};

pub struct Sandbox {
    engine: Engine,
    memory_limit: usize,
    cpu_cycles: u64,
    permissions: PermissionSet,
}

impl Default for Sandbox {
    fn default() -> Self {
        let mut config = Config::default();
        config.consume_fuel(true);
        Self {
            engine: Engine::new(&config).expect("Wasmtime engine initialization failed"),
            memory_limit: 16 * 1024 * 1024,
            cpu_cycles: 50_000_000,
            permissions: PermissionSet::default(),
        }
    }
}

impl Sandbox {
    pub fn adjust_limits(&mut self) {
        debug!("Adjusting sandbox limits");
    }

    pub fn set_permissions(&mut self, permissions: PermissionSet) {
        debug!("Setting sandbox permissions");
        self.permissions = permissions;
    }

    pub fn execute_plugin(&self, plugin: Plugin) -> Result<(), ExtensionHostError> {
        info!("Executing plugin: {}", plugin.manifest.name);

        let module = Module::from_binary(&self.engine, &plugin.wasm_bytes)
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        store.add_fuel(self.cpu_cycles)
            .map_err(|e| ExtensionHostError::ResourceLimitError(e.to_string()))?;

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let func = instance
            .get_typed_func::<(), ()>(&mut store, &plugin.manifest.entrypoint)
            .map_err(|_| ExtensionHostError::EntrypointNotFound(plugin.manifest.entrypoint.clone()))?;

        func.call(&mut store, ())
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        Ok(())
    }
}