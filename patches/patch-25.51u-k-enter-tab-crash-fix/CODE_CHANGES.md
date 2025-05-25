## Code Changes

- After `Enter`, ensure:
```rust
state.selected = Some(new_sibling_id);
Ensure inserted sibling inherits a valid parent
Before Tab:
let Some(parent_id) = state.selected else { return };
if !state.nodes.contains_key(&parent_id) { return; }

if !state.root_nodes.contains(&parent_id) {
    state.root_nodes.push(parent_id);
}
After inserting the child:
state.selected = Some(new_child_id);
In fallback path, check:
if state.root_nodes.is_empty() {
    // only promote if not already promoted once this frame
}
