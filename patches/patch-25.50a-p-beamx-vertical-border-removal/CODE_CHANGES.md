## Code Changes

- In vertical border draw loop (likely render_full_borderx()):
  - Skip drawing `│` at the top-right when:
    - x == area.right() - 1
    - y == beam_y1 or y == beam_y2 (rows 2 and 3 of logo)
  - BeamX logo remains unchanged
  - No changes to tick logic, glyphs, or center pulse
