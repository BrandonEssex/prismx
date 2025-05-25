use prismx::state::AppState;

#[test]
fn spotlight_exec_triage_switches_mode() {
    let mut state = AppState::default();
    state.spotlight_input = "/triage".into();
    state.execute_spotlight_command();
    assert_eq!(state.mode, "triage");
}

#[test]
fn spotlight_exec_settings_switches_mode() {
    let mut state = AppState::default();
    state.spotlight_input = "/settings".into();
    state.execute_spotlight_command();
    assert_eq!(state.mode, "settings");
}

#[test]
fn spotlight_exec_closes_panel() {
    let mut state = AppState::default();
    state.show_spotlight = true;
    state.spotlight_input = "/zen".into();
    state.execute_spotlight_command();
    assert!(!state.show_spotlight);
}

#[test]
fn spotlight_input_clears_after_execution() {
    let mut state = AppState::default();
    state.spotlight_input = "/gemx".into();
    state.execute_spotlight_command();
    assert!(state.spotlight_input.is_empty());
}
