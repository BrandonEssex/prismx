## Code Changes

- For every node with a parent:
  - Calculate midpoint between `child.position` and `parent.position`
  - Draw connector line: use `│`, `─`, `└` in Ratatui `Paragraph` or `Span`
- Connector color uses beam style or dim gray
- No longer requires `draw_beam()`
