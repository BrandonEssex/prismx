## 1. Smart Free Node Positioning
- `spawn_free_node()` in `src/gemx/interaction.rs` now scans the fallback grid
  until an empty slot is found. Existing node positions are checked to avoid
  overlap. Spacing multiplier increased from `*2` to `*3`.

## 2. Grid Position Enforcement
- `ensure_grid_positions()` in `src/state/mod.rs` performs the same scan when
  assigning coordinates for nodes that have none while auto-arrange is disabled.
  Occupied positions are tracked in a `HashSet`.
