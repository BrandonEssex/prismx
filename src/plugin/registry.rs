#[derive(Clone, Copy)]
pub struct PluginEntry {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub trusted: bool,
    pub trust_chain: &'static str,
}

pub fn registry() -> Vec<PluginEntry> {
    vec![
        PluginEntry {
            name: "GemX",
            version: "0.1.0",
            description: "Mindmap engine",
            trusted: true,
            trust_chain: "PrismX Core",
        },
        PluginEntry {
            name: "Dashboard",
            version: "0.1.0",
            description: "Project dashboard",
            trusted: true,
            trust_chain: "PrismX Core",
        },
        PluginEntry {
            name: "Mindtrace",
            version: "0.1.0",
            description: "AI memory system",
            trusted: false,
            trust_chain: "unknown",
        },
        PluginEntry {
            name: "RoutineForge",
            version: "0.1.0",
            description: "Task & habit manager",
            trusted: false,
            trust_chain: "unknown",
        },
    ]
}
