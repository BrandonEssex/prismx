use prismx::state::AppState;

#[test]
fn module_switcher_displays_correct_labels() {
    let mut state = AppState::default();
    state.module_switcher_index = 0;
    assert_eq!(state.get_module_by_index(), "gemx");
    state.module_switcher_index = 1;
    assert_eq!(state.get_module_by_index(), "zen");
    state.module_switcher_index = 2;
    assert_eq!(state.get_module_by_index(), "triage");
    state.module_switcher_index = 3;
    assert_eq!(state.get_module_by_index(), "settings");
    state.module_switcher_index = 4;
    assert_eq!(state.get_module_by_index(), "plugin");
}
