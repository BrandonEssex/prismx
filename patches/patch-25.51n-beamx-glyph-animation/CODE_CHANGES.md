## Code Changes

- `tick % 24` now animates:
  - `⇙ → ⟱ → ⬊ → ⬇` for entry
  - `⬉ → ⭱ → ⟰ → ⬆` for exit beams
  - `· → ✦ → ◆ → ✷ → ✸ → x → X` for center glyph

- Entry beam uses `status_color`
- Exit beams use `border_color`
- Center glyph uses `prism_color`

- Style update per frame:
```rust
let frame = tick % 24;
let entry_glyph = match frame { ... };
let exit_glyph = match frame { ... };
