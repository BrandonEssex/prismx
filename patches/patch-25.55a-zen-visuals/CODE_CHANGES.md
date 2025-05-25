## Code Changes

### 1. Center the Input Line

In `render_zen()`:
```rust
let input_line = format!("> {}", state.zen_input);
let x_offset = (area.width.saturating_sub(input_line.len() as u16)) / 2;
let centered = Rect::new(area.x + x_offset, area.y + area.height / 2, area.width, 1);
Render input using Paragraph::new(input_line) to centered

2. Add Animated Caret (|) Toggle
Track frame_tick and toggle cursor:

let caret = if frame_tick % 2 == 0 { "|" } else { " " };
let input_line = format!("> {}{}", state.zen_input, caret);
3. Background Pulse Effect
Apply light-dark gradient over time:

let bg_color = match frame_tick % 20 {
    0..=9 => Color::Rgb(18, 18, 18),
    _ => Color::Rgb(12, 12, 12),
};

let block = Block::default().style(Style::default().bg(bg_color));
4. Fade Old Lines (Optional)
Apply dimming to older entries:

for (i, line) in state.zen_lines.iter().rev().enumerate() {
    let style = if i == 0 {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    f.render_widget(Paragraph::new(line).style(style), target_rect);
}
