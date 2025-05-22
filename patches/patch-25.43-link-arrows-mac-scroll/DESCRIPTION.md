# Patch 25.43 – Link Arrows & Mac Scroll Fallback

## Goals
- Add directional arrows between linked nodes in mindmap (GemX)
- Add Cmd+Left / Cmd+Right key fallback for macOS terminals to trigger horizontal scroll

## Affected Modules
- `src/gemx/render.rs`
- `src/input/mac_fallback.rs`

## QA Checks
- Ctrl+← / Ctrl+→ scrolls on Linux/Ubuntu
- Cmd+← / Cmd+→ scrolls on macOS
- Links now show arrows in mindmap view
