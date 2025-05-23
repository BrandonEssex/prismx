## 1. Replace Grid Math

In `ensure_grid_positions()` in `src/state/mod.rs`, replace:

    node.x = ((index % 4) * 6) as i16;
    node.y = ((index / 4) * 2) as i16;

With:

    node.x = ((index % FREE_GRID_COLUMNS) * SIBLING_SPACING_X * 2) as i16;
    node.y = ((index / FREE_GRID_COLUMNS) * CHILD_SPACING_Y * 2) as i16;

## 2. Verify Constants in `layout.rs`

Ensure constants are:

    pub const SIBLING_SPACING_X: i16 = 3;
    pub const CHILD_SPACING_Y: i16 = 1;
    pub const FREE_GRID_COLUMNS: usize = 4;

## 3. Confirm `Ctrl+N` Spawn Position Logic

Add/verify logic in `interaction.rs` to avoid spawning new nodes at (0,0)

## 4. Apply Changes Safely

Ensure auto-arrange respects prior manual drag and zoom behavior

