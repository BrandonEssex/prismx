use prismx::state::AppState;
use std::time::{Duration, Instant};

#[test]
fn autosave_saves_when_dirty() {
    let path = "/tmp/zen_autosave.txt";
    let mut state = AppState::default();
    state.save_zen_file(path);
    state.zen_buffer.last_mut().unwrap().push('a');
    state.zen_dirty = true;
    state.zen_last_saved = Some(Instant::now() - Duration::from_secs(11));
    state.auto_save_zen();
    assert!(!state.zen_dirty);
    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.contains('a'));
}

#[test]
fn autosave_skips_when_recently_saved() {
    let path = "/tmp/zen_autosave_skip.txt";
    let mut state = AppState::default();
    state.save_zen_file(path);
    state.zen_buffer.last_mut().unwrap().push('b');
    state.zen_dirty = true;
    state.zen_last_saved = Some(Instant::now());
    state.auto_save_zen();
    assert!(state.zen_dirty);
    let content = std::fs::read_to_string(path).unwrap();
    assert!(!content.contains('b'));
}
