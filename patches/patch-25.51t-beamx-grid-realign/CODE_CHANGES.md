## Code Changes

### Final Layout:

⬉ ⇙
⬉ ⇙
✦
⬊
⬊


### Positioning:
```rust
// Row 0
render_glyph(f, x + 0,  y + 0, "⬉", border_color);
render_glyph(f, x + 11, y + 0, "⇙", status_color);

// Row 1
render_glyph(f, x + 3,  y + 1, "⬉", border_color);
render_glyph(f, x + 9,  y + 1, "⇙", status_color);

// Row 2 (center)
render_glyph(f, x + 6,  y + 2, center_glyph, prism_color);

// Row 3
render_glyph(f, x + 9,  y + 3, "⬊", border_color);

// Row 4
render_glyph(f, x + 11, y + 4, "⬊", border_color);
Adjusted all spacing to be fixed + symmetrical
Optional: animate ⇙, ⬉, and ⬊ by tick phase
Border pulse still syncs to tick 2–3 per exit beam
