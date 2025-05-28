## Code Changes

- In tui/mod.rs: separate `draw_nodes()`, `draw_links()`, `draw_ephemeral()`
- Store transient visuals (drag preview, arrows, highlights) in AppState or a render overlay map
- Render layers in order: layout ➝ links ➝ ephemeral
- Ensure links and temp highlights do not affect layout.rs or node position logic
