## 1. Replace Free Node Placement

Use logic similar to:

```rust
let mut y_cursor = view_y + 2;
while y_cursor < max_y {
  for x in 1..term_width {
    if !occupied((x, y_cursor)) { place_node_here }
  }
  y_cursor += 1;
}
2. Auto-Arrange Behavior

Auto-Arrange ON: full layout engine
Auto-Arrange OFF: freeze current layout; allow manual drag without node reset

3. Constants and Safety

Spacing should use at least *3 or *4 multiplier for visible gaps
Preserve existing dragged positions; do not reset manually-placed coords

