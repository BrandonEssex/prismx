use crate::util::errors::PrismXUtilError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrismXError {
    #[error("Failed to load dashboard configuration: {0}")]
    ConfigLoad(String),

    #[error("Failed to parse dashboard configuration: {0}")]
    ConfigParse(String),

    #[error("Failed to save dashboard configuration: {0}")]
    ConfigSave(String),

    #[error("Widget initialization error: {0}")]
    WidgetInit(String),

    #[error("Widget rendering error: {0}")]
    WidgetRender(String),

    #[error("Widget event handling error: {0}")]
    WidgetEvent(String),

    #[error("Unknown widget type: {0}")]
    UnknownWidget(String),

    #[error("General dashboard error: {0}")]
    General(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Dashboard module error: {0}")]
    Dashboard(#[from] PrismXUtilError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}