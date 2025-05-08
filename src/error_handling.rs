use thiserror::Error;

#[derive(Debug, Error)]
pub enum DashboardError {
    #[error("Failed to load dashboard configuration: {0}")]
    ConfigLoadError(String),

    #[error("Failed to parse dashboard configuration: {0}")]
    ConfigParseError(String),

    #[error("Failed to save dashboard configuration: {0}")]
    ConfigSaveError(String),

    #[error("Widget initialization error: {0}")]
    WidgetInitError(String),

    #[error("Widget rendering error: {0}")]
    WidgetRenderError(String),

    #[error("Widget event handling error: {0}")]
    WidgetEventError(String),

    #[error("Unknown widget type: {0}")]
    UnknownWidgetType(String),

    #[error("General dashboard error: {0}")]
    GeneralError(String),
}

#[derive(Debug, Error)]
pub enum PrismXError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Dashboard module error: {0}")]
    Dashboard(#[from] DashboardError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}