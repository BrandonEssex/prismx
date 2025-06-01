## Code Changes

- When inserting sibling:
  - Anchor to original node's parent
  - Do not overwrite parent's child list unless explicitly moved
- Log: `SIBLING_OK: parent_id = ...` after every sibling insert
