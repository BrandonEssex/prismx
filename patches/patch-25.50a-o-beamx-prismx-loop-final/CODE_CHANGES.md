## Code Changes

- In `render_beamx.rs`:
  - Replace pulse logic with:
    · → ✦ → ◆ → ✦ → · → x → X → x → · → ✦ → ◆ → ✦
  - Use `tick % 12` cycle
  - Only center glyph affected
  - Keep beam layout, spacing, and colors unchanged
