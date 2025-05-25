## Code Changes

- Beam grid now includes:

⬉ ⇙
⬉ ⇙
✦
⬊
⬊


- Entry Arrows:
  - Two `⇙` glyphs stacked at top-right
  - Use intensity animation only (not direction)

- Exit Arrows:
  - Two `⬉` top-left, two `⬊` bottom-right
  - Always face outward from `X`
  - Animate as: dim → bold → bright → fade

- Border Sync:
  - When exit glyph hits "bright" phase
    - Pulse `border_color.bright()` on matching edge

- Center X:
  - Shimmering pulse cycle of `✦ → ◆ → x → X → ✦`
  - Uses `prism_color`, but cycles hue slightly (optional)
