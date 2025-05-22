## 1. Modify render.rs

Add directional arrow rendering in link drawing logic:
- If Node A ➝ Node B, show → marker
- Arrow should follow orientation (left ➝ right for horizontal layout)

## 2. Modify mac_fallback.rs

Add macOS fallback keybinding:
- Cmd+Left → ScrollLeft
- Cmd+Right → ScrollRight
