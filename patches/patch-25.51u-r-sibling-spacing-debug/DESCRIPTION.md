# Patch 25.51u-r – Sibling Layout Spacing + Insert Debug Confirm

## Goals
- Ensure sibling nodes are assigned valid non-overlapping (x, y) positions
- Fix cases where all siblings are placed at (0, y)
- Add debug prints to verify exact inserted coordinates
- Clamp any sibling at x=0 and nudge to right if overlapping
