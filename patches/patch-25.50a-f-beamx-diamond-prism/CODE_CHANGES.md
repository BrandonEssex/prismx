## Code Changes

- In `render_beam_logo()`:
  - Line 1: draw `\` at x+0, `/` at x+3
  - Line 2: draw `◆` (fallback to `✦`, `·`) at x+2
  - Line 3: draw `/` at x+0, `\` at x+3
  - Align to `area.width - 6`, `area.y + 1`
  - Top-left & bottom-right = `border_color`
  - Top-right & bottom-left = `status_color`
  - Prism = `prism_color`

- In border draw:
  - Skip drawing `┓` if BeamX is active

- In `BeamStyle`:
  - Add `center_glyph: &'static str` (use `"◆"`)

