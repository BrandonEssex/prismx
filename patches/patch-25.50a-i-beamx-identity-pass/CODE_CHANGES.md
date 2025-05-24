## Code Changes

- In `render_settings.rs`:
  - Ensure Paragraphs start at x=0
  - Set alignment: `.alignment(Alignment::Left)`
  - Use explicit Rects if needed to override padding

- In `render_beamx()`:
  - Animate prism: tick % 3 → ("◆", "✦", "·")
  - Layout as:
      ⇘     ⇖
         ◆
      ⇖     ⇘
  - Use border_color vs status_color for beam styling
  - Prism always centered, logo rendered last
