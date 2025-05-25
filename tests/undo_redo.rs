use prismx::state::AppState;

#[test]
fn undo_removes_inserted_node() {
    let mut state = AppState::default();
    let initial = state.nodes.len();
    state.push_undo();
    state.add_sibling();
    assert_eq!(state.nodes.len(), initial + 1);
    state.spotlight_input = "/undo".into();
    state.execute_spotlight_command();
    assert_eq!(state.nodes.len(), initial);
}

#[test]
fn redo_restores_state() {
    let mut state = AppState::default();
    let initial = state.nodes.len();
    state.push_undo();
    state.add_child();
    assert_eq!(state.nodes.len(), initial + 1);
    state.spotlight_input = "/undo".into();
    state.execute_spotlight_command();
    assert_eq!(state.nodes.len(), initial);
    state.spotlight_input = "/redo".into();
    state.execute_spotlight_command();
    assert_eq!(state.nodes.len(), initial + 1);
}
