// FINAL VERSION — File Delivery Progress: 1/6
// File: src/error_handling.rs

use crate::util::errors::PrismXUtilError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrismXError {
    #[error("Widget initialization error: {0}")]
    WidgetInit(String),

    #[error("Widget rendering error: {0}")]
    WidgetRender(String),

    #[error("Widget event handling error: {0}")]
    WidgetEvent(String),

    #[error("Unknown widget type: {0}")]
    UnknownWidgetType(String),

    #[error("General dashboard error: {0}")]
    General(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Internal utility error: {0}")]
    Util(#[from] PrismXUtilError),
}