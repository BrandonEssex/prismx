# Patch 25.45d – Tree Layout Restore + Vertical Stack

## Goals
- Restore clean visual hierarchy between parent and children
- Align child nodes directly below parents (tight vertical tree)
- Remove excessive horizontal spacing in branch layout
- Maintain zoom scaling without affecting structure
- Support slight stagger or indent (optional, configurable later)

## Logic
- In layout_nodes or render:
  - child.x = parent.x
  - child.y = parent.y + BASE_CHILD_SPACING_Y
- Use minimal X offset if siblings would otherwise overlap

