
## 1. Link Glyphs
- Updated elbow glyphs to use `╰─` for last child and `↳` for mid-branch

## 2. Drag Highlight
- `render_gemx` now detects `state.dragging` and renders the node with a reversed cyan style

## 3. Detach on Drag
- `drag_update` checks distance from parent and calls `detach_node` when threshold exceeded
------
## 1. Link Rendering
- Update render.rs to use directional arrows
- Support vertical and horizontal orientations

## 2. Drag Indicators
- Show special glyph or highlight while dragging
- Optional: show hover preview or alignment line

## 3. Child Detach Logic
- In end_drag, check distance to parent
- If "too far", run detach_from_parent()
- Remove node from parent.children, clear .parent

## 4. Tests
- Move node away → confirm parent removed
- Render layout → confirm arrow characters shown
