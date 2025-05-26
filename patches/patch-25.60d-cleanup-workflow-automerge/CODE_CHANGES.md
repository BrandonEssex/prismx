## Code Changes

- Adds `should-run-patch.sh`: returns non-zero for legacy or deprecated patch IDs
- Adds patch metadata check: valid patches must have all 3 files (DESCRIPTION.md, FILES_CHANGED.txt, CODE_CHANGES.md)
- Skips workflow entirely if patch does not follow scaffold format
- Enables GitHub auto-merge for any patch with no conflicts + passing tests
