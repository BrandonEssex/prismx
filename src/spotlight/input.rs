use crate::node::NodeID;

/// Mode Spotlight is operating in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotlightMode {
    Command,
    Node,
    Tag,
    Jump,
}

/// Result item returned from a Spotlight search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotlightResult {
    Command(String),
    Node(NodeID),
    Tag(String),
    Jump(NodeID),
}

/// Captures user input and selection state for Spotlight.
#[derive(Debug)]
pub struct Spotlight {
    pub input_buffer: String,
    pub result_list: Vec<SpotlightResult>,
    pub focus_index: usize,
    pub mode: SpotlightMode,
}

impl Default for Spotlight {
    fn default() -> Self {
        Self {
            input_buffer: String::new(),
            result_list: Vec::new(),
            focus_index: 0,
            mode: SpotlightMode::Command,
        }
    }
}
