## Code Changes

- In `handle_tab()` (or wherever Tab inserts a child), insert:
```rust
state.ensure_valid_roots(); // before layout_nodes() is called
Refactor render_gemx() to early-return if root_nodes is still empty:
if state.root_nodes.is_empty() {
    f.render_widget(Paragraph::new("⚠ No valid root nodes."), Rect::new(area.x + 2, area.y + 2, 40, 1));
    return;
}
Deduplicate root promotion spam:
Only promote nodes not already promoted
Use HashSet or dedup() immediately after modification
Add unit test safety layer:
#[test]
fn test_tab_no_crash_on_orphan() {
    let mut state = AppState::new();
    state.nodes.insert(2, Node::new("Test", None));
    state.root_nodes.clear();
    state.ensure_valid_roots();
    assert_eq!(state.root_nodes.contains(&2), true);
}
