use prismx::state::AppState;

#[test]
fn dock_focus_updates_index_and_status() {
    let mut state = AppState::default();
    state.dock_focus_next();
    assert_eq!(state.dock_focus_index, Some(0));
    assert_eq!(state.status_message, "/settings");
    state.dock_focus_next();
    assert_eq!(state.dock_focus_index, Some(1));
    assert_eq!(state.status_message, "/triage");
    // wrap around
    for _ in 0..10 {
        state.dock_focus_next();
    }
    assert!(state.dock_focus_index.is_some());
}
