use crate::state::AppState;

/// Open the module switcher overlay and reset selection.
pub fn open_switcher(state: &mut AppState) {
    state.module_switcher_open = true;
    state.module_switcher_index = 0;
}

/// Close the module switcher overlay.
pub fn close_switcher(state: &mut AppState) {
    state.module_switcher_open = false;
}

/// Advance the selection to the next module.
pub fn next_module(state: &mut AppState) {
    state.module_switcher_index = (state.module_switcher_index + 1) % 4;
}

/// Switch the application to the currently selected module.
pub fn select_current(state: &mut AppState) {
    state.mode = state.get_module_by_index().into();
    state.module_switcher_open = false;
}
