## Code Changes

### 1. Track Last Fallback Position (In AppState)

```rust
pub fallback_next_x: i16,
pub fallback_next_y: i16,
Initialize:

fallback_next_x = 6;
fallback_next_y = GEMX_HEADER_HEIGHT + 2;
2. Update Coordinates During Fallback Injection
After fallback promotion:

let n = state.nodes.get_mut(&id).unwrap();
if n.x == 0 && n.y == 0 {
    n.x = state.fallback_next_x;
    n.y = state.fallback_next_y;

    // Advance grid
    state.fallback_next_y += 3;
    if state.fallback_next_y > area.height as i16 - 4 {
        state.fallback_next_y = GEMX_HEADER_HEIGHT + 2;
        state.fallback_next_x += 20;
    }
}
3. Reset Grid On Mode Switch or Tree Reset (Optional)
state.fallback_next_x = 6;
state.fallback_next_y = GEMX_HEADER_HEIGHT + 2;
4. Debug Print (Optional)
eprintln!("📐 Fallback placed Node {} at ({}, {})", id, n.x, n.y);
