## Code Changes

- In `complete_drag`, if dragged node is dropped on its own child, detect and abort cycle
- Convert node into free root: remove parent and assign manual_coords
- If dropped on canvas (no valid target), fallback to manual_coords placement
- Add safeguard to prevent illegal graph states (e.g. Node A becomes child of Node A)
