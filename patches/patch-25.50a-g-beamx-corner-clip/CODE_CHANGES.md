## Beam Fixes
- In `render_beam_logo()`:
  - Position: `x = area.width - 6`, `y = area.y`
  - Beam layout:
    - Line 0: `\` and `/`
    - Line 1: center glyph `◆` at x+2
    - Line 2: `/` and `\`
- Do NOT render:
  - `┓` at top-right if BeamX is active
  - Right-side `┃` and top `━` under beam glyphs
- Use `style.border_color` and `status_color` appropriately

## Shortcut Fix
- In key event handler:
  - Detect `Ctrl+.` and set `state.mode = "settings"`

