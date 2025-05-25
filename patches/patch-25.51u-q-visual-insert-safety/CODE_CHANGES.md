## Code Changes

### 1. In `add_sibling()` and `add_child()`

After assigning coordinates:
```rust
if state.debug_input_mode {
    eprintln!(
        "[Node {}] label=\"{}\", parent={:?}, x={}, y={}",
        new_id,
        node.label,
        node.parent,
        node.x,
        node.y
    );
}
If !auto_arrange and node has no parent, assign:

node.x = (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
node.y = GEMX_HEADER_HEIGHT + 2;
2. In render_gemx(): Debug Tree Output
After layout completes:

if state.debug_input_mode {
    eprintln!("Render Tree:");
    for (&id, coords) in &drawn_at {
        let label = &state.nodes[&id].label;
        let role = node_roles.get(&id).unwrap_or(&LayoutRole::Free);
        eprintln!("Node {} → (x: {}, y: {}) | {:?} | {}", id, coords.x, coords.y, role, label);
    }
}
3. (Optional) Render Visual Debug Border
If you use a widget-based drawing system:

if state.debug_input_mode {
    let border_rect = Rect::new(x, y, label_width, 1);
    f.render_widget(Clear, border_rect); // or draw styled frame/pipe for debugging
}
