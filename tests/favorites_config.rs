use prismx::state::{AppState, DockLayout};

#[test]
fn spotlight_dock_limit_command() {
    let mut state = AppState::default();
    state.spotlight_input = "/dock_limit=5".into();
    state.execute_spotlight_command();
    assert_eq!(state.favorite_dock_limit, 5);
}

#[test]
fn spotlight_dock_layout_command() {
    let mut state = AppState::default();
    state.spotlight_input = "/dock_layout=horizontal".into();
    state.execute_spotlight_command();
    assert_eq!(state.favorite_dock_layout, DockLayout::Horizontal);
}
