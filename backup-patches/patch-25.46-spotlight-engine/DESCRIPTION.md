# Patch 25.46 – Spotlight Engine

## Goals
- Add a modular Spotlight panel to PrismX, inspired by Alfred, fzf, and VSCode
- Support real-time fuzzy search over commands, nodes, tags, and recent actions
- Arrow-key navigation and Enter to dispatch registered actions
- ESC to cancel and return focus

## UX Model
- Trigger: Alt+Space
- Type to filter results
- ↑/↓ to move focus
- Enter to run selected item
- ESC to close

## Modules
- spotlight.rs
- integration points in gemx.rs and tui/mod.rs
