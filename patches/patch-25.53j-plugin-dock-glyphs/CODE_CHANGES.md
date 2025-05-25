## Code Changes

### 1. Add Plugin Glyph Registry

In `AppState`:
```rust
pub plugin_favorites: Vec<FavoriteEntry>;

pub struct FavoriteEntry {
    pub icon: &'static str,
    pub command: &'static str,
}
2. Expose Plugin Registration
pub fn register_plugin_favorite(state: &mut AppState, icon: &'static str, command: &'static str) {
    if state.plugin_favorites.len() < 5 {
        state.plugin_favorites.push(FavoriteEntry { icon, command });
    }
}
3. Merge Default + Plugin Dock Entries
In dock rendering:

let all = default_favorites
    .iter()
    .map(|&(icon, cmd)| FavoriteEntry { icon, command: cmd })
    .chain(state.plugin_favorites.iter().cloned());

for (i, entry) in all.take(5).enumerate() {
    f.render_widget(
        Paragraph::new(format!("{} \\", entry.icon)).style(style),
        Rect::new(x, y + i as u16, 4, 1),
    );
}
