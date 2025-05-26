## Code Changes

- Add condition: prevent `parent_id` assignment during drag unless explicit linking mode is active
- Use a `link_mode` flag or drag intent check
- Prevent cluster dragging unless it's initiated from a parent
- Log unexpected or invalid movement attempts if debug mode is on
