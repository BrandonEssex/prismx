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

pub const COMMANDS: [&str; 5] = ["/triage", "/zen", "/settings", "/goto", "/plugin"];

/// Return command suggestions that start with the given input prefix.
pub fn command_suggestions(input: &str) -> Vec<&'static str> {
    COMMANDS
        .iter()
        .filter(|c| c.starts_with(input))
        .copied()
        .take(5)
        .collect()
}
