## Code Changes

### 1. Add Debug Tracer to All Inserts
In `add_child()` and `add_sibling()`:
```rust
if self.debug_input_mode {
    eprintln!(
        "[INSERT] Node {} → label=\"{}\", parent={:?}, x={}, y={}, mode={:?}",
        new_id,
        node.label,
        node.parent,
        node.x,
        node.y,
        self.mode
    );
}
2. In render_gemx() — Tree Draw Log
if state.debug_input_mode {
    eprintln!("[RENDER TREE]");
    for (&id, coords) in &drawn_at {
        let label = &state.nodes[&id].label;
        let role = node_roles.get(&id).unwrap_or(&LayoutRole::Free);
        eprintln!(
            "Node {} → ({}, {}) | {:?} | {}",
            id, coords.x, coords.y, role, label
        );
    }
}
3. Optional: Ghost Box for Undrawn Inserted Node
After fallback:

if !drawn_at.contains_key(&id) {
    drawn_at.insert(id, Coords { x: 3, y: 3 });
    node_roles.insert(id, LayoutRole::Free);
}
4. Force-Fill Coordinates for Empty Roots
if let Some(n) = state.nodes.get_mut(&id) {
    if n.x == 0 && n.y == 0 {
        n.x = 6;
        n.y = GEMX_HEADER_HEIGHT + 3;
    }
}
