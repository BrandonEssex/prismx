## Code Changes

- Reviewed and refactored:
  - `scroll_x` and `scroll_y` scaling math in `render_gemx`
  - Zoom target anchor points
  - Scroll clamping logic at high zoom levels

- Retested zoom-to-cursor from:
  - `patch-25.45x-zoom-to-cursor-sync`
  - `patch-25.45f-void-layout`
