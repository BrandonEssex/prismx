# Patch 25.45d – Tree Layout Restore + Vertical Stack

## Goals
- Restore clean visual hierarchy between parent and children
- Align child nodes directly below parents (tight vertical tree)
- Remove excessive horizontal spacing in branch layout
- Maintain zoom scaling without affecting structure
- Support slight stagger or indent (optional, configurable later)

## Logic
- In layout_nodes or render:
  - child.x = parent.x + (index - mid) * SIBLING_SPACING_X
  - child.y = parent.y + CHILD_SPACING_Y

