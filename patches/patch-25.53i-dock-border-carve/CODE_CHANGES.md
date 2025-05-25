## Code Changes

### 1. Vertical Layout Stack w/ Border Break

```rust
let base_y = area.height.saturating_sub(favorites.len() as u16 + 2);
f.render_widget(Paragraph::new("\\__").style(style), Rect::new(0, base_y - 1, 3, 1));

for (i, glyph) in favorites.iter().enumerate() {
    let y = base_y + i as u16;
    let styled = format!("{} |", glyph);
    f.render_widget(Paragraph::new(styled).style(style), Rect::new(0, y, 5, 1));
}
2. Finish the Bottom Wall
After glyphs finish, continue horizontal border from their base:
let h_y = base_y + favorites.len() as u16;
f.render_widget(Paragraph::new("  |______________").style(style), Rect::new(0, h_y, area.width, 1));
→ This reconnects the wall

3. Toggle Layout
Expose: /dock_carve=true, /dock_layout=vertical, /dock_limit=5
4. Color from Theme
Style::default().fg(state.theme.border_color)
