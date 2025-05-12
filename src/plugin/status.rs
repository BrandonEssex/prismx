#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginStatus {
    Active,
    Inactive,
    Error(String),
    Disabled,
}

impl PluginStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PluginStatus::Active => "Active",
            PluginStatus::Inactive => "Inactive",
            PluginStatus::Disabled => "Disabled",
            PluginStatus::Error(_) => "Error",
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, PluginStatus::Error(_))
    }
}