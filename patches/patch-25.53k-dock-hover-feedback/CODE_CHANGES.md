## Code Changes

### 1. Track Dock Focus Index in AppState

```rust
pub dock_focus_index: Option<usize>;
2. Keyboard Navigation Handler
In input handling:

if key == Key::Up {
    state.dock_focus_index = Some(state.dock_focus_index.unwrap_or(0).saturating_sub(1));
}
if key == Key::Down {
    state.dock_focus_index = Some(state.dock_focus_index.unwrap_or(0) + 1);
}
Clamp at dock.len() if needed.

3. Highlight Focused Glyph in Dock Render
In dock rendering loop:

let is_focused = Some(i) == state.dock_focus_index;

let style = if is_focused {
    Style::default().fg(Color::LightCyan).add_modifier(Modifier::REVERSED)
} else {
    Style::default().fg(theme.border_color)
};

f.render_widget(Paragraph::new(format!("{} \\", glyph)).style(style), Rect::new(x, y + i as u16, 4, 1));
4. Optional: Show Hint in Status Bar
if let Some(index) = state.dock_focus_index {
    if let Some(fav) = dock_entries.get(index) {
        state.status_hint = Some(format!("↵ to run: {}", fav.command));
    }
}
