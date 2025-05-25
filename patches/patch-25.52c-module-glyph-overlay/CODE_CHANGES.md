## Code Changes

### 1. Add a glyph mapping for modules:

```rust
fn module_icon(mode: Mode) -> &'static str {
    match mode {
        Mode::GemX => "💭",
        Mode::Zen => "✍️",
        Mode::Triage => "🧭",
        Mode::Spotlight => "🔍",
        _ => "❓",
    }
}
2. Render icon overlay at top-right or top-left:
Inside render_ui_frame() or equivalent:

let glyph = module_icon(state.mode);
let style = Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD);
f.render_widget(Paragraph::new(glyph).style(style), Rect::new(1, 0, 2, 1));
Or position to top-right:

let w = area.width;
f.render_widget(Paragraph::new(glyph).style(style), Rect::new(w - 3, 0, 2, 1));
Use minimal spacing and avoid border collision
Consider drawing it above the UI border layer if possible
3. Optional:
Animate icon briefly on module switch (blink or invert)
Add config option to disable icons for minimal mode
