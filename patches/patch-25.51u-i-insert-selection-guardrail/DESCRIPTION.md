# Patch 25.51u-i – Selection + Insert Guardrail Patch

## Goals
- Ensure that `Enter` always updates the selection to the new sibling
- Ensure `Tab` only works when a valid, connected node is selected
- Prevent fallback from triggering due to broken insert states
- Log an explicit warning when `Tab` is called without valid selection
