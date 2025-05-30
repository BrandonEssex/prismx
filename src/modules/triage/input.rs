use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use crate::state::AppState;

pub fn handle_key(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    if !state.sticky_overlay_visible { return false; }
    if let Some(idx) = state.sticky_focus {
        match code {
            KeyCode::Char(c) if mods.is_empty() || mods == KeyModifiers::SHIFT => {
                state.sticky_notes_data[idx].body.push(c);
                return true;
            }
            KeyCode::Backspace => {
                state.sticky_notes_data[idx].body.pop();
                return true;
            }
            KeyCode::Left if mods.contains(KeyModifiers::SHIFT) => {
                state.sticky_notes_data[idx].translate(-1, 0);
                return true;
            }
            KeyCode::Right if mods.contains(KeyModifiers::SHIFT) => {
                state.sticky_notes_data[idx].translate(1, 0);
                return true;
            }
            KeyCode::Up if mods.contains(KeyModifiers::SHIFT) => {
                state.sticky_notes_data[idx].translate(0, -1);
                return true;
            }
            KeyCode::Down if mods.contains(KeyModifiers::SHIFT) => {
                state.sticky_notes_data[idx].translate(0, 1);
                return true;
            }
            _ => {}
        }
    }
    false
}

pub fn handle_mouse(state: &mut AppState, me: MouseEvent) -> bool {
    if !state.sticky_overlay_visible { return false; }
    match me.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(idx) = state.sticky_note_at(me.column, me.row) {
                state.sticky_focus = Some(idx);
                for (i, note) in state.sticky_notes_data.iter_mut().enumerate() {
                    note.focused = i == idx;
                }
                state.sticky_drag.start(me.column, me.row);
                return true;
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if state.sticky_drag.active {
                let (dx, dy) = state.sticky_drag.delta(me.column, me.row);
                if let Some(idx) = state.sticky_focus {
                    state.sticky_notes_data[idx].translate(dx, dy);
                }
                return true;
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            if state.sticky_drag.active {
                state.sticky_drag.stop();
                return true;
            }
        }
        _ => {}
    }
    false
}
