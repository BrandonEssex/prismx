## Code Changes

- Add enum `ZenJournalView { Compose, Review }` to AppState
- New toggle input (e.g. Ctrl+Shift+V) to switch views
- Compose Mode:
  - Entry box + timestamp
  - "Send" button (Enter or UI icon)
- Review Mode:
  - Layout all entries from today into vertical stacked panes
  - Use padding margins (25% width on each side)
  - Timestamp + tag grouping
- Use dark theme with subtle line separation between entries
