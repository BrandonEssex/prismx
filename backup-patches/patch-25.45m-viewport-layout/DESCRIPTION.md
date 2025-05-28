# Patch 25.45m – Viewport-Aware Layout + Spawn Offset Fix

## Goals
- Prevent nodes from overlapping the GemX label or each other at the top
- Respect terminal screen size when choosing layout origin
- Apply a header-aware padding (`GEMX_HEADER_HEIGHT = 2`)
- Prevent free nodes and root siblings from starting at (0,0)
- If layout would exceed terminal width, dynamically compress sibling spacing or wrap
- Create a "legibility-first" layout mode for constrained screens

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 1
- GEMX_HEADER_HEIGHT = 2

