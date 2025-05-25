## Code Changes

### 1. Add Configuration Fields
```rust
pub favorite_dock_limit: usize = 3;
pub favorite_dock_layout: DockLayout = DockLayout::Vertical;

pub enum DockLayout {
    Vertical,
    Horizontal,
}
2. Define Favorites
let all_favorites = vec!["⚙️", "📬", "💭", "🧘", "🔍"];
let favorites = &all_favorites[..state.favorite_dock_limit];
3. Draw Horizontally or Vertically
Horizontal:

let line = favorites.join("  ");
f.render_widget(Paragraph::new(line).style(style), Rect::new(x, y, width, 1));
Vertical:

for (i, glyph) in favorites.iter().enumerate() {
    f.render_widget(Paragraph::new(*glyph).style(style), Rect::new(x, y + i as u16, 4, 1));
}
4. Add Border Block
let height = if horizontal { 3 } else { favorites.len() as u16 + 2 };
let width = if horizontal { favorites.len() as u16 * 3 + 2 } else { 6 };
let border = Block::default().borders(Borders::ALL).style(Style::default().fg(theme.border_color));
f.render_widget(border, Rect::new(x - 1, y - 1, width, height));
5. Toggle Command
Add to /settings or Spotlight:

/dock_layout=horizontal
/dock_limit=5
