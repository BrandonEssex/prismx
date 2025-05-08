use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtensionHostError {
    #[error("Plugin manifest not found at path: {0}")]
    ManifestNotFound(String),

    #[error("Failed to parse plugin manifest: {0}")]
    ManifestParseError(String),

    #[error("Plugin WASM binary not found at path: {0}")]
    WasmBinaryNotFound(String),

    #[error("Entrypoint function '{0}' not found in WASM module")]
    EntrypointNotFound(String),

    #[error("Plugin execution failed: {0}")]
    PluginExecutionError(String),

    #[error("Profiling error: {0}")]
    ProfilingError(String),
}