use prismx::{state::AppState, node::Node};

#[test]
fn test_tab_no_crash_on_orphan() {
    let mut state = AppState::default();
    state.nodes.clear();
    state.nodes.insert(2, Node::new(2, "Test", None));
    state.root_nodes.clear();
    state.ensure_valid_roots();
    assert!(state.root_nodes.contains(&2));
}
