## Code Changes

- Refactor `recalculate_roles()`:
  - Prevents nodes from being parented to themselves
  - Skips assignment if node is already assigned
  - Uses clearer coordinate bounds (e.g. y > parent.y + CHILD_SPACING_Y)
  - Logs recursion skips and parent changes
- Add `debug_input_mode` trace to fallback injection
- Limit fallback promotions per frame to 1
- Clamp fallback recursion to 32 safe frames
