## Code Changes

### 1. Add Favorites Stack Rendering in `render_ui_frame()`

Default icons:
```rust
let mut dock = vec!["⚙️", "📬", "💭"]; // Settings, Triage, GemX
Swap icons based on mode/state:

if state.mode == Mode::GemX {
    dock[2] = "💬"; // GemX active
}
if state.triage_open {
    dock[1] = "📫"; // Triage active
}
2. Render as vertical stack with trailing slashes
|__
 ⚙️\
 📬 \
 💬  \____
let x = 1;
let y = area.height.saturating_sub(6);
let style = Style::default().fg(Color::Cyan);

let lines = vec![
    "|__",
    &format!(" {}\\", dock[0]),
    &format!(" {} \\", dock[1]),
    &format!(" {}   \\____", dock[2]),
];

for (i, line) in lines.iter().enumerate() {
    f.render_widget(Paragraph::new(*line).style(style), Rect::new(x, y + i as u16, 12, 1));
}
3. Future Expansion:
Favorites can grow to 4–5 entries via config
Plugins may populate glyph slots
Highlights or animations could flash active one
