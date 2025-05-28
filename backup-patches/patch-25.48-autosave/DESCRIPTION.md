# Patch 25.48 – Persistence + Autosave

## Goals
- Add automatic periodic save of active workspace
- Prevent data loss during crash or forced exit
- Save occurs every N seconds if dirty

## Details
- File: `autosave.json` in ./data/
- Time interval: every 10s (configurable)
- Save only if state is dirty

## Affected Files
- `src/state/autosave.rs` (new)
- `src/gemx/events.rs`: hook into activity
- `src/storage/json_io.rs`: shared with Patch 25.47
