## 1. Input Fix
- Ensure Alt+key is correctly registered
- Add debug output when key triggers zoom_scale changes

## 2. Zoom Logic
- Zoom In: zoom_scale += 0.1 (max 2.0)
- Zoom Out: zoom_scale -= 0.1 (min 0.5)
- Reset Zoom: zoom_scale = 1.0, center selected node

## 3. Visual Fix
- Apply zoom_scale in render.rs
- Pan view to center selected node on reset
- Clamp visible range to prevent node overflow

## 4. Debugging
- Optional: overlay or print current zoom_scale on screen
