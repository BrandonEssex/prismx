use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum MindmapError {
    #[error("Node not found: {0}")]
    NodeNotFound(u64),

    #[error("Invalid coordinates provided: ({0}, {1})")]
    InvalidCoordinates(f64, f64),

    #[error("Persistence failure: {0}")]
    PersistenceFailure(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(String),

    #[error("TUI rendering or event error: {0}")]
    TUIError(String),

    #[error("Configuration load error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Async runtime error: {0}")]
    AsyncRuntimeError(#[from] tokio::task::JoinError),
}