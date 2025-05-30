pub fn action_group(action: &str) -> &'static str {
    if matches!(
        action,
        "quit"
            | "switch_module"
            | "open_module_switcher"
            | "help"
            | "toggle_keymap"
            | "toggle_triage"
            | "toggle_plugin"
            | "toggle_settings"
            | "mode_zen"
    ) {
        "Global"
    } else if action.starts_with("zen_") || matches!(action, "save") {
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

/// Common shortcut mappings by module context.
/// Used by dynamic status bar or shortcut overlay.
pub fn shortcuts_for(module: &str) -> &'static [(&'static str, &'static str)] {
    match module {
        "Zen" => &[
            ("Ctrl+R", "Zen Mode"),
            ("Ctrl+S", "Save Entry"),
            ("Ctrl+L", "Scroll Log"),
            ("Ctrl+Z", "Undo"),
            ("Ctrl+H", "Toggle Help"),
        ],
        "Triage" => &[
            ("Ctrl+Y", "Triage Panel"),
            ("Enter", "Edit Tag"),
            ("Ctrl+D", "Delete Entry"),
            ("Ctrl+T", "Toggle Priority"),
        ],
        "GemX" => &[
            ("Ctrl+N", "New Node"),
            ("Ctrl+W", "Drill Down"),
            ("Ctrl+Q", "Pop Up"),
            ("Ctrl+P", "Auto-Arrange"),
            ("Alt+←/→", "Sibling Nav"),
            ("Ctrl+B", "Add Child"),
        ],
        "Spotlight" => &[
            ("Alt+Space", "Open Spotlight"),
            ("Ctrl+H", "History"),
            ("Esc", "Close"),
        ],
        "Settings" => &[
            ("Ctrl+.", "Open Settings"),
            ("↑/↓", "Navigate"),
            ("Enter", "Apply"),
        ],
        _ => &[
            ("Ctrl+N", "New Node"),
            ("Ctrl+W", "Drill Down"),
            ("Ctrl+Q", "Pop Up"),
            ("Alt+Space", "Spotlight"),
        ],
    }
}
