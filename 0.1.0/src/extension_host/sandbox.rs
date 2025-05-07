//! Sandbox environment management using Wasmtime.

use wasmtime::{Engine, Store, Module, Instance, Limits, Config};
use crate::{Plugin, Capability, PermissionSet, errors::{Result, ExtensionHostError}};
use tracing::{info, warn, debug};

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
    pub fn adjust_limits(&mut self, profile_report: &crate::ResourceProfileReport) -> Result<()> {
        debug!("Adjusting sandbox limits based on profiling");
        if profile_report.estimated_memory_bytes > self.memory_limit {
            warn!("Plugin estimated memory ({}) exceeds default limit ({}); adjusting accordingly.",
                  profile_report.estimated_memory_bytes, self.memory_limit);
            self.memory_limit = profile_report.estimated_memory_bytes;
        }

        if profile_report.estimated_cpu_cycles > self.cpu_cycles {
            warn!("Plugin estimated CPU cycles ({}) exceed default limit ({}); adjusting accordingly.",
                  profile_report.estimated_cpu_cycles, self.cpu_cycles);
            self.cpu_cycles = profile_report.estimated_cpu_cycles;
        }

        Ok(())
    }

    pub fn set_permissions(&mut self, permissions: PermissionSet) -> Result<()> {
        debug!("Setting sandbox permissions");
        self.permissions = permissions;
        Ok(())
    }

    pub fn execute_plugin(&self, plugin: Plugin) -> Result<()> {
        info!("Executing plugin: {}", plugin.manifest.name);

        let module = Module::from_binary(&self.engine, &plugin.wasm_bytes)
            .map_err(|e| ExtensionHostError::SandboxInitializationError(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        store.add_fuel(self.cpu_cycles)
            .map_err(|e| ExtensionHostError::ResourceLimitError(e.to_string()))?;

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        let entrypoint = plugin.manifest.entrypoint;
        let func = instance.get_typed_func::<(), ()>(&mut store, &entrypoint)
            .map_err(|_| ExtensionHostError::EntrypointNotFound(entrypoint.clone()))?;

        func.call(&mut store, ())
            .map_err(|e| ExtensionHostError::PluginExecutionError(e.to_string()))?;

        info!("Plugin execution completed: {}", plugin.manifest.name);
        Ok(())
    }
}