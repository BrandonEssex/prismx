## Code Changes

- Entry beam glyphs (`⇙`) animate intensity:
```rust
match tick % 8 {
  0..=1 => "⇙",            // Dim
  2..=3 => "⇙", bold,       // Mid
  4..=5 => "⇙", bright,     // High
  _     => "⇙", dim         // Fade
}
Exit beam glyphs (⬉, ⬊) animate outward:
match tick % 8 {
  0..=1 => "⬉", dim,
  2..=3 => "⬉", bold,
  4..=5 => "⬉", bright,
  _     => "⬉", normal
}
Border edge (top-left, bottom-right) pulses with border_color.bright() when exit glyph hits tick 4–5
Removed all directional flip animations (e.g., ⬇, ⟰, etc.)
Beam direction is fixed, visual intensity only
