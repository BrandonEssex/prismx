use prismx::state::AppState;

#[test]
fn word_count_updates_on_edit() {
    let mut state = AppState::default();
    if let Some(last) = state.zen_buffer.last_mut() {
        last.clear();
        last.push_str("hello world");
    }
    state.update_zen_word_count();
    state.zen_dirty = true;
    assert_eq!(state.zen_word_count, 2);
    assert!(state.zen_dirty);
}

#[test]
fn open_sets_filename_and_resets_dirty() {
    let path = "/tmp/zen_footer_test.txt";
    std::fs::write(path, "one two three").unwrap();
    let mut state = AppState::default();
    state.open_zen_file(path);
    assert_eq!(state.zen_current_filename, path);
    assert_eq!(state.zen_word_count, 3);
    assert!(!state.zen_dirty);
}
