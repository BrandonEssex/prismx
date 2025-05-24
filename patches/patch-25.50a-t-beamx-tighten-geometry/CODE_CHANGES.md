## Code Changes

- In `beamx.rs`, change:
  let x = area.right().saturating_sub(7);
  ⟶
  let x = area.right().saturating_sub(8);

- Adjust all beam glyphs 1 column to the right:
  - x+1: ⬊ / ⬈
  - x+3: ⥤
  - x+4: prism pulse (⊙)
  - x+5: ⥢
  - x+7: ⬋ / ⬉

- Ensure `render_full_border` skip logic uses updated x boundary
