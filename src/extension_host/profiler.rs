use super::errors::ExtensionHostError;
use wasmtime::{Engine, Module};

#[derive(Debug)]
pub struct ResourceProfileReport {
    pub estimated_memory_bytes: usize,
    pub estimated_cpu_cycles: u64,
}

pub struct ResourceProfiler {
    engine: Engine,
}

impl ResourceProfiler {
    pub fn new() -> Self {
        Self {
            engine: Engine::default(),
        }
    }

    pub fn profile_plugin(&self, wasm_bytes: &[u8]) -> Result<ResourceProfileReport, ExtensionHostError> {
        let _ = Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| ExtensionHostError::ProfilingError(e.to_string()))?;

        Ok(ResourceProfileReport {
            estimated_memory_bytes: wasm_bytes.len() * 2,
            estimated_cpu_cycles: 50_000_000,
        })
    }
}