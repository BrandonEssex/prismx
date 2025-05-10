#[derive(Debug, Clone)]
pub enum PluginStatus {
    Active,
    Inactive,
    Error(String),
}