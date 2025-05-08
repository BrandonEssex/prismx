# PrismX — Modular Terminal Organizer (v0.1.14)

**Built for System Engineers, DevOps, Developers, and Plugin Hackers**

---

## ✨ Overview

PrismX is a customizable TUI (terminal UI) productivity environment built in Rust.  
It blends project triage, note-taking, timers, dashboards, and plugin execution into one smooth terminal experience.

Built with:

- **Ratatui**: Responsive, styled terminal UI
- **Crossterm**: Cross-platform input/output
- **Serde/JSON**: Config and data persistence
- **WASM Plugins**: Extension support with sandboxed execution
- **Tokio**: Async support for plugin loading and IO

---

## ⚙️ Installation

### Prerequisites

- Rust 1.70+
- macOS/Linux terminal
- [Optional] For plugin development: `wasm-pack`, `wasmtime`

```bash
git clone https://github.com/yourname/prismx
cd prismx
cargo build --release
🚀 Usage

./target/release/prismx
⌨️ Keyboard Shortcuts

Keys  Action
Ctrl + Z  Toggle Zen Mode
Ctrl + /  Open Spotlight Search
Ctrl + N  Create Inbox Task
Ctrl + T  Toggle Triage View
Ctrl + P  Start Pomodoro Timer
Ctrl + W  Start Stopwatch
Ctrl + Q  Quit
📦 Modules & Features

Inbox & Triage
Manage tasks with shards, tags, priority, assignment, and archival.

Spotlight
Fuzzy-search any resource via plugins with real-time filtering and inline actions.

Zen Mode
Fullscreen writing zone. Autosaves to ~/.config/prismx/zen_scratchpad.md.

Dashboard (Experimental)
Grid-based layout with draggable widgets like Mindmap, Extensions, and Notes.

🧩 Plugins (WASM)

Write Your Own Plugin
Every plugin is a .prismx-ext directory with:

example-plugin.prismx-ext/
├── plugin.wasm
└── prismx-plugin.json
Manifest Example (prismx-plugin.json):

{
  "name": "SysInspector",
  "author": "you",
  "version": "1.0.0",
  "prismx_api_version": "0.1.0",
  "entrypoint": "run"
}
WASM Entry Point:

#[no_mangle]
pub extern "C" fn run() {
    println!("Hello from plugin!");
}
Compile Your Plugin
wasm-pack build --target web
🧪 CLI Testing

RUST_LOG=debug cargo run --release
📂 File Structure (User Edition)

prismx/
├── data/                  # JSON notes, tasks, mindmaps
├── extensions/            # Plugin directories (*.prismx-ext)
├── logs/                  # Debug logs
├── assets/                # Templates (e.g. scratchpad)
├── exports/               # .md export destination
├── config.toml           # Global config
└── target/release/prismx
✅ Confirmed Functional Modules

 Zen Mode (autosave, hotkeys)
 Inbox Triage (create, assign, archive)
 Pomodoro + Stopwatch
 Plugin sandboxing (wasmtime)
 Spotlight search engine
 Persistent JSON/Markdown data
🔧 Configuration

Edit config.toml or override scratchpad path:

[zen_mode]
autosave_interval_secs = 5
scratchpad_path = "~/.config/prismx/my_scratchpad.md"
🔒 Security

WASM plugins sandboxed with memory + CPU limits
Filesystem access is explicitly restricted by capabilities
❤️ Contribute

Fork + PR. New plugins welcome!

🧠 Final Notes

This README is kept in sync with every new version.
Plugin authors, system admins, and terminal hackers — welcome.