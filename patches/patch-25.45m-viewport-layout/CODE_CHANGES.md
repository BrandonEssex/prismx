## 1. Offset Layout Entry Point

Update `layout_nodes()` to use:
```rust
let start_y = GEMX_HEADER_HEIGHT + 1;
and dynamically compute start_x based on screen width.

2. Smart Spawn Positioning

In spawn_free_node() and ensure_grid_positions():

Start Y at GEMX_HEADER_HEIGHT + 1
Ensure no nodes are placed at or near (0,0)
3. Sibling Overflow Compression (Optional)

If screen width is exceeded:

Dynamically reduce SIBLING_SPACING_X
Or wrap to multiple rows (if overflow_protect is enabled)
4. Final Touches

Ensure no layout overlap with status/header/footer areas

