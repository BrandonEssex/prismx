## 1. Update Node Render Logic
- Use scaled spacing: x = node.x * spacing_x * zoom_scale
- Use zoomed label width for offset calculations
- Ensure space between nodes ≥ MIN_NODE_GAP * zoom_scale

## 2. Apply in render.rs
- Avoid overlapping neighbors when rendering
- Round to whole pixels when drawing

## 3. Constants
- BASE_SPACING_X/Y, MIN_NODE_GAP in layout.rs or render.rs
- Allow easy future tuning or user customization

