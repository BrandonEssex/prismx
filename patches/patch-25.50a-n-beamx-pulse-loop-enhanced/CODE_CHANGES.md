## Code Changes

- In `render_beamx.rs`:
  - Replace center pulse logic with:
      · → ◆ → ✦ → x → X → x → ✦ → ◆ → ·
  - Use `tick % 36` for slower pacing
  - Center glyph only changes — no change to arrow layout
