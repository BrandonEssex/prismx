use thiserror::Error;
use crate::dashboard::error::DashboardError;
use serde_json;
use std::io;

#[derive(Debug, Error)]
pub enum PrismXError {
    #[error("Failed to load dashboard configuration: {0}")]
    LoadConfig(String),

    #[error("Failed to parse dashboard configuration: {0}")]
    ParseConfig(String),

    #[error("Failed to save dashboard configuration: {0}")]
    SaveConfig(String),

    #[error("Widget initialization error: {0}")]
    InitError(String),

    #[error("Widget rendering error: {0}")]
    RenderError(String),

    #[error("Widget event handling error: {0}")]
    EventError(String),

    #[error("Unknown widget type: {0}")]
    UnknownWidget(String),

    #[error("General dashboard error: {0}")]
    General(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Dashboard module error: {0}")]
    Dashboard(#[from] DashboardError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
