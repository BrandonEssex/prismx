pub mod panel;
pub mod logic;

pub use panel::render_triage_panel;
pub use logic::{TriageEntry, TriageSource, handle_inline_command, capture_entry, update_pipeline};
