# PrismX `v10.0.0`
_Offline-First Cognitive Interface for Engineering, Planning, and Autonomous Extension_

> Built for minds that think in trees.

---

## 🌌 Overview

**PrismX** is a modular, offline-first engineering command interface designed to unify planning, task management, intelligent plugins, and visual cognition. At its core, PrismX uses a tree-based **Mindmap Interface**, paired with a real-time **Dashboard**, extensible **Plugin System**, and persistent local **Shard/Tag Database**.

**Project Aether** (introduced in `v10.0.0`) brings AI-assisted autonomy, federation override logging, and internal evolution modeling for future-proofed extensibility.

---

## 🚀 Key Features

| Feature | Description |
|--------|-------------|
| 🌳 **Mindmap Core** | Rapid tree-based thought capture with in-place editing, hierarchy, and shortcut navigation |
| 🧠 **Smart Plugins** | Modular WASM-based extensions with trust scoring, sandboxing, and signal hooks |
| 🛠 **Offline-First Mode** | Fully usable with no internet; syncs only when configured to do so |
| 🗂 **Shard & Tag System** | JSON-based searchable database of nodes, tasks, and ideas |
| 🔭 **Spotlight Launcher** | Instant fuzzy search and command execution from any screen |
| 🎛 **Dashboard Widgets** | Realtime visual layout with live clocks, shard counts, plugin state, and alerts |
| 🎨 **Zen Mode** | Distraction-free interface with adjustable visual context (Work, Personal, Focus) |
| ⌨ **Keymap Overlays** | Universal keybindings with support for Ctrl, Alt, and custom maps |
| ⚡ **AutoTag & AutoTest Agents** | AI agents monitor and tag new data, perform commit-level tests |
| 🧬 **Trust & Override Logs** | All AI decisions and plugin actions are traceable and permissioned |

---

## 🖥 System Requirements

- **OS**: Linux, macOS, Windows (x64 / ARM64)
- **Terminal**: ANSI-compatible, UTF-8 required
- **Storage**: Local file system access (uses JSON + snapshots)
- **Dependencies**: Rust 1.76+ (for builds), WebAssembly runtime (optional)

---

## 🔧 Installation

### 🦀 Build from Source

```bash
git clone https://github.com/your-org/prismx
cd prismx
cargo build --release
./target/release/prismx

📦 Binary Release (Coming Soon)
Pre-built packages will be available for:

macOS (arm64, x64)
Linux (deb, rpm)
Windows (.exe)
⚙️ Quick Start

# Launch PrismX in terminal
prismx

# Open Mindmap
Alt + M

# Create a new node
Enter or Ctrl + N

# Search or execute command
Alt + Spacebar

# Toggle Zen Mode
Ctrl + .

# Exit
Ctrl + Q
🧱 Core Architecture

src/
├── tui/                  # Terminal UI (dashboard, mindmap, overlays)
├── plugin/               # Plugin loader, sandboxing, trust scoring
├── shard/                # Tree + Tag + Database system
├── core/                 # Core traits, state, IO
├── config/               # Keymaps, plugin manifests, themes
├── export/               # External import/export tools
└── ai/                   # Project Aether AI subsystems
📦 Plugin System (v2)

Plugins declare their own:

Capabilities (data access, keybinds, signals)
Slot renderers (TUI widgets or overlays)
Trust score requirements (AI enforcement)
Configs and theming
Runtime behavior (hooks, tasks, async polling)
📄 Plugin Manifest Example:

{
  "name": "gemdrop",
  "version": "1.0.0",
  "permissions": ["shard.read", "node.write", "ui.slot.render"],
  "keybinds": ["ctrl+g"],
  "entry": "plugin/gemdrop.wasm"
}
See /config/plugin.json for plugin registry.
🧠 Project Aether (Experimental AI)

Autonomous Task Planning
AutoTag + AutoTest Agents
Plugin Performance Sandbox
AI Override Trail (Audit View)
Federation Learning Modules
Adaptive UI Synthesis
All AI actions are logged, scored, and subject to override.

🗺 Roadmap

Milestone Status    Notes
v1.0.0    ✅ Done    Core CLI/TUI + Mindmap
v2.0.0    ✅ Done    Full Plugin Framework
v3.0.0    ✅ Done    Dashboard + Zen Mode
v5.0.0    ✅ Done    RefractPack, Signal Bus
v10.0.0   ✅ Now     Project Aether + Trust Logs
v11+ 🔜 Dev     Federation Sync, AutoModeling
🙌 Contributing

Fork the repo, create a branch
Submit via pull request with [feature] or [fix] prefix
All PRs must pass AutoTest and Trust Scoring
🧪 Test Suite
cargo test --all
📚 Documentation Index

End User Manual
Developer Integration
Plugin API
Keybinding Cheatsheet
AI Override Trail
🧙 Credits

Built by humans, extended by AI.
PrismX is a project of the Public Infrastructure Consortium.

🛡 License

MIT License. Use freely, fork responsibly.
See LICENSE file for full terms.