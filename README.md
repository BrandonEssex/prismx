# PrismX `v10.1.0+Final`
_Offline-First Cognitive Planning Terminal_  
_Built for Minds That Think in Trees._

---

## 🌌 Overview

**PrismX** is a modular, offline-first, terminal-native interface for organizing thought, managing tasks, launching AI-augmented plugins, and executing workflows with total local control.  

Everything in PrismX is centered around its **Mindmap Core**, extended through **intelligent plugins**, **dashboard widgets**, and a **shard-based workspace model**.  

Version `v10.1.0` introduces **Zen Journaling**, **plugin slot rendering**, **fully interactive shortcut maps**, and the **animated PrismX icon system**—with enhancements across all AI, TUI, and plugin systems.

---

## 🚀 Key Features

| Feature                        | Description |
|-------------------------------|-------------|
| 🌳 Mindmap Core               | Real-time tree editing, task planning, in-place input |
| 🔍 Spotlight Command Bar      | Global fuzzy search, command execution, plugin launcher |
| 🗂 Shard/Tag System           | JSON-based offline node database with tags and filters |
| 📊 Dashboard Widgets          | Visual UI elements for time, shards, plugins, shortcuts |
| 🧘 Zen Mode                   | Distraction-free visual mode with journaling |
| 🍌 Animated PrismX Icon       | Theme-aware animated visual in top-right corner |
| 🔌 Plugin System              | WASM plugins with trust scoring, keybinds, UI slot support |
| ⌨ Full Keymap Control         | Ctrl/Alt/VIM mode shortcuts with editable config |
| 🧠 Project Aether Integration | AI overrides, AutoTag/Test agents, signal trust logs |
| 📴 Offline-Only Enforcement   | Complete sandbox execution with no network dependency |
| 🎨 RefractPack Personalization| Theme, layout, icon, and shortcut presets (optional) |

---

## 🖥 System Requirements

- **OS**: Linux/macOS/Windows (UTF-8 terminal)
- **CPU**: x64 / ARM64
- **Storage**: Local file access required
- **Build**: Rust 1.76+, Cargo
- **Optional**: WebAssembly Runtime (for plugins)

---

## 🔧 Installation

```bash
git clone https://github.com/your-org/prismx
cd prismx
cargo build --release
./target/release/prismx
### Build (WebAssembly)

```bash
cd crates/prismx-wasm
wasm-pack build --target web
```

See [crates/prismx-wasm](crates/prismx-wasm) for source.

Prebuilt binaries coming soon for Linux (deb/rpm), macOS (arm64/x64), and Windows.
⚙️ Quick Start

prismx                        # launch main interface
Alt + Shift + S               # open Spotlight command bar
Enter / Tab / Ctrl+N         # create mindmap nodes
Ctrl + .                     # toggle Zen Mode
Ctrl + Q                     # quit
Ctrl + ← / → (Cmd on macOS)    # horizontal scroll
🧱 Architecture

src/
├── tui/                  # UI panels, widgets, dashboard
├── shard/                # Mindmap + Tag + JSON database
├── plugin/               # WASM engine, trust scoring, slots
├── ai/                   # Project Aether agents & audit logs
├── config/               # Keymaps, themes, plugin manifests
├── export/               # File import/export interfaces
└── core/                 # Runtime signals, hooks, events
🎨 RefractPack (Themes + Personalization)

Included in v10.1.0:

Custom icon modes (banana, beam, pulse)
Toggleable animations
Full color schemes via theme.toml
Adjustable font size and style
Keybinding presets (VIM or Modern)
Example:

[icon]
mode = "banana"
animation = true

[font]
family = "JetBrains Mono"
size = 14
🔌 Plugin System

Plugins are declared via config/plugin.json and loaded from plugin/.

Example Manifest
{
  "name": "gemdrop",
  "version": "1.1.0",
  "permissions": ["shard.read", "node.write", "ui.slot.render"],
  "keybinds": ["ctrl+g"],
  "entry": "plugin/gemdrop.wasm",
  "slot": "bottom-right"
}
Plugin Features
Keybind declarations
Signal listeners (hooks)
Dashboard widgets
Visual slot renderers
Trust scoring with override log
All plugin interactions are sandboxed and logged in the trust map.

🧘 Zen Journaling

New in v10.1.0, Zen Mode includes:

Hidden UI for deep focus
Typing zone with instant journaling
Export with:
Ctrl + E
# or via CLI
prismx --zen-export > journal.txt
Zen Profile Icons:

Green → Focus
Blue → Work
Pink → Personal
🔭 Spotlight & Command Box

Fuzzy search and launcher:

Alt + Shift + S
Search examples:

"node:inspect duct"
"tag:@urgent"
"run:gemdrop"
📊 Dashboard Widgets

Modular UI components show:

Clock, day, and week
Node count and completion stats
Plugin activity
Active shard name
Keymap overlay help
Widgets are configured in:

config/dashboard.json
📂 Shards & Tags

Term  Description
Shard A JSON save of a node tree (mindmap)
Tag Metadata label (@urgent, #area, +draft)
Node  A single idea, task, or note entry
Use Spotlight to filter tags and jump between shards.

⌨ Keyboard Shortcuts

See: docs/cheatsheet.md

Default mode: Ctrl-based
VIM mode toggle available in keymap.rs

🔑 Hotkey Compatibility (macOS + iTerm2)

PrismX is fully keyboard-driven and depends on accurate key modifier support.
macOS users should enable CSI u Mode in iTerm2 for full compatibility.
Without CSI u, combinations like Ctrl+Shift+= or Alt+Shift+D may not work.

How to enable CSI u:
1. Open iTerm2 → Preferences → Profiles → Keys
2. Enable CSI u Mode
3. Accept prompt to remove conflicting mappings (can be restored later)

Recommended modifier-only shortcuts:
- Ctrl+Alt+Z: Zoom In
- Ctrl+Alt+X: Zoom Out
- Ctrl+Alt+0: Reset Zoom
- Alt+Shift+D: Toggle Debug Mode

All commands use modifiers (no raw letters) to preserve typing flow in Zen, Spotlight, and GemX.
🧠 Project Aether (AI Subsystems)

Included in 10.1.0:

Autonomous Task Planning (internal)
AutoTest Agents (QA, CI-like behavior)
AutoTag Agent (real-time tagging)
Trust Scoring + Override Logs
AI plugin isolation sandbox
All AI events are logged and reversible.

🛠 Contribution Guidelines

Fork, branch, and PR to dev
All PRs must:
Pass cargo test
Declare keymaps in keymap.rs
Register plugins in plugin.json
Update documentation if needed
📚 Documentation

End User Manual
Developer Guide
Plugin API Reference
Keyboard Cheat Sheet
AI Override Log
🧙 Credits

Developed by the Public Infrastructure Consortium.
Extended by Project Aether AI under supervised autonomy.

🛡 License

MIT License. Use freely. Fork responsibly.
See LICENSE.