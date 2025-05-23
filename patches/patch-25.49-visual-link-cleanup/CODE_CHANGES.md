## 1. Link Glyphs
- Updated elbow glyphs to use `╰─` for last child and `↳` for mid-branch

## 2. Drag Highlight
- `render_gemx` now detects `state.dragging` and renders the node with a reversed cyan style

## 3. Detach on Drag
- `drag_update` checks distance from parent and calls `detach_node` when threshold exceeded
