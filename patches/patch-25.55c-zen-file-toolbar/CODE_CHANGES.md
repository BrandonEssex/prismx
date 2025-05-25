## Code Changes

### 1. Track Toolbar Toggle State

```rust
pub zen_toolbar_open: bool,
pub zen_recent_files: Vec<String>,
pub zen_toolbar_index: usize,
2. Toggle Command (Spotlight or hotkey)
if input == "/toolbar" {
    state.zen_toolbar_open = !state.zen_toolbar_open;
}
Or Ctrl+T in event handler.

3. Render Toolbar in Left Padding Area
let x = area.x + 1;
let y_base = area.y + 2;

let entries = vec!["+ New", "Open", "Save"];
entries.extend(state.zen_recent_files.iter().map(|s| s.as_str()));

for (i, entry) in entries.iter().enumerate() {
    let style = if i == state.zen_toolbar_index {
        Style::default().fg(Color::White).add_modifier(Modifier::REVERSED)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    f.render_widget(Paragraph::new(*entry).style(style), Rect::new(x, y_base + i as u16, 16, 1));
}
4. Input Handling (↑ ↓ Enter)
if key == Key::Up {
    state.zen_toolbar_index = state.zen_toolbar_index.saturating_sub(1);
}
if key == Key::Down {
    state.zen_toolbar_index += 1;
}
if key == Key::Enter {
    match state.zen_toolbar_index {
        0 => state.new_zen_file(),
        1 => state.open_zen_file(),
        2 => state.save_zen_file(),
        _ => state.open_recent(index),
    }
}
