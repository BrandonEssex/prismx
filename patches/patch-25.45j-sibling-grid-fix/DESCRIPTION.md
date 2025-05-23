# Patch 25.45j – Fix Sibling Spacing & Free Node Collision

## Goals
- Respect SIBLING_SPACING_X when auto_arrange is disabled
- Prevent free nodes from overlapping (e.g., Node A and B)
- Align initial spawn logic with defined constants
- Maintain signed coordinate system
- Do not override manually placed positions

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 1
- FREE_GRID_COLUMNS = 4

