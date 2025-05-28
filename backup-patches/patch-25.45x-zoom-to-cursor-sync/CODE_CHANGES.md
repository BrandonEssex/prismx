## Code Changes

- Add `zoom_to_anchor(selected_node_id)` to recenter view on zoom
- In `draw()`, apply zoom_scale to layout_x/y
- Update `node_at_position()` to inverse scale coordinates
- Update `drag_update()` to apply delta / zoom_scale
- Clamp scroll_x / scroll_y after zoom to content bounds

