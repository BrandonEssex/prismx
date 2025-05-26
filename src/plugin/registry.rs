#[derive(Clone, Copy)]
pub struct PluginEntry {
    pub name: &'static str,
    pub description: &'static str,
}

pub fn registry() -> Vec<PluginEntry> {
    vec![
        PluginEntry { name: "GemX", description: "Mindmap engine" },
        PluginEntry { name: "Dashboard", description: "Project dashboard" },
        PluginEntry { name: "Mindtrace", description: "AI memory system" },
        PluginEntry { name: "RoutineForge", description: "Task & habit manager" },
    ]
}
