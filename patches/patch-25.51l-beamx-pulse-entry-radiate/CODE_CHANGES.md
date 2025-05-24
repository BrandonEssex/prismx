## Code Changes

- Added `BeamXAnimationMode::PulseEntryRadiate`

- New frame-by-frame logic in `render()`:
  - `⇙` appears at tick 0–2
  - `✦ → X` pulse on tick 3–7
  - Border beams (`⬉`, `⬊`) appear tick 8–10
  - Beams pulse tick 11–14
  - Entry fades at tick 15–19
  - Center `X` shimmer resumes at tick 20+

- Each phase defines:
  - Which glyphs to render
  - What color or style they use

- Future animations can follow same tick-frame logic
