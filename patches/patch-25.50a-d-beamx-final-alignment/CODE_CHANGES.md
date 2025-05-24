## Code Changes

- In `render_beam_logo(frame, area, style)`:
  - Set beam_x = area.width - 6
  - beam_y = area.y + 1
  - Render:
    - Line 0: beam_x + 0 `\`, beam_x + 3 `/`
    - Line 1: beam_x + 2 `*` or `·`
    - Line 2: beam_x + 0 `/`, beam_x + 3 `\`
  - Use style.beam_left, beam_right, prism color

- In every `render_*(...)` function:
  - Call `render_beam_logo()` and `render_full_border()` **last**

- Ensure `settings` and `help` screens still show logo

- Update color palette per mode:
  - gemx: cyan/white beams, white prism
  - zen: green beams, white prism
  - triage: white beams, cyan prism
  - settings: gray beams + white center

