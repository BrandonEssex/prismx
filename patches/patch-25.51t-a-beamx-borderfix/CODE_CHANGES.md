## Code Changes

- In `render_full_border()`:
  - Adjust right/bottom border calculation to `area.width - 1` and `area.height - 1`
  - Prevent box draw calls from exceeding area rect

- In `BeamX::render()`:
  - Clamp shimmer trail to within `area.x..area.x + area.width`
  - Add gradient fade at shimmer edge using `BeamXAnimationMode::FadeOut`

- Visual Bugfix:
  - Strip bottom-right corner character overdraw:
```rust
if x == area.x + area.width - 1 && y == area.y + area.height - 1 {
    continue; // Skip corner glyph
}
Optional: Add diagnostics in debug mode:
if matches!(self.mode, BeamXMode::Debug) {
    eprintln!("Shimmer bounds: {} → {}", area.x, area.x + area.width);
}
