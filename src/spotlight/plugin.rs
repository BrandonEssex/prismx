// Removed unused: use std::collections::HashMap;

// Retained private scaffolding for future plugin registry, but not exported to avoid unused warnings.

#[allow(dead_code)]
enum SearchScope {
    Notes,
    Todos,
    Projects,
    All,
}

#[allow(dead_code)]
struct PluginRegistry;

#[allow(dead_code)]
impl PluginRegistry {
    pub fn register_plugin(&self, _name: &str) {
        // Future implementation
    }
}