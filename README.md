# PrismX Terminal Productivity OS
![version](https://img.shields.io/badge/version-v2.6.0--QA--final-blue)

**Version:** v2.6.0-beta1-QA  
**Branch:** QA  
**Author:** Brandon Essex  
**License:** MIT  
**Status:** Quality Assurance – Integration Pending

---

## Overview

**PrismX** is a modular, offline-first, keyboard-centric productivity OS built for developers, system operators, and technical thinkers. It fuses a dynamic **mindmap interface**, programmable **plugin runtime**, and a highly-configurable **terminal UI** into one fast, extensible workspace.

> PrismX is not a note-taker. It’s a command center for structured thought.

---

## Key Features

### 1. Mindmap Core (GemX Engine)
- Tree-based node editing, rendered live in the terminal.
- Inline editing with instant keyboard commands.
- Hierarchical node structures: Tasks, notes, bookmarks, plans.
- Node typing: `#task`, `#idea`, `#ref`, etc.
- Instant node creation:
  - `Enter` - Create sibling node.
  - `Tab` - Create child node.
  - `Ctrl+N` - Create free-floating node.
- Upcoming:
  - Drag-and-drop via keyboard or mouse.
  - MindTrace Zettelkasten plugin (smart backlinks).

---

### 2. Shard System
- Lightweight project containers.
- Each shard has isolated state, layout, and plugin context.
- Switchable via hotkeys or CLI (planned).
- Shard export/import (WIP).

---

### 3. Plugin System
- Runtime plugin loader with manifest declarations.
- Sandboxed permission model.
- Plugin hooks include:
  - UI slots
  - Keybindings
  - Metadata overlays
  - State access
- Example plugins: `focus-beacon`, `node-stats`, `logtail`.

---

### 4. RefractPack v1.2.0
A default plugin suite included with PrismX.

| Plugin         | Description                                 |
|----------------|---------------------------------------------|
| Clock          | Realtime terminal clock widget              |
| Shortcut Map   | Displays all active hotkeys (`?` to open)   |
| Log Viewer     | Tail system logs in-app                     |
| Plugin Dash    | View and manage installed plugins           |
| Focus Beacon   | Dynamic Prism-X symbol with context glow    |
| Tag Glossary   | Shows tag cloud and jump targets            |
| Debug Overlay  | WIP – System performance and state info     |

---

### 5. Command Bar
- `:` prefixed commands (e.g. `:zen`, `:plugin reload`)
- Upcoming:
  - Plugin-defined commands.
  - Command history and autocomplete.
  - Macros and presets.

---

### 6. Zen Mode
- Fullscreen minimalist view for focused writing.
- Clean, no UI noise.
- Escape to exit Zen.
- Green border fixed in this version (QA Patch).

---

### 7. Tag and Typing System
- Tags (`#idea`, `#bug`, `#urgent`) attach to any node.
- Tag Glossary plugin displays usage statistics.
- Node typing system in progress.
- Smart auto-tagging (upcoming via AI plugin).

---

### 8. UI Layout and Slots
- Dashboard layout with plugin-defined slots:
  - Sidebar
  - Footer
  - Zen Area
  - Command Area
- Theme-aware rendering.
- Planned: Widget resizing and snap-grid layout.

---

### 9. Storage and Export
- JSON-backed persistent storage.
- Auto-backups in `/data/backup/`
- Export system:
  - JSON (shard and node data)
  - Markdown (tree structure)
  - Glossary/tag-based reports
- Upcoming: PDF, HTML, CSV exporters.

---

### 10. Keybinding System
Fully user-configurable via `config/keymap.json`.

| Action         | Default Key   |
|----------------|----------------|
| New Node       | Ctrl+N         |
| Sibling Node   | Enter          |
| Child Node     | Tab            |
| Move Focus     | Ctrl+H/J/K/L   |
| Edit Node      | Ctrl+E         |
| Zen Mode       | `:zen`         |
| Show Shortcuts | `?`            |

---

## QA Status (v2.6.0-beta1)

| Component         | Status     | Notes                                      |
|------------------|------------|--------------------------------------------|
| Mindmap View     | ✅ Stable  | Renders on launch, full keyboard support   |
| Zen Mode         | ✅ Fixed   | Green border restored, stable rendering    |
| Plugin Runtime   | ✅ Stable  | Slots load correctly, no panics reported   |
| Command Bar      | ⚠️ Partial| Requires toggle fix to show on launch      |
| Icon System      | ⚠️ Partial| Color update inconsistent between modes    |
| Overlay/Shortcuts| ✅ Stable  | `?` hotkey functioning and theme-safe      |

---

## File Structure

/src
├─ main.rs # Entry point
├─ state/ # App and UI state
├─ node_tree/ # Mindmap logic
├─ plugin/ # Plugin manifest and registry
├─ config/ # Keymaps, themes, toggles
├─ export/ # Export modules (md/json/etc.)
├─ triage/ # Inbox and capture system
├─ dashboard_widgets.rs # RefractPack UI definitions


---

## Roadmap

| Version     | Features                                                         |
|-------------|------------------------------------------------------------------|
| v2.6.0      | Plugin Store, Manifest Validator, Runtime Permissions            |
| v2.7.0      | Export API (PDF/HTML/CSV), AI MindTrace Engine                   |
| v2.8.0      | Multi-User Shards, Shared Views, Real-Time Collaboration         |
| v2.9.0      | Embedded Scripting (Rhai), CLI DSL, Node Automations             |
| v3.0.0      | PrismX OS Mode: System Control, AI CLI (`pxa`), Task Compiler    |

---

## Power User Notes

- Every node = thought, tag, or plan.
- Use shards to isolate clients, projects, or contexts.
- Plugins are fully modular and declarative.
- Tags shape the visual and logic layer.
- Mindmap always remains central — no optional mode.
- JSON = inspectable, portable, versionable.

---

## Plugin Manifest Example

```json
{
  "id": "focus-beacon",
  "name": "Dynamic Prism Icon",
  "entry": "focus_beacon.rs",
  "permissions": [
    "state.read",
    "ui.render.status_bar",
    "event.listen"
  ]
}
Contributing

Pull requests must include working builds (cargo build)
Format with cargo fmt, lint with cargo clippy
Test manually in Zen, Mindmap, and Plugin modes
Submit plugins via /plugin/your_id/ with valid manifest
Author

Brandon Essex
MIT License
Offline-first. Modular. Built for builders.

Coming Soon

Plugin Storefront UI
AI CLI Assistant (pxa)
Zettelkasten Backlink Engine (MindTrace)
Export Templates + Theming