use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrismXError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}