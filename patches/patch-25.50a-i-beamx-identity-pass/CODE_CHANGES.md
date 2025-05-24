## Code Changes

- In `render_settings.rs`:
  - Ensure all header Paragraphs render from x=0
  - Manually set Rect if needed: `Rect::new(0, y, width, 1)`
  - Add `.alignment(Alignment::Left)` if any Paragraph is misaligned

- In `render_beamx.rs`:
  - Animate prism glyph using `tick % 3`: (`◆`, `✦`, `·`)
  - Use symmetric glyphs: `⇘` top-left, `⇖` top-right, etc.
  - Refactor style application to reflect “beam energy” vs “frame border”

- Optional:
  - Add `BeamPulseMode::Glow` or enum variant if needed
