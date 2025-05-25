## Code Changes

### 1. Define Clickable Bounds for Each Glyph

Store positions:
```rust
struct FavoriteEntry {
    label: &'static str,
    mode: Mode,
    bounds: Rect, // x, y, width, height
}
During render, track:

let mut dock_entries = vec![];
for (i, &(glyph, mode)) in default_favorites.iter().enumerate() {
    let y = base_y + i as u16;
    let bounds = Rect::new(1, y, 4, 1);
    dock_entries.push(FavoriteEntry { label: glyph, mode, bounds });
    f.render_widget(Paragraph::new(format!(" {}\\", glyph)).style(style), bounds);
}
2. Add Mouse Click Handling (if backend supports)
In mouse handler:

if let Some(click) = state.last_mouse_click {
    for entry in &dock_entries {
        if entry.bounds.contains(click.position) {
            state.mode = entry.mode;
            state.spotlight = false;
            break;
        }
    }
}
3. Plugin-Extendable Dock (future)
Leave open:

state.plugin_favorites: Vec<FavoriteEntry>;
Each plugin could register glyphs and handlers in future patch.

