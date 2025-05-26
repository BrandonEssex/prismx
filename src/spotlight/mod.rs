use crate::node::NodeID;

pub mod parser;
pub mod commands;

pub use commands::{COMMANDS, command_preview};

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

/// Compute a fuzzy match score. Delegates to [`parser::fuzzy_score`].
fn fuzzy_score(candidate: &str, query: &str) -> Option<usize> {
    parser::fuzzy_score(candidate, query)
}

/// Return command suggestions using the predictive ranking parser.
pub fn command_suggestions(input: &str) -> Vec<&'static str> {
    parser::rank(input, &COMMANDS).into_iter().take(5).collect()
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
