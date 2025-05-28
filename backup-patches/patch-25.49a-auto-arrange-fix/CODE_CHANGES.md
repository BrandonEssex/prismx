## 1. State Persistence
- Track last known manual position per node (state or node struct)

## 2. Grid-Based Layout
- Add layout_fallback_grid() to position nodes on a non-overlapping grid
- Apply only when auto_arrange == false and no saved position

## 3. Spacing Constants
- NODE_SPACING_X: default 20
- NODE_SPACING_Y: default 5
- Place nodes in rows/cols if no previous position

## 4. Future-Ready
- Keep layout logic modular so color-based branch separation (future) can reuse spacing engine

