## 1. Enable Drag-and-Drop
- Add drag state tracking to selected node
- Allow repositioning via mouse/touch or keyboard nav
- Optional: Ctrl+D to toggle drag mode on/off

## 2. Snap-to-Grid Mode
- Add toggle in global state (`bool snap_to_grid`)
- On drag end, if snap enabled: round node x/y to nearest 20

## 3. Hotkeys
- Add Ctrl+G to toggle snap mode
- Optional: Display status overlay showing "Snap ON/OFF"

## 4. Persistence
- When layout is re-run, nodes with custom positions should retain them unless re-centered
