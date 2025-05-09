#[derive(Debug, Clone)]
pub struct PluginStatus {
    pub name: String,
    pub status: String,
    pub version: String,
    pub hooks: Vec<String>,
    pub last_error: Option<String>,
}