## Code Changes

- In `render_gemx.rs`:
  - If `state.auto_arrange == true`:
    - Run layout using `layout_nodes()` and `PackRegion`
    - Calculate map bounds (`min_x`, `max_x`, `min_y`, `max_y`)
    - Dynamically set `state.zoom_scale` based on:
      ```rust
      view_width / total_node_width
      ```

- Auto-scroll center:
  ```rust
  state.scroll_x = (center_x * spacing_x) - (viewport_width / 2);
Show label:
Paragraph::new("[A] Auto-Arrange").style(...).render(f, ...)
Add Spotlight command:
if input == "/arrange" {
    state.auto_arrange = true;
}
Optional: Add /zoom reset or /zoom fit aliases
