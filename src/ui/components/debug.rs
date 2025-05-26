use crate::state::{AppState, DebugSnapshot};
use std::time::Instant;

/// Capture a debug snapshot of the current app state and write it to the
/// `logs/snapshots/` directory. The filename is timestamped using local time.
pub fn write_debug_snapshot(state: &mut AppState) {
    let snapshot = DebugSnapshot::from_state(state);
    match crate::logging::write_snapshot(&snapshot) {
        Ok(path) => {
            state.status_message = format!("Snapshot saved to {}", path.display());
            state.status_message_last_updated = Some(Instant::now());
        }
        Err(err) => {
            state.status_message = format!("Snapshot failed: {err}");
            state.status_message_last_updated = Some(Instant::now());
        }
    }
}
