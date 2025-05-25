## Code Changes

### 1. Add Entrance Beam Arrow Effect

In `render_spotlight()` or `draw_spotlight_panel()`:

```rust
let arrow_tail = vec!["→", "→", "→"];
for (i, arrow) in arrow_tail.iter().enumerate() {
    let style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::DIM);
    f.render_widget(Paragraph::new(*arrow).style(style), Rect::new(x - i as u16 * 2, y, 2, 1));
}
You can animate this using a frame counter:

let offset = (frame_index % 3) as u16;
let tail = ["  ", "→ ", "→→", "→→→"];
2. Optional: Add a glow/pulse animation around the panel
If supported in your renderer:

Pulse color or change border thickness during activation
Highlight or shimmer border for 1–2 frames on open
3. Animate only on open (toggle):
When spotlight_opened == true && last_frame_was_closed == true
Store prev_spotlight_state and diff it to detect opening event.

4. Fallback (for limited terminals)
If animations aren't supported, fade the border in over 2 frames
Use inverse style + dim modifier instead of full motion
