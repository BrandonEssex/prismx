## Code Changes

- Track occupied grid positions with a `HashSet<(x, y)>`
- When spawning new free node, scan for nearest unoccupied spot (spiral or Y-stack)
- Use `frame.size()` to avoid off-screen placement
- Ensure spacing respects visible UI and borders
