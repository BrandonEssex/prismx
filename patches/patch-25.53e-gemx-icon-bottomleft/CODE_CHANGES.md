## Code Changes

- In `render_ui_frame()`:

```rust
let icon = match state.mode {
    Mode::GemX => "💭",
    Mode::Zen => "🧘",
    Mode::Triage => "🧭",
    Mode::Spotlight => "🔍",
    Mode::Settings => "⚙️",
    _ => "❓",
};

let x = area.left() + 1;
let y = area.bottom().saturating_sub(3);

f.render_widget(Paragraph::new(format!("  \\\n   \\\n     {}\\____", icon)), Rect::new(x, y, 10, 3));
Stylize with .fg(Color::Cyan) or dim white
Optional: Add plugin hook for displaying badges beside it
