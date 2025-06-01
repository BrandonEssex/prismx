use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use prismx::state::AppState;
use prismx::modules::switcher;

#[test]
fn handle_key_open_and_navigation() {
    let mut state = AppState::default();
    // Open via Shift+Tab
    assert!(!state.module_switcher_open);
    assert!(switcher::input::handle_key(&mut state, KeyCode::BackTab, KeyModifiers::NONE));
    assert!(state.module_switcher_open);
    let start_index = state.module_switcher_index;
    // Right arrow increments
    assert!(switcher::input::handle_key(&mut state, KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(state.module_switcher_index, (start_index + 1) % switcher::MODULES.len());
    // Left arrow decrements
    assert!(switcher::input::handle_key(&mut state, KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(state.module_switcher_index, start_index);
}

#[test]
fn handle_key_close() {
    let mut state = AppState::default();
    state.module_switcher_open = true;
    state.module_switcher_index = 0;
    // Esc closes
    assert!(switcher::input::handle_key(&mut state, KeyCode::Esc, KeyModifiers::NONE));
    assert!(!state.module_switcher_open);
}
