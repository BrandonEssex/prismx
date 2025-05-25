use prismx::state::AppState;

#[test]
fn drill_to_valid_node_focuses_correct_root() {
    let mut state = AppState::default();
    let root = state.selected.unwrap();
    state.add_child();
    let child = state.selected.unwrap();
    state.set_selected(Some(root));
    state.drill_selected();
    assert_eq!(state.drawing_root, Some(root));
    state.set_selected(Some(child));
    state.drill_selected();
    assert_eq!(state.drawing_root, Some(child));
    assert_eq!(state.view_stack.len(), 2);
}

#[test]
fn pop_returns_to_previous_root() {
    let mut state = AppState::default();
    let root = state.selected.unwrap();
    state.add_child();
    let child = state.selected.unwrap();
    state.set_selected(Some(root));
    state.drill_selected();
    state.set_selected(Some(child));
    state.drill_selected();
    state.pop_stack();
    assert_eq!(state.drawing_root, Some(root));
    state.pop_stack();
    assert_eq!(state.drawing_root, None);
}

#[test]
fn drill_fails_gracefully_on_deleted_node() {
    let mut state = AppState::default();
    state.add_child();
    let child = state.selected.unwrap();
    state.delete_node();
    state.selected = Some(child);
    state.drill_selected();
    assert_eq!(state.drawing_root, None);
}

#[test]
fn scroll_and_zoom_reset_on_pop() {
    let mut state = AppState::default();
    state.scroll_x = 5;
    state.scroll_y = 5;
    state.zoom_scale = 1.2;
    state.drill_selected();
    assert_eq!(state.scroll_x, 0);
    assert_eq!(state.scroll_y, 0);
    assert_eq!(state.zoom_scale, 1.0);
    state.scroll_x = 3;
    state.scroll_y = 4;
    state.zoom_scale = 1.3;
    state.pop_stack();
    assert_eq!(state.scroll_x, 0);
    assert_eq!(state.scroll_y, 0);
    assert_eq!(state.zoom_scale, 1.0);
}
