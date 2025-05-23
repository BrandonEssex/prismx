## 1. Shortcut Mapping
- Shortcut::ZoomIn → Ctrl+Alt+Z
- Shortcut::ZoomOut → Ctrl+Alt+X
- Shortcut::ZoomReset → Ctrl+Alt+0

## 2. State Logic
- Clamp zoom_scale between 0.5 and 2.0
- Default = 1.0

## 3. Render Logic
- Multiply all node/link coordinates by zoom_scale
- Adjust scroll offsets and clamp if needed

## 4. Optional Debug
- Display current zoom scale in status bar

