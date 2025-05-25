## Code Changes

### 1. In `recalculate_roles()`:
```rust
for &id in &self.root_nodes {
    self.layout_roles.insert(id, LayoutRole::Root);
}
2. In layout_nodes():
Ensure root nodes always produce output:

if depth == 0 {
    roles.insert(node_id, LayoutRole::Root);
    out.insert(node_id, Coords { x, y }); // even if no children
    return (y, x, x + node.label.len() as i16 + 2);
}
3. In render_gemx():
Guarantee all root nodes appear:

for &root_id in &state.root_nodes {
    if !drawn_at.contains_key(&root_id) {
        if let Some(n) = state.nodes.get(&root_id) {
            drawn_at.insert(root_id, Coords { x: n.x, y: n.y });
            node_roles.insert(root_id, LayoutRole::Root);
        }
    }
}
4. Aggressive fallback suppression:
In fallback loop:

if reachable_ids.contains(id)
    || state.root_nodes.contains(&id)
    || drawn_at.contains_key(&id)
    || state.fallback_promoted_this_session.contains(id)
{
    continue;
}
And log it only once per ID.
5. Optional: Show ghost if root is missing position
if n.x == 0 && n.y == 0 {
    drawn_at.insert(root_id, Coords { x: 5, y: 5 });
}
