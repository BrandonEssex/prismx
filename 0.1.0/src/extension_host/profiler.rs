use wasmtime::{Engine, Module};
use super::errors::{Result, ExtensionHostError};
use tracing::{debug, warn, info};

#[derive(Debug)]
pub struct ResourceProfileReport {
    pub estimated_memory_bytes: usize,
    pub estimated_cpu_cycles: u64,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

impl ResourceProfileReport {
    pub fn log_warnings(&self) {
        for warning in &self.warnings {
            warn!("{}", warning);
        }
        for recommendation in &self.recommendations {
            info!("Recommendation: {}", recommendation);
        }
    }
}

#[derive(Default)]
pub struct ResourceProfiler {
    engine: Engine,
}

impl ResourceProfiler {
    pub fn profile_plugin(&self, wasm_bytes: &[u8]) -> Result<ResourceProfileReport> {
        debug!("Profiling plugin resources");

        let module = Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| ExtensionHostError::ProfilingError(e.to_string()))?;

        let estimated_memory_bytes = module.serialize()?.len() * 2;
        let estimated_cpu_cycles = (estimated_memory_bytes as u64) * 10;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        if estimated_memory_bytes > 20 * 1024 * 1024 {
            warnings.push(format!("High memory usage predicted: {} bytes", estimated_memory_bytes));
            recommendations.push("Consider reducing plugin memory footprint.".into());
        }

        if estimated_cpu_cycles > 100_000_000 {
            warnings.push(format!("High CPU usage predicted: {} cycles", estimated_cpu_cycles));
            recommendations.push("Optimize computational complexity to reduce CPU load.".into());
        }

        Ok(ResourceProfileReport {
            estimated_memory_bytes,
            estimated_cpu_cycles,
            warnings,
            recommendations,
        })
    }
}