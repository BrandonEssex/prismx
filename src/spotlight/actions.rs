use crate::spotlight::state::SpotlightState;

pub fn perform_move(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Move triggered for UID: {}", item.uid);
    }
}

pub fn perform_delete(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Delete triggered for UID: {}", item.uid);
    }
}

pub fn perform_export(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Export triggered for UID: {}", item.uid);
    }
}

pub fn toggle_favorite(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        println!("Favorite toggled for UID: {}", item.uid);
    }
}