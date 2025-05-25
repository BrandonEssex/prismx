## Code Changes

- In `add_child()`:
  - Always assign temp position:
```rust
child.x = parent.x;
child.y = parent.y + 1;
Do this regardless of auto_arrange value
In render_gemx():
if drawn_at.is_empty() {
    f.render_widget(Paragraph::new("⚠ Layout Error"), Rect::new(area.x + 2, area.y + 2, 30, 1));
}
Added logging:
eprintln!("⚠️ layout_nodes returned empty — layout failed");
Optional: visually mark nodes created with no Coords or invalid roles
