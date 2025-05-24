## Code Changes

- In `render_beamx()`:
  - Use this layout:
    ⬊    ⬋
     ⥤X⥢
    ⬈    ⬉

  - Animate `X` with a tick-based cycle:
      - tick % 24 → cycle through `·`, `◆`, `✦`, `X` (and back)

  - Keep beam glyphs static, but use `border_color` and `status_color` for differentiation
  - Logo aligned right with: `x = area.right().saturating_sub(7)`

- Optional:
  - Add `BeamXRenderMode::PrismPulseX` variant if needed for modular theme control
