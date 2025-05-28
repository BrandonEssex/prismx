## 1. Update layout_recursive_safe()

Replace sibling layout loop with:

```rust
let count = node.children.len();
let mid = count / 2;

for (i, &child_id) in node.children.iter().enumerate() {
    let offset_x = (i as i16 - mid as i16) * SIBLING_SPACING_X;
    let child_x = x + offset_x;
    let child_y = y + CHILD_SPACING_Y;
    layout_recursive_safe(..., child_id, child_x, child_y, ...)
}
2. Remove fallback sibling layout logic from spawn_free_node

Free nodes should be grid-based. Tree nodes use layout.rs logic only.

3. Respect Manual Drag When auto_arrange = false

Do not recalculate layout when auto-arrange is off

