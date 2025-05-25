## Code Changes

### 1. Apply Document Padding (25% Left/Right)

In `render_zen()`:
```rust
let padding = area.width / 4;
let usable_width = area.width - (padding * 2);
let input_rect = Rect::new(area.x + padding, area.y + area.height / 2, usable_width, 1);
2. Improve Input and Caret Visibility
let style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
let caret = if frame_tick % 2 == 0 { "|" } else { " " };
let input_line = format!("> {}{}", state.zen_input, caret);
3. Fade Previous Lines But Keep Them Within Bounds
for (i, line) in state.zen_lines.iter().rev().enumerate() {
    let y = area.height / 2 - (i as u16) - 1;
    let style = if i == 0 {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    f.render_widget(Paragraph::new(line).style(style), Rect::new(area.x + padding, y, usable_width, 1));
}
4. (Optional) Render Hidden Sidebars for Later Use
Left sidebar (Recent Files):

let left_label = Paragraph::new("Recent").style(Style::default().fg(Color::DarkGray));
f.render_widget(left_label, Rect::new(area.x + 1, area.y + 2, padding - 2, 1));
Right sidebar (Font Settings):

let right_label = Paragraph::new("Aa").style(Style::default().fg(Color::DarkGray));
f.render_widget(right_label, Rect::new(area.right() - padding + 1, area.y + 2, 3, 1));
