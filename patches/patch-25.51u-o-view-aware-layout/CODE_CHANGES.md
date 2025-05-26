## Code Changes

- In `layout_nodes()` or layout system:
  - Read terminal frame size (frame.size())
  - Set a dynamic bounding box for layout
  - When placing children, prefer vertical stacking (Y-axis) until bounds are hit
  - Then shift to the right (X-axis) and continue stacking
- Add a basic cluster logic: if multiple children, stack downward within view first
- Gracefully degrade if canvas is larger than screen — fallback to current layout
- Log layout decisions if debug mode is enabled
