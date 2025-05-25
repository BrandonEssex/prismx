## Code Changes

### 1. In `handle_enter()` (sibling insert):
- After insertion, immediately set:
```rust
state.selected = Some(new_node_id);
Ensure new node inherits parent from current selection:
let parent_id = state.nodes.get(&selected_id).and_then(|n| n.parent);
2. In handle_tab() (child insert):
Guard clause at top:
let Some(parent_id) = state.selected else {
    eprintln!("⚠ Tab insert failed: no selected node.");
    return;
};
if !state.nodes.contains_key(&parent_id) {
    eprintln!("⚠ Tab insert failed: selected node is invalid.");
    return;
}
After inserting the child:
state.selected = Some(new_child_id);
3. Prevent layout wipe
Confirm fallback in render_gemx() only triggers after selection is valid
If layout is empty but selected exists, show fallback message once, and exit gracefully
