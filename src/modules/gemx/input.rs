use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use std::time::Instant;
use crate::state::AppState;
use crate::gemx::interaction::{self, node_at_position, start_drag as start_move,
    drag_update, end_drag};
use crate::modules::gemx::{logic, layout};
use ratatui::prelude::Rect;

/// Handle GemX (mindmap) specific keyboard shortcuts.
pub fn handle_key(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    match code {
        // Create a free node with Ctrl+N only (no Shift)
        KeyCode::Char('n') if mods == KeyModifiers::CONTROL => {
            if state.mode != "gemx" {
                println!("HOTKEY_SCOPE_OK");
                return false;
            }
            state.push_undo();
            crate::gemx::interaction::spawn_free_node(state);
            true
        }
        // Insert sibling on Enter
        KeyCode::Enter if mods.is_empty() => {
            if state.can_insert_node() {
                state.push_undo();
                state.handle_enter_key();
                if let Some(id) = state.selected {
                    layout::focus_new_node(state, id);
                }
            } else {
                state.status_message = "Cannot insert: edit node label first".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Insert child on Tab
        KeyCode::Tab if mods.is_empty() => {
            if state.can_insert_node() {
                state.push_undo();
                state.handle_tab_key();
                if let Some(id) = state.selected {
                    layout::focus_new_node(state, id);
                }
            } else {
                state.status_message = "Cannot insert: edit node label first".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Promote or free on Shift+Tab
        KeyCode::BackTab if mods.is_empty() => {
            state.push_undo();
            state.handle_shift_tab_key();
            true
        }
        // Alt+Right: insert sibling to the right
        KeyCode::Right if mods == KeyModifiers::ALT => {
            if state.can_insert_node() {
                state.push_undo();
                state.handle_enter_key();
                if let Some(id) = state.selected {
                    layout::focus_new_node(state, id);
                }
            } else {
                state.status_message = "Cannot insert: edit node label first".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Alt+Down: insert child below
        KeyCode::Down if mods == KeyModifiers::ALT => {
            if state.can_insert_node() {
                state.push_undo();
                state.handle_tab_key();
                if let Some(id) = state.selected {
                    layout::focus_new_node(state, id);
                }
            } else {
                state.status_message = "Cannot insert: edit node label first".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Alt+Left: insert peer beside parent
        KeyCode::Left if mods == KeyModifiers::ALT => {
            if state.can_insert_node() {
                if let Some(current) = state.selected {
                    state.push_undo();
                    if let Some(parent_id) = logic::parent_id(&state.nodes, current) {
                        state.set_selected(Some(parent_id));
                        state.add_sibling_node();
                        let new_id = state.selected.unwrap();
                        if let Some(p) = state.nodes.get(&parent_id).cloned() {
                            if let Some(n) = state.nodes.get_mut(&new_id) {
                                n.x = p.x - crate::layout::SIBLING_SPACING_X;
                                n.y = p.y;
                            }
                        }
                        let (tw, th) = crossterm::terminal::size().unwrap_or((80, 20));
                        layout::enforce_viewport_bounds(
                            &mut state.nodes,
                            Rect::new(0, 0, tw, th),
                        );
                        layout::focus_new_node(state, new_id);
                    } else {
                        // root node
                        state.add_sibling_node();
                        let new_id = state.selected.unwrap();
                        if let Some(cur) = state.nodes.get(&current).cloned() {
                            if let Some(n) = state.nodes.get_mut(&new_id) {
                                n.x = cur.x - crate::layout::SIBLING_SPACING_X;
                                n.y = cur.y;
                            }
                        }
                        let (tw, th) = crossterm::terminal::size().unwrap_or((80, 20));
                        layout::enforce_viewport_bounds(
                            &mut state.nodes,
                            Rect::new(0, 0, tw, th),
                        );
                        layout::focus_new_node(state, new_id);
                    }
                }
            } else {
                state.status_message = "Cannot insert: edit node label first".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Alt+Up: insert parent above
        KeyCode::Up if mods == KeyModifiers::ALT => {
            if state.can_insert_node() {
                if let Some(current) = state.selected {
                    state.push_undo();
                    state.add_sibling_node();
                    let new_id = state.selected.unwrap();
                    logic::reparent(&mut state.nodes, &mut state.root_nodes, current, Some(new_id));
                    if let Some(cur) = state.nodes.get(&current).cloned() {
                        if let Some(n) = state.nodes.get_mut(&new_id) {
                            n.x = cur.x;
                            n.y = cur.y - crate::layout::CHILD_SPACING_Y;
                        }
                    }
                    let (tw, th) = crossterm::terminal::size().unwrap_or((80, 20));
                    layout::enforce_viewport_bounds(&mut state.nodes, Rect::new(0, 0, tw, th));
                    layout::focus_new_node(state, new_id);
                }
            } else {
                state.status_message = "Cannot insert: edit node label first".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Ctrl+Shift+Up: promote selected node
        KeyCode::Up if mods.contains(KeyModifiers::CONTROL) && mods.contains(KeyModifiers::SHIFT) => {
            if let Some(id) = state.selected {
                state.push_undo();
                crate::modules::gemx::logic::promote(&mut state.nodes, &mut state.root_nodes, id);
                state.selection_trail.push_back((id, Instant::now()));
                if state.selection_trail.len() > 8 { state.selection_trail.pop_front(); }
                state.status_message = "Node promoted".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Ctrl+Shift+Down: demote selected node under previous sibling
        KeyCode::Down if mods.contains(KeyModifiers::CONTROL) && mods.contains(KeyModifiers::SHIFT) => {
            if let Some(id) = state.selected {
                state.push_undo();
                crate::modules::gemx::logic::demote_prev_sibling(&mut state.nodes, &mut state.root_nodes, id);
                state.selection_trail.push_back((id, Instant::now()));
                if state.selection_trail.len() > 8 { state.selection_trail.pop_front(); }
                state.status_message = "Node demoted".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        _ => false,
    }
}

/// Handle GemX mouse interactions. Currently unused.
pub fn handle_mouse(state: &mut AppState, me: MouseEvent) -> bool {
    match me.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(id) = node_at_position(state, me.column, me.row) {
                state.push_undo();
                start_move(state, id, me.column, me.row);
                state.drag_hover_target = None;
                state.start_drag();
                return true;
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if state.dragging.is_some() {
                drag_update(state, me.column, me.row);
                if let Some(tid) = node_at_position(state, me.column, me.row) {
                    if Some(tid) != state.dragging {
                        state.drag_hover_target = Some(tid);
                    } else {
                        state.drag_hover_target = None;
                    }
                } else {
                    state.drag_hover_target = None;
                }
                return true;
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            if let Some(id) = state.dragging {
                end_drag(state);
                let target = state.drag_hover_target.take();
                if let Some(tgt) = target {
                    if me.modifiers.contains(KeyModifiers::CONTROL) {
                        state.link_map.entry(id).or_default().push(tgt);
                    } else if logic::node_depth(&state.nodes, id)
                        == logic::node_depth(&state.nodes, tgt)
                    {
                        logic::merge_nodes(
                            &mut state.nodes,
                            &mut state.root_nodes,
                            id,
                            tgt,
                        );
                    } else {
                        logic::reparent(&mut state.nodes, &mut state.root_nodes, id, Some(tgt));
                    }
                }
                logic::adopt_orphans(&mut state.nodes, &mut state.root_nodes);
                return true;
            }
        }
        _ => {}
    }
    false
}
