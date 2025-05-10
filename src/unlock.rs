use crate::state::AppState;

pub fn unlock_interface(state: &mut AppState) {
    // Simulate unlocking gated features
    state.sidebar_visible = true;
}