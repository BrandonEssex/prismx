# PrismX - Modular TUI Framework for Engineers & Plugin Developers

**Version:** 0.1.22  
**License:** MIT  
**Platform:** macOS/Linux (Unix-like)  
**Interface:** Terminal (TUI) + CLI  
**Audience:** DevOps Engineers, Plugin Developers, Power Users

---

## ✨ Overview

PrismX is a modular, extensible terminal-based system designed to help engineers organize and control projects, notes, mindmaps, tasks, dashboards, and external WASM-based plugins.

It offers powerful plugin support, JSON-backed storage, real-time TUI rendering, keyboard-centric navigation, and a growing ecosystem of enhancements.

---

## ✅ Key Features

- **Modular Plugin Host** with WASM sandboxing and runtime profiling.
- **Zen Mode** scratchpad for focused writing/editing.
- **Inbox & Triage** system for task management.
- **Mindmap Engine** with live canvas rendering.
- **Spotlight Search** with fuzzy matching & ranking.
- **Dashboard Engine** (optional plugin integration).
- **Config Watcher** with hot-reloading.
- **Themed Logging** and autosave-safe fallback behavior.
- **TUI optimized** for mouse-free keyboard navigation.
- **JSON-structured** persistent data storage.

---

## 🚀 Getting Started

### 1. **Build & Launch**

```bash
git clone https://github.com/your-org/prismx.core.git
cd prismx.core
cargo build --release
./target/release/prismx

    Required Dependencies:

        Rust 1.70+

        libwasmtime for WASM plugins

        macOS/Linux terminal with UTF-8

2. Directory Layout

prismx/
├── assets/               # Templates & initial JSON
├── config.toml           # Main configuration
├── data/                 # Persisted state (mindmaps, inbox)
├── logs/                 # Logs and debug traces
├── extensions/           # PrismX-compatible WASM plugins
├── src/                  # Core source
├── README.md             # You’re here

⌨️ CLI/TUI Usage
Global Shortcuts

    Ctrl + Z – Toggle Zen Mode

    Ctrl + / – Toggle Spotlight Search

    Ctrl + D – Archive selected Inbox Task

    Ctrl + Alt + N – Add New Inbox Entry

    Esc – Back

    q – Quit

TUI Navigation (Inbox & Spotlight)

    ↑ / ↓ – Move selection

    Enter – Activate / Edit

    m – Move item (Spotlight)

    x – Delete item

    e – Export item to Markdown

    f – Toggle favorite

🧩 Plugin Development Guide
Plugin Structure

A PrismX-compatible plugin must contain:

example-plugin.prismx-ext/
├── plugin.wasm              # Compiled WASM binary
└── prismx-plugin.json       # Manifest

Manifest Format

{
  "name": "Hello Plugin",
  "author": "devx",
  "version": "1.0.0",
  "prismx_api_version": "0.1.0",
  "entrypoint": "run"
}

Plugin Entrypoint (Rust/AssemblyScript)

#[no_mangle]
pub extern "C" fn run() {
    // Your logic here
}

    Tip: Use wasmtime and target wasm32-unknown-unknown.

⚙️ Configuration

Edit config.toml:

[logging]
enabled = true
log_level = "info"
log_to_file = true
log_file_path = "logs/prismx.log"

🛠️ Extension Development Roadmap

Zen Mode with autosave

WASM Plugin execution

Spotlight Search engine

Mindmap canvas + API

Inbox triage view

Plugin-injected dashboard

Configurable themes

    LSP Plugin support

🧪 Testing

cargo test        # Run unit tests
cargo test -- --ignored  # Run integration tests (plugin/dash)

🛡️ Security Notes

    All WASM plugins are sandboxed via wasmtime.

    Capability-based permission model under development.

🧠 Contributing

Pull requests welcome! For advanced plugin authorship or engine integration, check the Developer Manual (coming soon).
📄 License

MIT © 2024-2025 PrismX Contributors