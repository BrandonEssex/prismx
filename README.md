# PrismX User Manual

---

## **1. Introduction & System Requirements**

PrismX is a modular, terminal-based productivity platform designed for engineers, developers, and sysadmins. Written in Rust, it combines Zen-mode writing, TUI dashboards, fuzzy Spotlight search, plugin sandboxing, and task triage into a unified experience.

### 1.1 Target Audience
- Infrastructure engineers
- TUI enthusiasts
- Terminal-focused task managers

### 1.2 Supported Platforms
- macOS (x86_64, Apple Silicon)
- Linux (x86_64, ARM64)

### 1.3 System Requirements
- Rust 1.72+
- UTF-8 capable terminal (recommended: Alacritty, Kitty, iTerm2)
- For plugins: Wasmtime-compatible WASM host

---

## **2. Detailed Features & Usage Examples**

### 2.1 Zen Mode
- Activate: `Ctrl+Z`
- Minimal fullscreen Markdown scratchpad
- Autosaves every 10s
- Manual Save: `Ctrl+S`

### 2.2 Inbox & Triage
- Toggle View: `Ctrl+T`
- Add Task: `Ctrl+Alt+N`
- Archive: `Ctrl+D`
- Tag, assign, prioritize

### 2.3 Mindmap Engine
- Node canvas editor
- Create/move/delete nodes
- Undo/Redo supported

### 2.4 Spotlight Search
- Launch: `Ctrl+/`
- Fuzzy query with scoring
- Inline Actions: `m` (move), `x` (delete), `e` (export), `f` (pin)

### 2.5 Dashboard
- Drag & drop widgets
- Mindmap, Pinned, Extensions
- Undo/Redo and grid snap

### 2.6 Plugin Host
- Execute `.wasm` plugins
- Capability-based FS/Net/CPU limits
- Wasmtime sandbox

---

## **3. Complete Configuration & Customization Reference**

Config file: `~/.config/prismx/config.toml`

```toml
[zen_mode]
autosave_interval_secs = 10
title_fade_delay_secs = 2
scratchpad_path = "~/.config/prismx/zen_scratchpad.md"

[extension_host]
plugin_dir = "extensions/"
log_level = "info"

[dashboard]
grid_snap = 5
```

Shortcut remapping is possible via `input.rs` or future `[shortcuts]` section.

---

## **4. Third-party Integrations & Extensibility Guide**

### 4.1 Spotlight Plugin Injection
- Implement the `Searchable` and `SearchableSource` traits
- Register in `PluginRegistry`

### 4.2 Extension Host Plugins
- Directory format:
```
example-plugin.prismx-ext/
├── plugin.wasm
└── prismx-plugin.json
```

- Required Manifest Fields:
```json
{
  "name": "Example Plugin",
  "author": "Dev",
  "version": "0.1.0",
  "prismx_api_version": "0.1.0",
  "entrypoint": "run"
}
```

---

## **5. Troubleshooting, Logging, and Debugging**

### 5.1 Log Files
- `logs/zen_debug.log`
- `logs/spotlight.log`
- `logs/extension_host.log`

### 5.2 Enable Debug Mode
```sh
export PRISMX_DEBUG=true
```

### 5.3 Fallback Recovery
- If `inbox.json`, `dashboard_config.json`, or `mindmaps.json` corrupts, a `.bak` file is created and the system loads defaults.

---

## **6. Advanced User Guide**

### 6.1 Autosave Coordination
- Zen scratchpad and Mindmap autosave independently
- Use `update()` cycle to sync periodic saves

### 6.2 Exported Markdown
- Spotlight allows `e` to export matched items
- Files written to `exports/[uid].md`

### 6.3 Theme & Widget Customization
- Modify `widget_themes.json`
- Example:
```json
{
  "mindmap": {
    "border_color": "blue"
  }
}
```

---

## **7. Developer & Extension Building Guide**

### 7.1 Building PrismX
```sh
git clone https://github.com/prismx-org/prismx
cd prismx
cargo build --release
```

### 7.2 Plugin API Notes
- Plugins must be compiled to WASM
- Use `wasmtime` compatible toolchain (e.g., `wasm32-wasi`)

### 7.3 Debugging Plugins
- Log to `stdout` or file
- Enable tracing via host log level

### 7.4 Favorites and Predictive Search
- Spotlight tracks frequency and recency per UID
- Can boost score manually via `.favorites.rs`

---

## **8. Comprehensive Reference Appendices**

### A. File Storage Map
| File                          | Purpose                           |
|-------------------------------|-----------------------------------|
| `inbox.json`                 | Triage task store                 |
| `mindmaps.json`              | Mindmap data                      |
| `zen_scratchpad.md`          | Zen buffer                        |
| `dashboard_config.json`      | Widget layout                     |
| `spotlight.log`              | Spotlight query history           |
| `widget_themes.json`         | Custom widget styles              |

### B. Default Shortcuts
| Action              | Shortcut     |
|---------------------|--------------|
| Zen Mode Toggle     | `Ctrl+Z`     |
| Inbox Toggle        | `Ctrl+T`     |
| Open Spotlight      | `Ctrl+/`     |
| Manual Save         | `Ctrl+S`     |
| Archive Task        | `Ctrl+D`     |
| Scratchpad Focus    | `Ctrl+Alt+N` |

### C. Glossary
- **TUI** – Terminal User Interface
- **WASM** – WebAssembly sandbox module
- **Zen Mode** – Focused writing environment
- **Capability** – Permission granted to plugin
- **Overlay** – Popup-style modal in terminal

---