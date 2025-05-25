use prismx::state::{AppState, register_plugin_favorite};

#[test]
fn plugin_favorite_triggers_spotlight() {
    let mut state = AppState::default();
    state.favorite_dock_limit = 5;
    register_plugin_favorite(&mut state, "🔨", "/run-tests");
    state.trigger_favorite(0); // plugin comes first
    assert_eq!(state.spotlight_input, "/run-tests");
    assert!(state.show_spotlight);
}
