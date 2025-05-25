## Code Changes

### 1. Add command preview engine

Inside Spotlight render:
```rust
let preview = match spotlight_input.as_str() {
    "/triage" => "→ Switches to Triage panel",
    "/zen" => "→ Opens Zen mode",
    "/settings" => "→ Opens settings",
    _ => "⚠ Unknown command",
};
Render preview:

f.render_widget(Paragraph::new(preview), Rect::new(x, y + 2, width, 1));
Use red/yellow for invalid commands using .fg(Color::Red) etc.
