# Patch 25.51q – Layout Safety + Child Injection Fix

## Goals
- Prevent crashes when adding a child node in auto-arrange mode
- Ensure child nodes are always assigned initial positions
- Add fallback rendering if layout fails
- Show a visible warning if no nodes are drawn
