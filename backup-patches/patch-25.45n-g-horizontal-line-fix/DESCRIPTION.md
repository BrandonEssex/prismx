# Patch 25.45n-g – Horizontal Sibling Line Fix

## Goals
- Align all sibling nodes horizontally
- Prevent vertical stacking of same-level children
- Ensure tree fans out horizontally, like a true mindmap

## Fix
- Move `child_y = y + 1` outside the loop in layout_recursive_safe
- All children of a node should share the same vertical level (Y)

