## Code Changes

### 1. In `add_sibling()` after coordinate assignment:

```rust
if !self.auto_arrange {
    if let Some(selected) = self.nodes.get(&selected_id) {
        sibling.x = selected.x + SIBLING_SPACING_X;
        sibling.y = selected.y;
    } else {
        sibling.x = (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
        sibling.y = GEMX_HEADER_HEIGHT + 2;
    }

    // Clamp and force unique x if x == 0 (indicates bad layout)
    if sibling.x == 0 {
        sibling.x = ((self.nodes.len() as i16) % 5 + 1) * SIBLING_SPACING_X;
    }
}
2. Add explicit debug log
if self.debug_input_mode {
    eprintln!(
        "[add_sibling] Inserted Node {} → x={}, y={}, parent={:?}",
        new_id,
        sibling.x,
        sibling.y,
        parent_id
    );
}
3. Optional: Draw a ghost anchor box on invalid position
(in render_gemx())

if debug_input_mode && sibling.x == 0 {
    f.render_widget(Paragraph::new("■"), Rect::new(1, y, 1, 1));
}
