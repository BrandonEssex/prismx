## 1. Shortcut Enum
- enum Shortcut { ZoomIn, ZoomOut, ResetZoom, ToggleDebugInput, ... }

## 2. Key Matching
- match_shortcut(code, modifiers) → Shortcut
- All keys must include a modifier (Alt, Ctrl, or Alt+Shift)

## 3. Input Debug Toggle
- Add field: debug_input_mode: bool in AppState
- Toggle with Ctrl+Shift+D
- On event, write key + modifier to state.status_message

## 4. Status Bar Integration
- Render state.status_message at bottom of the screen
- Clear or auto-expire after timeout (optional)
