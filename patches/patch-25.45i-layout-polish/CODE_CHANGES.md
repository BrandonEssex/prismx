## 1. Fix Child Spacing
- CHILD_SPACING_Y = 1
- child.y = parent.y + CHILD_SPACING_Y

## 2. Fix Sibling Offset
- child.x = parent.x + (i - mid) * SIBLING_SPACING_X

## 3. Fallback Layout (Manual Mode)
- For free nodes or auto_arrange=false:
  node.x = (index % FREE_GRID_COLUMNS) * 6
  node.y = (index / FREE_GRID_COLUMNS) * 2

## 4. Drag Fix
- Ensure drag handles i16 and scroll_x/y offsets correctly

