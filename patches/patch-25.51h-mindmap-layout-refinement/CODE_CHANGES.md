## Code Changes

- In `layout_nodes()`:
  - Child nodes now inherit `x` from parent
  - Use `y = parent.y + CHILD_SPACING_Y`

- In `auto_arrange`/`pack_region` logic:
  - Reduce spacing at low zoom scales (e.g. `zoom < 0.7`)
  - Dynamically set `BASE_SPACING_X/Y` to tighter values

- Adjusted glyph alignment:
  - Elbows (`├─`, `└─`) line up better at small grid sizes
  - Parent glyphs lock vertically with child groups

- Optional: Add `layout_debug=true` toggle to `AppState` to visualize layout zones
