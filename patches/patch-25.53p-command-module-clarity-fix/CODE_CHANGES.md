## Code Changes

### 1. Spotlight Background Clearing
- Added `Clear` widget in `render_spotlight` to clear the drawing area so
  background panels cannot bleed through.

### 2. Solid Styling
- Spotlight block, input line, preview text and suggestions now render with
  `bg(Color::Black)` for full opacity.
- Removed dim styling from suggestions and warnings.
- Input line uses `Modifier::BOLD` for easier reading.
