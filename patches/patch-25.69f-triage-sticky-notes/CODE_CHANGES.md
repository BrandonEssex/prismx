## Code Changes

- Add `StickyNote` struct with:
  - Title, body text
  - Timestamp, background color
  - Optional pinned/focused state
- Support drag-and-drop with visual feedback
- Style note as post-it: padded, outlined, background-filled
- Movement keys: `Shift+Arrow` or drag mouse
- Integrate with Triage panel (toggle via Ctrl+Shift+N)
- Render notes in layered Z-order with focus cycling
