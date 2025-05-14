# PrismX: Modular Offline Planning System
![version](https://img.shields.io/badge/version-v2.6.0--QA--final-blue)

Version: v2.6.0-beta1-QA
Branch: QA
Author: Brandon Essex
License: MIT
Status: Quality Assurance – Integration Pending

⚡ Overview

PrismX is a modular, offline-first, keyboard-centric productivity OS built for developers, system operators, and technical thinkers. It fuses a dynamic mindmap interface, programmable plugin runtime, and a highly-configurable terminal UI into one fast, extensible workspace.

PrismX is not a note-taker. It’s a command center for structured thought.
🚀 Key Features

🧠 1. Mindmap Core (GemX Engine)
Tree-based node editing, rendered live in the terminal
Inline editing with instant keyboard commands
Hierarchical node structures: Tasks, notes, bookmarks, plans
Node typing: #task, #idea, #ref, etc.
Instant node creation via:
Enter → Create sibling
Tab → Create child
Ctrl+N → Create free-floating node
Upcoming:
Drag-and-drop via keyboard or mouse
MindTrace Zettelkasten plugin (smart backlinks)
🧭 2. Shard System
Lightweight project containers
Each shard has isolated state, layout, plugin context
Switchable via hotkeys or CLI command (planned)
Shard export/import (WIP)
🧰 3. Plugin System
Runtime plugin loader with manifest declarations
Sandboxed permission model
Plugin hooks include:
UI slots
Keybindings
Metadata overlays
State access
Example plugin: focus-beacon, node-stats, logtail
💎 4. RefractPack v1.2.0
A default plugin suite included with PrismX:

Plugin  Description
Clock Realtime terminal clock widget
Shortcut Map  Displays all active hotkeys (? to open)
Log Viewer  Tail system logs in-app
Plugin Dash View and manage installed plugins
Focus Beacon  Dynamic Prism-X symbol with context glow
Tag Glossary  Shows tag cloud and tag-based jump targets
Debug Overlay WIP - System performance & state info
🛠 5. Command Bar & CLI System
: prefixed command input
Upcoming:
Plugin-registered commands
Autocomplete
Context-aware macros
🎯 6. Zen Mode
Fullscreen minimalist view of current node or branch
Ideal for writing, reading, or planning
Escape to return
Green border for focus confirmation (QA Patch Fixed)
🧩 7. Tag & Node Typing Engine
Attach #tags to any node
Node Types planned for task, note, comment, reference
Glossary Plugin: Displays tag usage, frequency, links
Upcoming: Smart Auto-Tagging via Node AI Hooks
🧱 8. Layout + UI Slots
Configurable dashboard layout system
Slot-based plugin areas:
Sidebar
Footer
Zen Panel
Command Bar
Theme-aware coloring and borders
Upcoming: Grid snapping + widget resizing
🧾 9. Storage & Export System
JSON-backed database (human-readable)
Auto-backup to /data/backup
Export options:
JSON full shard
Markdown node tree
Tag Index (Glossary)
Planned:
PDF, HTML, and CSV export plugins
⌨️ 10. Keymap System
Customizable and declarative via config/keymap.json:

Action  Default Key
New Node  Ctrl+N
Sibling Node  Enter
Child Node  Tab
Move Focus  Ctrl+H/J/K/L
Edit Node Ctrl+E
Zen Mode  : then zen
Shortcuts ?
Upcoming: Vim/Emacs layers, Mouse Mode, Gamepad input (experimental)

🧪 QA Status (v2.6.0-beta1)

Component Status  Notes
Mindmap Display ✅ Stable  Default layout rendering patched
Zen Mode  ✅ Fixed Border rendering and green state restored
Plugin Loader ✅ Stable  New plugins registered via manifest
Prism Icon System ⚠️ Partial  Color change triggers only in some views
Command Bar ⚠️ Pending  Still hidden by default on toggle
Layout Sync ✅ Stable  No visual shifts on dashboard refresh
Shortcut Overlay  ✅ Stable  Fully working with ? hotkey
📁 Project Structure

/src
 ├─ main.rs              # App entry point
 ├─ state/               # App state, UI state, node state
 ├─ node_tree/           # Mindmap structure + traversal
 ├─ plugin/              # Plugin registry, sandbox, manifest
 ├─ config/              # Themes, keymaps, runtime flags
 ├─ export/              # Export formats (md, json, etc)
 ├─ triage/              # Inbox scratchpad + capture logic
 ├─ dashboard_widgets.rs # RefractPack components
📌 Roadmap (Confirmed Milestones)

Version Highlights
v2.6.0  Plugin Store (manifest validator, local registry)
v2.7.0  Export API (PDF/HTML/CSV), Node Intelligence Layer (MindTrace)
v2.8.0  Multi-user collaboration (shared shards), User presence overlay
v2.9.0  Embedded scripting (Rhai), Custom node actions, AI inspector hooks
v3.0.0  PrismX OS Mode – task scripting DSL, system-wide command bus
🧠 Power User Notes

Every node = a thought, task, tag, or plan
Tag combinations define contexts and filters
Shards are like isolated universes – you can import/export them
Plugins define behavior: want task automation? Build a plugin.
All state is transparent, versioned, and portable
“Minimal by default, extensible when needed” is the design mantra
🔧 Example Plugin Manifest

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
🤝 Contributing

Fork → PR → QA Test
Use cargo fmt, cargo clippy, and descriptive commits
QA patches must pass startup.sh + plugin validation
👤 Author & Aesthetic

Brandon Essex
Offline-first. Modular. Beautiful by accident, fast by design.

📦 Coming Soon

Plugin Storefront (plugin.store/)
AI CLI Assistant (pxa)
PrismX Community Presets (themes/, layouts/, nodesets/)
Zettelkasten MindTrace Engine