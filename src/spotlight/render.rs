pub fn command_icon(cmd: &str) -> &'static str {
    match cmd {
        "zen" => "📄",
        "settings" => "⚙️",
        "triage" => "🧭",
        "gemx" => "💭",
        "plugin" => "🔌",
        "search" => "🔍",
        _ => "🔍",
    }
}
