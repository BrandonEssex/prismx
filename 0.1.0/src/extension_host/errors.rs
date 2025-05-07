use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtensionHostError {
    #[error("Plugin manifest not found at path: {0}")]
    ManifestNotFound(String),

    #[error("Failed to parse plugin manifest: {0}")]
    ManifestParseError(String),

    #[error("Plugin WASM binary not found at path: {0}")]
    WasmBinaryNotFound(String),

    #[error("Incompatible plugin API version (expected: {expected}, found: {found})")]
    IncompatibleApiVersion {
        expected: String,
        found: String,
    },

    #[error("Sandbox initialization failed: {0}")]
    SandboxInitializationError(String),

    #[error("Plugin execution failed: {0}")]
    PluginExecutionError(String),

    #[error("Entrypoint function '{0}' not found in WASM binary")]
    EntrypointNotFound(String),

    #[error("Resource profiling failed: {0}")]
    ProfilingError(String),

    #[error("Resource limit error: {0}")]
    ResourceLimitError(String),

    #[error("Capability enforcement violation: {0}")]
    CapabilityViolation(String),

    #[error("Permission set error: {0}")]
    PermissionSetError(String),
}