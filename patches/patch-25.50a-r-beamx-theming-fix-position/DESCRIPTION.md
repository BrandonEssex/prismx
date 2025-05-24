# Patch 25.50a-r – BeamX Mode Theming + Alignment Fix

## Goals
- Fix layout regression from BeamX component:
  - BeamX was shifted too far left
  - Spacing between arrows broke the “X” shape
  - Logo no longer intersects upper-right border

- Restore correct layout (7 character wide):
  beam_x = area.right().saturating_sub(7)
  ⬊    ⬋
  ⥤ X ⥢
  ⬈    ⬉

- Add BeamXMode enum:
  - Default, Zen, Triage, Spotlight, Settings
  - Use mode to set: arrow glyphs, center pulse, color themes
