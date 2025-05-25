## Code Changes

### 1. In `render_gemx()`:

In the `else` block with `collect(...)`, update logic to draw root nodes with no children:
```rust
fn collect(nodes: &NodeMap, id: NodeID, out: &mut HashMap<NodeID, Coords>) {
    if let Some(n) = nodes.get(&id) {
        out.insert(id, Coords { x: n.x, y: n.y });
        if !n.collapsed {
            for child in &n.children {
                collect(nodes, *child, out);
            }
        }
    }
}
→ Already correct, but make sure you also draw root-only nodes even if they’re not connected:

for &root_id in &roots {
    if !state.nodes.contains_key(&root_id) {
        continue;
    }
    collect(&state.nodes, root_id, &mut drawn_at);
    let (_, roles) = layout_nodes(&state.nodes, root_id, 0, area.width as i16, state.auto_arrange);
    node_roles.extend(roles);
}
2. In add_sibling():
If parent_id is None, fallback to:

// Assign sibling as new root
self.root_nodes.push(new_id);
self.root_nodes.sort_unstable();
self.root_nodes.dedup();
Optional: assign a temporary offset (to avoid overlap)
if !self.auto_arrange {
    sibling.x = 2 + (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
    sibling.y = GEMX_HEADER_HEIGHT + 2;
}
3. In recalculate_roles():
Ensure root nodes always get assigned LayoutRole::Root, even with no children.

