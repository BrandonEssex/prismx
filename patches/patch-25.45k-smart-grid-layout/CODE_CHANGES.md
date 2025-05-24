
## 1. Smart Free Node Positioning
- `spawn_free_node()` in `src/gemx/interaction.rs` now scans the fallback grid
  until an empty slot is found. Existing node positions are checked to avoid
  overlap. Spacing multiplier increased from `*2` to `*3`.

## 2. Grid Position Enforcement
- `ensure_grid_positions()` in `src/state/mod.rs` performs the same scan when
  assigning coordinates for nodes that have none while auto-arrange is disabled.
  Occupied positions are tracked in a `HashSet`.

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

