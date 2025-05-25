use prismx::state::AppState;
use crossterm::event::KeyCode;

#[test]
fn toolbar_toggle_via_spotlight() {
    let mut state = AppState::default();
    state.spotlight_input = "/toolbar".into();
    state.execute_spotlight_command();
    assert!(state.zen_toolbar_open);
    state.spotlight_input = "/toolbar".into();
    state.execute_spotlight_command();
    assert!(!state.zen_toolbar_open);
}

#[test]
fn toolbar_navigation_wraps() {
    let mut state = AppState::default();
    state.zen_toolbar_open = true;
    state.zen_toolbar_handle_key(KeyCode::Up);
    assert_eq!(state.zen_toolbar_index, state.zen_toolbar_len() - 1);
    state.zen_toolbar_handle_key(KeyCode::Down);
    assert_eq!(state.zen_toolbar_index, 0);
}

#[test]
fn open_and_save_updates_recent() {
    let mut state = AppState::default();
    let path = "/tmp/zen_test.txt";
    std::fs::write(path, "hello") .unwrap();
    state.open_zen_file(path);
    assert_eq!(state.zen_recent_files[0], path);
    state.save_zen_file(path);
    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.contains("hello"));
}
