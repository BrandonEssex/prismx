## Code Changes

- In `render_beamx()`:
  - Layout:
      ⬊     ⬋
         ⊙
      ⬈     ⬉

  - Animate center:
    let center = match tick % 48 {
        0..=5   => "·",
        6..=11  => "✦",
        12..=23 => "❖",
        24..=29 => "✦",
        30..=35 => "X",
        36..=41 => "✦",
        42..=47 => "❖",
        _       => "·",
    };

  - Replace ⊙ with `center` string dynamically

  - Use:
    - Diagonals → `border_color`
    - Center → `prism_color`
  - Align top-right: `x = area.right().saturating_sub(5)`
