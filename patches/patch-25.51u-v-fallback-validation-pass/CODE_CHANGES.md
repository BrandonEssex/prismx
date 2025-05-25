## Code Changes

### 1. Validate Promoted Node After Insertion

After fallback promotion:
```rust
let Some(n) = state.nodes.get_mut(&id) else {
    eprintln!("❌ Fallback failed: Node {} not found in state.nodes", id);
    return;
};

if n.x == 0 && n.y == 0 {
    n.x = 6 + ((id as i16) % 10) * SIBLING_SPACING_X;
    n.y = GEMX_HEADER_HEIGHT + 3;
}

drawn_at.insert(id, Coords { x: n.x, y: n.y });
node_roles.insert(id, LayoutRole::Root);
2. Assert Layout Conditions After Render
At end of render_gemx():

for &id in &state.root_nodes {
    if !drawn_at.contains_key(&id) {
        eprintln!("❌ Layout failed to draw root node {}", id);
    }
}
Optional hard stop:

if drawn_at.is_empty() {
    panic!("❌ Layout aborted: No root nodes successfully rendered.");
}
3. Debug Fallback Log with Full Chain
eprintln!("✅ Promoted Node {}: role=Root, x={}, y={}", id, n.x, n.y);
