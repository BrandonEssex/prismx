# PrismX

**Version:** 0.1.22  
**Repository:** https://github.com/BrandonEssex/prismx  
**Author:** BrandonEssex

---

## Overview

PrismX is a modular, TUI-powered task and knowledge management platform built for developers, system engineers, and productivity-driven users. It supports project tracking, to-do triaging, mind mapping, plugin loading (WASM), and note management—all fully extendable via Rust or WASM-based plugins.

---

## Features

- TUI-based dashboard with project/task/idea triage
- Inbox-based task management
- JSON-backed data models (no DB required)
- Markdown note support
- Mindmap canvas system
- Spotlight-style fuzzy search across all data
- Zen mode and customizable shortcuts
- Plugin support via `.prismx-ext` folders with `plugin.wasm` and manifest
- Modular code layout with auto-loaded extensions

---

## Getting Started

### Running PrismX
```bash
cargo run --release
```

### Directory Layout
```bash
data/
├── inbox.json
├── mindmaps.json
├── notes/
│   ├── note1.md
│   └── ...
├── dashboard_config.json
├── widget_themes.json

extensions/
└── myplugin.prismx-ext/
    ├── plugin.wasm
    └── prismx-plugin.json
```

---

## Creating a Plugin (WASM)

### Plugin Manifest Example
```json
{
  "name": "MyPlugin",
  "version": "0.1.0",
  "author": "BrandonEssex",
  "entrypoint": "run",
  "prismx_api_version": "0.1.0"
}
```

### Plugin Rust Entrypoint
```rust
#[no_mangle]
pub extern "C" fn run() {
    // Your plugin logic
}
```

Compile to wasm:
```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

---

## Key Shortcuts

| Action                 | Shortcut            |
|------------------------|---------------------|
| Toggle Zen Mode        | Ctrl+Z              |
| Open Scratchpad        | Ctrl+Alt+N          |
| Toggle Spotlight       | Ctrl+/              |
| Archive task (inbox)   | Ctrl+D              |
| Open task entry        | Ctrl+Alt+I          |
| Navigate (Inbox)       | ↑ ↓ Enter           |

---

## Extension System

- Register `.wasm` plugin modules in `extensions/` directory.
- Plugins run in isolated sandboxed environment.
- Profiles and permissions enforced using heuristics + runtime limits.
- Supports `Capability::Filesystem`, `Capability::Network`, `Capability::Memory`, etc.

---

## Contributing

- All modules are designed to be plugin-expandable.
- Contributions welcome at:  
  [https://github.com/BrandonEssex/prismx](https://github.com/BrandonEssex/prismx)

For questions: contact `BrandonEssex` on GitHub.

---

## License

MIT License