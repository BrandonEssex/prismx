use crate::node::NodeID;

pub enum SpotlightMode {
    Command,
    Node,
    Tag,
    Jump,
}

pub enum SpotlightResult {
    Command(String),
    Node(NodeID),
    Tag(String),
    Jump(NodeID),
}

pub struct Spotlight {
    pub input_buffer: String,
    pub result_list: Vec<SpotlightResult>,
    pub focus_index: usize,
    pub mode: SpotlightMode,
}

/// Return a preview message for the given command input.
///
/// If the input matches a known command, `Some((message, true))` is returned.
/// Unknown commands starting with `/` return `Some((message, false))`.
/// All other input yields `None`.
pub fn command_preview(input: &str) -> Option<(&'static str, bool)> {
    let trimmed = input.trim();
    if !trimmed.starts_with('/') || trimmed.len() <= 1 {
        return None;
    }

    let (msg, known) = match trimmed {
        "/triage" => ("Switches to Triage panel", true),
        "/zen" => ("Opens Zen mode", true),
        "/gemx" => ("Opens GemX mode", true),
        "/settings" => ("Opens Settings panel", true),
        _ => ("Unknown command", false),
    };

    Some((msg, known))
}

#[cfg(test)]
mod tests {
    use super::command_preview;

    #[test]
    fn known_command_has_preview() {
        assert_eq!(command_preview("/triage"), Some(("Switches to Triage panel", true)));
        assert_eq!(command_preview("/zen"), Some(("Opens Zen mode", true)));
    }

    #[test]
    fn unknown_command_reports_warning() {
        assert_eq!(command_preview("/badcmd"), Some(("Unknown command", false)));
    }
}
