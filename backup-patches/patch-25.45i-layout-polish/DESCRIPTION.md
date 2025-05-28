# Patch 25.45i – Layout Polish + Fallback Grid + Drag Fix

## Goals
- Improve child spacing (closer to parent)
- Fix sibling overlap with correct horizontal offset
- When auto-arrange is off, assign nodes to a fallback grid layout instead of dumping them
- Fix drag behavior to respect scroll and signed coordinates

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 1
- FREE_GRID_COLUMNS = 4

