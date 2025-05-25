## Code Changes

- In `render_full_border()` or wherever outer box borders are drawn:
  - Prevent rendering the **right vertical line**:
```rust
if x == area.x + area.width - 1 {
    continue; // skip right edge
}
If using Block::default().borders(Borders::ALL):
Switch to Borders::LEFT | Borders::TOP | Borders::BOTTOM or custom draw
Optional: Replace arrow glyphs (↗, ↘) with absolute-positioned BeamX animation anchors
Add a debug flag to toggle between full vs trimmed border if needed
