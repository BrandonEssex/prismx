# Patch 25.51u-l – Root Visibility + Sibling Insert Repair

## Goals
- Fix layout wipe caused by root nodes that are unreachable or not drawn
- Ensure root nodes with no children are still drawn
- Fix `add_sibling()` when inserting into a root context
