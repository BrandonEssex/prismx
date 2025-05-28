/// Helper to pick an icon for a command suggestion.
pub fn command_icon(cmd: &str) -> &'static str {
    match cmd {
        "zen" => "\u{1F4C4}",  // 📄
        "settings" => "\u{2699}\u{FE0F}", // ⚙️
        "triage" => "\u{1F9ED}", // 🧭
        "gemx" => "\u{1F4AD}", // 💭
        "plugin" => "\u{1F50C}", // 🔌
        "search" => "\u{1F50D}", // 🔍
        _ => "\u{1F50D}",       // 🔍
    }
}
