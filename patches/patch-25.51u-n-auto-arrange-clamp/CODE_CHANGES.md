## Code Changes

- Update `layout_nodes()` to apply Y-offset fallback if new node position is `(0,0)` or collides with another
- Add spacing increment (e.g. +2 rows per fallback) for visual clarity
- Prevent duplicate X/Y placement when auto-arranging children
- Respect layout lock if root is undefined
