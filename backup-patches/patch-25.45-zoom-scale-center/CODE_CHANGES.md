## 1. State Management
- Add a `zoom_scale: f32` to global or viewport state (default: 1.0)
- Clamp between 0.5 and 2.0

## 2. Input
- Ctrl+= increases zoom (scale += 0.1)
- Ctrl+- decreases zoom (scale -= 0.1)
- Ctrl+0 resets scale to 1.0
- Add hotkeys to `hotkeys.rs`

## 3. View Logic
- Apply scale during canvas rendering (`render.rs`)
- Adjust pan/offset logic so selected node stays centered if `zoom_to_cursor = true`

## 4. Optional
- Add status overlay or debug print of current zoom level
