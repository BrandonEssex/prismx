## Code Changes

### 1. In `handle_enter()` (sibling insert):
- After creating the sibling:
```rust
state.selected = Some(new_id);
If the selected node has no parent:
if state.nodes[&new_id].parent.is_none() && !state.root_nodes.contains(&new_id) {
    state.root_nodes.push(new_id);
}
2. In handle_tab() (child insert):
Validate selection:
let Some(parent_id) = state.selected else {
    eprintln!("⚠ Tab failed: no selected node.");
    return;
};
if !state.nodes.contains_key(&parent_id) {
    eprintln!("⚠ Tab failed: selected node does not exist.");
    return;
}
Ensure parent is promoted if needed:
if !state.root_nodes.contains(&parent_id) {
    state.root_nodes.push(parent_id);
}
After insertion:
state.selected = Some(new_child_id);
3. In render_gemx():
If fallback was already run this frame, do not retry
If layout_nodes returns empty:
f.render_widget(Paragraph::new("⚠ Layout failed — no visible nodes."), area);
return;
Always call:
state.root_nodes.sort_unstable();
state.root_nodes.dedup();
4. Optional: Log invalid insert sequences if both Enter and Tab fail
