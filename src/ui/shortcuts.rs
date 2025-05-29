pub fn action_group(action: &str) -> &'static str {
    if action.starts_with("zen_") || matches!(action, "mode_zen" | "save") {
        "Zen"
    } else if action.contains("triage") {
        "Triage"
    } else if action.contains("plugin") {
        "Plugins"
    } else if action.contains("settings") {
        "Settings"
    } else if action.starts_with("debug") {
        "Debug"
    } else {
        "GemX"
    }
}
