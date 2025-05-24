## Code Changes

- Adjust render offset in beamx.rs:
  let beam_x = area.right().saturating_sub(7);

- Align glyphs tightly:
  beam_x + 0: ⬊, ⬈
  beam_x + 1: ⥤
  beam_x + 3: X / pulse
  beam_x + 5: ⥢
  beam_x + 6: ⬋, ⬉

- Introduce:
  pub enum BeamXMode { Default, Zen, Triage, Spotlight, Settings }

- Add:
  impl From<BeamXMode> for BeamXStyle { ... }

- In each module:
  let mode = BeamXMode::from(app.mode);
  let beamx = BeamX { tick, enabled: true, style: BeamXStyle::from(mode) };
  beamx.render(f, area);
