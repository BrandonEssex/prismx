pub mod commands;
pub mod input;
pub mod parser;
pub mod result;
pub mod search;

pub use commands::{command_preview, COMMANDS};
pub use input::{Spotlight, SpotlightMode, SpotlightResult};
pub use result::command_icon;
pub use search::{command_suggestions, command_suggestions_scored, fuzzy_score};

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
