use crossterm::event::{KeyCode, KeyModifiers};

use crate::state::{AppState, ZenViewMode};
use crate::state::view::ZenLayoutMode;

/// Handle key events when the log viewer is active.
/// Returns true if the input was processed.
pub fn handle_log_keys(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    match code {
        KeyCode::Up => {
            if state.logs_scroll + 1 < 1000 { // arbitrary cap
                state.logs_scroll += 1;
            }
            true
        }
        KeyCode::Down => {
            if state.logs_scroll > 0 {
                state.logs_scroll -= 1;
            }
            true
        }
        KeyCode::Char(c) if mods.is_empty() || mods == KeyModifiers::SHIFT => {
            state.logs_filter.push(c);
            true
        }
        KeyCode::Backspace => {
            state.logs_filter.pop();
            true
        }
        _ => false,
    }
}

/// Route keystrokes while in Zen mode to the editor handler.
pub fn route_zen_keys(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    if state.mode == "zen"
        && state.zen_layout_mode == ZenLayoutMode::Compose
        && state.zen_view_mode == ZenViewMode::Write
    {
        if code == KeyCode::Char('d') && mods.contains(KeyModifiers::CONTROL) {
            if let Some(idx) = state.zen_history_index.take() {
                state.delete_journal_entry(idx);
            }
            return true;
        }

        crate::zen::editor::handle_key(state, code);
        return true;
    }
    false
}

/// Toggle Zen Write/Review view.
pub fn toggle_zen_view(state: &mut AppState) {
    state.zen_view_mode = match state.zen_view_mode {
        crate::state::ZenViewMode::Write => crate::state::ZenViewMode::Review,
        crate::state::ZenViewMode::Review => crate::state::ZenViewMode::Write,
    };
}

/// Toggle the floating debug overlay when `?` is pressed with no modifiers.
pub fn toggle_debug_overlay_key(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    if code == KeyCode::Char('?') && mods.is_empty() {
        state.debug_overlay = !state.debug_overlay;
        return true;
    }
    false
}

/// Handle mouse input while in GemX/mindmap view.
pub fn handle_gemx_mouse(state: &mut AppState, me: crossterm::event::MouseEvent) {
    use crossterm::event::{MouseButton, MouseEventKind};

    match me.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(id) = crate::gemx::interaction::node_at_position(state, me.column, me.row) {
                crate::gemx::interaction::start_drag(state, id, me.column, me.row);
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            crate::gemx::interaction::drag_update(state, me.column, me.row);
        }
        MouseEventKind::Up(MouseButton::Left) => {
            crate::gemx::interaction::end_drag(state);
        }
        _ => {}
    }
}
