# Patch 25.51u-a – Tab Crash Fix & Root Sanity Precheck

## Goals
- Fix crash when inserting child with Tab after root loss
- Ensure ensure_valid_roots() executes *before* layout_nodes()
- Prevent layout wipeout or crash when adding to unreachable tree
- Preserve promotion logging from 25.51u
