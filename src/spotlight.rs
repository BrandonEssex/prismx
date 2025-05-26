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

pub const COMMANDS: [&str; 5] = ["triage", "zen", "settings", "goto", "plugin"];

/// Compute a fuzzy match score. Lower scores are better. Returns `None` if the
/// query characters do not appear in order within the candidate string.
fn fuzzy_score(candidate: &str, query: &str) -> Option<usize> {
    let mut pos = 0;
    let cand = candidate.to_ascii_lowercase();
    let query = query.to_ascii_lowercase();
    for ch in query.chars() {
        if let Some(idx) = cand[pos..].find(ch) {
            pos += idx + 1;
        } else {
            return None;
        }
    }
    Some(pos - query.len())
}

/// Return command suggestions using fuzzy matching against the input.
pub fn command_suggestions(input: &str) -> Vec<&'static str> {
    let mut scored: Vec<(usize, &str)> = COMMANDS
        .iter()
        .filter_map(|c| fuzzy_score(c, input).map(|s| (s, *c)))
        .collect();
    scored.sort_by_key(|t| t.0);
    scored.into_iter().map(|t| t.1).take(5).collect()
}

/// Return a preview message for the given command input.
///
/// If the input matches a known command, `Some((message, true))` is returned.
/// Unknown commands starting with `/` return `Some((message, false))`.
/// All other input yields `None`.
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

#[cfg(test)]
mod tests {
    use super::command_preview;

    #[test]
    fn known_command_has_preview() {
        assert_eq!(command_preview("triage"), Some(("Switches to Triage panel", true)));
        assert_eq!(command_preview("zen"), Some(("Opens Zen mode", true)));
    }

    #[test]
    fn unknown_command_reports_warning() {
        assert_eq!(command_preview("badcmd"), Some(("Unknown command", false)));
    }
}
