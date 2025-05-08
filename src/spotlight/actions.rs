use crate::spotlight::state::SpotlightState;

pub fn perform_move(state: &SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Move triggered for item: {}", item.title);
        // Placeholder: show shard move interface
    }
}

pub fn perform_delete(state: &SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Delete triggered for item: {}", item.title);
        // Placeholder: show confirmation dialog
    }
}

pub fn perform_export(state: &SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Export triggered for item: {}", item.title);
        // Placeholder: write to .md file
    }
}

pub fn toggle_favorite(state: &SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Favorite toggled for item: {}", item.title);
        // Placeholder: toggle pinned state
    }
}