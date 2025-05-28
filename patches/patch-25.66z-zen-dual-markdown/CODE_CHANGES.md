## Code Changes

- Add new layout: `ZenLayoutMode::Dual`
- Render input + scroll side-by-side
- Extend `highlight_tags_line` to handle:
  - `*italic*` → Dim
  - `**bold**` → Bold
  - `` `code` `` → Yellow
