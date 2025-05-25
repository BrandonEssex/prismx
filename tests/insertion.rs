use prismx::{state::AppState, node::Node};

#[test]
fn enter_adds_sibling_with_same_parent() {
    let mut state = AppState::default();
    if let Some(b) = state.nodes.get_mut(&2) {
        b.x = 10;
    }
    let original = state.selected.unwrap();
    let orig_parent = state.nodes.get(&original).unwrap().parent;
    state.add_sibling();
    let new_id = state.selected.unwrap();
    assert_ne!(original, new_id);
    assert_eq!(state.nodes.get(&new_id).unwrap().parent, orig_parent);
    if let Some(pid) = orig_parent {
        assert!(state.nodes.get(&pid).unwrap().children.contains(&new_id));
    } else {
        assert!(state.root_nodes.contains(&new_id));
    }
}

#[test]
fn tab_adds_child_under_selection() {
    let mut state = AppState::default();
    // separate root nodes so role detection is stable
    if let Some(b) = state.nodes.get_mut(&2) {
        b.x = 10;
    }
    let parent = state.selected.unwrap();
    state.add_child();
    let new_id = state.selected.unwrap();
    assert_eq!(state.nodes.get(&new_id).unwrap().parent, Some(parent));
    assert!(state.nodes.get(&parent).unwrap().children.contains(&new_id));
}

#[test]
fn missing_parent_becomes_free() {
    use prismx::layout::{layout_nodes, LayoutRole};
    let mut state = AppState::default();
    let root = state.root_nodes[0];
    let new_id = 999;
    state.nodes.insert(new_id, Node::new(new_id, "Dangling", Some(123)));
    state.nodes.get_mut(&root).unwrap().children.push(new_id);

    let (_c, roles) = layout_nodes(&state.nodes, root, 0, 80, true);
    assert_eq!(roles.get(&new_id), Some(&LayoutRole::Free));
}

#[test]
fn tab_requires_valid_selection() {
    let mut state = AppState::default();
    state.selected = Some(999);
    let count = state.nodes.len();
    state.add_child();
    assert_eq!(state.nodes.len(), count);
}

#[test]
fn tab_after_enter_keeps_child_reachable() {
    let mut state = AppState::default();
    if let Some(b) = state.nodes.get_mut(&2) { b.x = 10; }
    state.add_sibling();
    let sibling = state.selected.unwrap();
    state.add_child();
    let child = state.selected.unwrap();
    let parent = state.nodes.get(&child).unwrap().parent;
    assert!(parent.is_some());
    assert!(state.nodes.get(&parent.unwrap()).unwrap().children.contains(&child));
}
