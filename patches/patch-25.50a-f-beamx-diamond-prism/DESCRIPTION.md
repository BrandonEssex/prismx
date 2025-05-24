# Patch 25.50a-f – Final BeamX Alignment with Diamond Prism

## Goals
- Draw symmetric X-beam logo in top-right corner
- Use `◆` as prism center glyph (fallback: `✦`, `·`)
- Ensure beams are aligned correctly:
  - `\` and `\` (top-left and bottom-right) control border color
  - `/` and `/` (top-right and bottom-left) are status beams
- Remove corner glyphs that intersect beams
- Logo must appear in all screens, always top-right, always last
