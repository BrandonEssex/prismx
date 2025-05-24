## Code Changes

- Create `src/beamx.rs`
  - Add fn `render_beam_logo(frame: &mut Frame<B>, area: Rect)`
  - Use Unicode symbols: `╱`, `╲`, `━`, `●`
  - Add colored spans using ratatui Style and `set_string()`

- In `render_gemx()`:
  - Call `render_beam_logo()` at top of layout
  - Show PrismX beam-X near top center of frame

- Optional:
  - Add `animate_beam_x()` to show fading frames or shimmer lines
  - Use `State.status_message` or tick counter to toggle frames
