use crate::spotlight::state::SpotlightState;

pub fn perform_move(state: &mut SpotlightState) {
    if let Some(item) = state.selected() {
        println!("Move triggered for UID: {}", item.title);
    }
}

pub fn perform_delete(state: &mut SpotlightState) {
    if let Some(item) = state.selected() {
        println!("Delete triggered for UID: {}", item.title);
    }
}

pub fn perform_export(state: &mut SpotlightState) {
    if let Some(item) = state.selected() {
        println!("Export triggered for UID: {}", item.title);
    }
}

pub fn toggle_favorite(state: &mut SpotlightState) {
    if let Some(item) = state.selected() {
        println!("Favorite toggled for UID: {}", item.title);
    }
}