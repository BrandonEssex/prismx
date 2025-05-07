use wasmtime::{Engine, Module};
use super::errors::{ExtensionHostError, Result};

#[derive(Debug)]
pub struct ResourceProfileReport {
    pub estimated_memory_bytes: usize,
    pub estimated_cpu_cycles: u64,
}

#[derive(Default)]
pub struct ResourceProfiler {
    engine: Engine,
}

impl ResourceProfiler {
    pub fn profile_plugin(&self, wasm_bytes: &[u8]) -> Result<ResourceProfileReport> {
        let module = Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| ExtensionHostError::ProfilingError(e.to_string()))?;

        let estimated_memory_bytes = module.serialize().map_err(|e| {
            ExtensionHostError::ProfilingError(format!("Serialization failed: {e}"))
        })?.len() * 2;

        let estimated_cpu_cycles = (estimated_memory_bytes as u64) * 10;

        Ok(ResourceProfileReport {
            estimated_memory_bytes,
            estimated_cpu_cycles,
        })
    }
}