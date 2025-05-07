# PrismX TUI Organizer (v0.1.12)

PrismX is a modular, extensible, terminal-based productivity suite for system engineers, DevOps professionals, and CLI-first users. It combines real-time task management, mindmapping, plugin execution, and distraction-free note-taking—customizable via extensions and written entirely in Rust.

---

## ✨ Features

- **Interactive TUI Dashboard** with mode switching (Inbox, Zen Mode, Spotlight Search)
- **Zen Mode**: Minimalist autosaving markdown scratchpad
- **Inbox & Triage View**: Task intake, tagging, and routing
- **Mindmap Engine**: JSON-based, node-level asynchronous control API
- **Spotlight**: Real-time fuzzy search with action commands
- **WASM Plugin Engine**: Safe, profile-aware extension execution
- **Cross-platform**: macOS & Linux ready

---

## ⚙️ Installation

```bash
git clone https://github.com/your-org/prismx
cd prismx
cargo build --release
./target/release/prismx
🧑‍💻 Usage Walkthrough

Launch PrismX
./prismx
Keyboard Shortcuts
Shortcut  Action
Ctrl + Z  Toggle Zen Mode
Ctrl + Alt + N  New Inbox Entry (from anywhere)
Ctrl + D  Toggle Spotlight Debug Overlay
Esc Exit modes or cancel input
Arrow Keys  Navigate TUI widgets
Enter Confirm/Activate item
📥 Inbox/Triage Mode

Launch with Ctrl + Alt + N or via TUI
Tasks are autosaved to data/inbox.json
Supports tagging, assigning, prioritizing, and archiving
🧘 Zen Mode

Toggle with Ctrl + Z
Scratchpad stored at ~/.config/prismx/zen_scratchpad.md
Auto-saves every 10 seconds
Uses fallback if file is missing
🔍 Spotlight

Triggered by Ctrl + /
Searches across notes, projects, plugins (extendable)
Inline actions:
m: Move
x: Delete
e: Export .md
f: Favorite
📦 WASM Plugin Engine

Example Plugin Directory
extensions/
└── example-plugin.prismx-ext/
    ├── plugin.wasm
    └── prismx-plugin.json
Manifest Format (prismx-plugin.json)
{
  "name": "Example Plugin",
  "author": "You",
  "version": "1.0.0",
  "prismx_api_version": "0.1.0",
  "entrypoint": "main"
}
Entrypoint
Your plugin must export a no-arg WASM function matching the manifest entrypoint.

#[no_mangle]
pub extern "C" fn main() {
    // Your plugin logic here
}
Plugin Capabilities
Plugins run in Wasmtime sandbox
CPU and memory limits enforced
Future: networking, file access (capability gated)
🧪 CLI & JSON Usage Examples

Triage Task from CLI (Simulated via file)
{
  "title": "Restart CI agents",
  "tags": ["infra", "urgent"],
  "assigned_to": "team-devops",
  "priority": "High"
}
Drop this JSON into data/inbox.json or use TUI to modify interactively.

Mindmap API Example (via mindmap_api.rs)
create_node("New Node", 42.0, 24.0).await;
move_node(1, 88.0, 50.0).await;
delete_node(1).await;
🛠 Configuration

Configuration can be expanded in ~/.config/prismx/config.toml:

[zen_mode]
title_fade_delay_secs = 2
autosave_interval_secs = 10
scratchpad_path = "~/.config/prismx/zen_scratchpad.md"
📂 File Structure

data/
├── inbox.json
├── dashboard_config.json
├── mindmaps.json
└── widget_themes.json

logs/
├── zen_debug.log
└── spotlight.log

assets/
└── default_scratchpad.md

extensions/
└── example-plugin.prismx-ext/
    ├── plugin.wasm
    └── prismx-plugin.json
🔧 Developers & Contributors

Extend the TUI via Rust modules under src/
Add plugins by registering in extensions/
Submit patches via pull request
🔄 Version History

0.1.12 — Stable TUI, Spotlight fixes, plugin integration revalidated
0.1.11 — Ingestable JSON, Inbox fully patched
0.1.10 — Mindmap refactored, diagnostics added
🧩 Coming Soon

Plugin capabilities UI
Cloud-sync options
Markdown import/export for notes and mindmaps
MIT Licensed | Built with Rust | DevOps ready