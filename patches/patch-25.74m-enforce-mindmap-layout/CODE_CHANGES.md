## Code Changes

- When inserting siblings, each node must appear at an incremented x offset
- When inserting children, each node must appear beneath parent with unique y offset
- Print `LAYOUT_OK: <x>,<y>` for each new node in debug
- Absolutely no node should appear at `(0,0)` unless it's a free-floating root
