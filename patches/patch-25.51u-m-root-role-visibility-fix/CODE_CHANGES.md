## Code Changes

### 1. In `recalculate_roles()`:
Ensure all declared root nodes have role `Root`:
```rust
for &root_id in &self.root_nodes {
    self.layout_roles.insert(root_id, LayoutRole::Root);
}
2. In render_gemx():
Ensure all root nodes are drawn even if disconnected:

for &root_id in &state.root_nodes {
    if !drawn_at.contains_key(&root_id) {
        if let Some(n) = state.nodes.get(&root_id) {
            drawn_at.insert(root_id, Coords { x: n.x, y: n.y });
            node_roles.insert(root_id, LayoutRole::Root);
        }
    }
}
3. Fallback Guard Enhancement:
In fallback loop:

if reachable_ids.contains(id)
    || state.root_nodes.contains(id)
    || drawn_at.contains_key(&id)
    || state.fallback_this_frame
    || state.fallback_promoted_this_session.contains(id)
{
    continue;
}
Only promote truly invisible and non-rooted nodes.

4. Optional: Visual warning when fallback is blocked
if layout fails and fallback skipped:
    render: "⚠ Node exists but not reachable — layout blocked."
