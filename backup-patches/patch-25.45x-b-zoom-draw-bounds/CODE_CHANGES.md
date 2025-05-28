## Code Changes

- In `render_gemx()`:
  - Before every `render_widget`, check:
    `if draw_x >= area.width || draw_y >= area.height { continue; }`
- In `zoom_in()`:
  - Change zoom cap from 2.0 to 1.5 (or tunable max)
- In arrow render block:
  - Clamp mid and draw_sy before drawing
- Optional: add warning log when zoom tries to draw outside bounds

