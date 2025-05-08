use wasmtime::{Engine, Module};

use crate::extension_host::errors::{ExtensionHostError, Result};

pub struct ResourceProfiler {
    engine: Engine,
}

impl ResourceProfiler {
    pub fn new() -> Self {
        ResourceProfiler {
            engine: Engine::default(),
        }
    }

    pub fn profile_plugin(&self, wasm_bytes: &[u8]) -> Result<ResourceProfileReport> {
        let module = Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| ExtensionHostError::ProfilingError(e.to_string()))?;

        let estimated_memory_bytes = module.serialize().unwrap_or_default().len() * 2;
        let estimated_cpu_cycles = wasm_bytes.len() as u64 * 10;

        Ok(ResourceProfileReport {
            estimated_memory_bytes,
            estimated_cpu_cycles,
        })
    }
}

#[derive(Debug)]
pub struct ResourceProfileReport {
    pub estimated_memory_bytes: usize,
    pub estimated_cpu_cycles: u64,
}