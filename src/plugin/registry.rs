#[derive(Clone, Copy)]
pub struct PluginEntry {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub trusted: bool,
    pub trust_chain: &'static str,
    pub tags: &'static [&'static str],
}

pub fn registry() -> Vec<PluginEntry> {
    vec![
        PluginEntry {
            name: "GemX",
            version: "0.1.0",
            description: "Mindmap engine",
            trusted: true,
            trust_chain: "PrismX Core",
            tags: &["editor", "trusted"],
        },
        PluginEntry {
            name: "Dashboard",
            version: "0.1.0",
            description: "Project dashboard",
            trusted: true,
            trust_chain: "PrismX Core",
            tags: &["utility", "trusted"],
        },
        PluginEntry {
            name: "Mindtrace",
            version: "0.1.0",
            description: "AI memory system",
            trusted: false,
            trust_chain: "unknown",
            tags: &["debug"],
        },
        PluginEntry {
            name: "RoutineForge",
            version: "0.1.0",
            description: "Task & habit manager",
            trusted: false,
            trust_chain: "unknown",
            tags: &["utility"],
        },
    ]
}

use crate::state::PluginTagFilter;

pub fn registry_filtered(filter: PluginTagFilter) -> Vec<PluginEntry> {
    registry()
        .into_iter()
        .filter(|p| match filter {
            PluginTagFilter::All => true,
            PluginTagFilter::Trusted => p.trusted,
            PluginTagFilter::Debug => p.tags.iter().any(|t| *t == "debug"),
        })
        .collect()
}
