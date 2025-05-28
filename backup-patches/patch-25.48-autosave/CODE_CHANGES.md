## 1. Autosave Daemon
- Spawns loop thread/timer in background
- Triggers save if dirty flag is true

## 2. Dirty Flag
- On node add/edit/delete, set dirty = true

## 3. Save Format
- Match Patch 25.47 JSON schema
- Save to ./data/autosave.json
