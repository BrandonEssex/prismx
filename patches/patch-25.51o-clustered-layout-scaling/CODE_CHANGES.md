## Code Changes

- Fixed `handle_add_child()`:
  - Properly assigns role and inserts `x`, `y` offsets relative to parent
  - Prevents accidental clear/layout crash

- Added screen-aware auto-zoom:
  - After layout, calculate:
    ```rust
    layout_width = (max_x - min_x + 1) * BASE_SPACING_X;
    layout_height = ...
    ```
    - Set:
    ```rust
    state.zoom_scale = (view_width / layout_width).min(max_zoom)
    ```

- Auto-Arrange no longer locks zoom every frame
  - Add flag: `state.zoom_locked_by_user`
  - Prevent override after manual zooming

- Layout grid spacing:
  - World space is "infinite"
  - PackRegion inserts based on real layout width
  - Small maps zoom in, big maps zoom out

