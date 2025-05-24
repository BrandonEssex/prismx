## Code Changes

- Beam glyph layout:
╱  /    ← Line 1
 ✦      ← Line 2
/  ╲    ← Line 3

- Top-left and bottom-right beams:
- Connect directly into border
- Use `style.border_color`

- Top-right and bottom-left beams:
- Do NOT touch edges
- Use `style.status_color`

- Glyph `✦` or `◉` centered (x+2, y+1)
- Configurable as `style.center_glyph`

- In border rendering:
- Skip `┓` corner if beam crosses there

- Always draw beam/logo last (overlays underneath)

- Optional: stub for future bottom-left clip region

