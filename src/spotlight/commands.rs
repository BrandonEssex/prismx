pub const COMMANDS: [&str; 5] = ["triage", "zen", "settings", "goto", "plugin"];

pub fn command_preview(input: &str) -> Option<(&'static str, bool)> {
    let trimmed = input.trim().trim_start_matches('/');
    if trimmed.is_empty() {
        return None;
    }

    let (msg, known) = match trimmed {
        "triage" => ("Switches to Triage panel", true),
        "zen" => ("Opens Zen mode", true),
        "gemx" => ("Opens GemX mode", true),
        "settings" => ("Opens Settings panel", true),
        _ => ("Unknown command", false),
    };

    Some((msg, known))
}
