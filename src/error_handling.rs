use thiserror::Error;
use crate::extension_host::errors::ExtensionHostError;
use crate::spotlight::plugin::SearchScope;

#[derive(Error, Debug)]
pub enum PrismXError {
    #[error("Extension host error: {0}")]
    ExtensionHost(#[from] ExtensionHostError),

    #[error("Spotlight plugin error: {0}")]
    PluginError(String),

    #[error("Invalid search scope: {0:?}")]
    InvalidSearchScope(SearchScope),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("General error: {0}")]
    General(String),
}