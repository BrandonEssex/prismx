## Code Changes

### 1. After fallback promotion:
```rust
let fallback_x = 5 + (state.root_nodes.len() as i16 % 10) * SIBLING_SPACING_X;
let fallback_y = GEMX_HEADER_HEIGHT + 2;

if let Some(n) = state.nodes.get_mut(&id) {
    if n.x == 0 && n.y == 0 {
        n.x = fallback_x;
        n.y = fallback_y;
    }
}
2. Inject Coordinates to drawn_at Always:
drawn_at.insert(id, Coords { x: fallback_x, y: fallback_y });
node_roles.insert(id, LayoutRole::Root);
3. Clamp Rect Dimensions Before Rendering:
let safe_rect = Rect::new(
    x.min(area.width.saturating_sub(1)),
    y.min(area.height.saturating_sub(1)),
    width.min(area.width),
    height.min(area.height)
);
Use this for all emergency or fallback draws.

4. Skip Fallback If Already Injected:
if drawn_at.contains_key(&id) || fallback_promoted_this_session.contains(id) {
    continue;
}
