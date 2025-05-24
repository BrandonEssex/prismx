## Code Changes

- In `render_gemx.rs`:
  - If `state.auto_arrange` is true:
    - Render `[A] Auto-Arrange` in top-left corner at `(x + 1, y + 1)`
    - Use dimmed gray color (`Style::default().fg(Color::DarkGray)`)

- Added layout fail-safe:
```rust
if drawn_at.is_empty() {
  let msg = Paragraph::new("⚠️ Auto-Arrange failed");
  f.render_widget(msg, Rect::new(area.x + 2, area.y + 3, 30, 1));
}
Optional: Add state.layout_debug = true toggle in future patch
