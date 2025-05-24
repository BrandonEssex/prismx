## Code Changes

- In `render_full_border*()`:
  - Skip drawing upper-right corner border if BeamX is enabled
  - Detect `x = area.right() - 2`, `y = area.top()`
  - Condition: `if beamx_enabled && within_beam_area(...) { continue; }`

- BeamX logo position remains unchanged
