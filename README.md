# PrismX

**PrismX** is a modular, Rust-based Terminal User Interface (TUI) productivity hub for macOS and Linux. Designed for developers, system engineers, and terminal-centric power users, PrismX supports mindmapping, task triage, Zen writing mode, Spotlight-style search, and WASM-powered plugins — all within a keyboard-friendly terminal UI.

---

## Features

- **Mindmap Builder** – Visualize ideas hierarchically in a node-based layout.
- **Inbox & Triage** – GTD-inspired inbox view with tag-based filtering.
- **Zen Mode** – Minimal writing interface with autosave & scratchpad.
- **Spotlight Search** – Global fuzzy search overlay with keyboard-driven commands.
- **Custom Dashboards** – Add widgets and plugin outputs to a central workspace.
- **WASM Plugin System** – Load sandboxed WebAssembly extensions dynamically.
- **JSON State Storage** – Persist data for notes, tasks, projects in local `.json`.
- **TUI-first Navigation** – Built with `ratatui` and `crossterm`.

---

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/BrandonEssex/prismx
cd prismx
cargo build --release
```

Ensure you have the following installed:
- Rust (latest stable)
- WASM32 target for plugin development:
```bash
rustup target add wasm32-wasi
```

---

## Usage

Launch the TUI:

```bash
./target/release/prismx
```

Basic keyboard shortcuts (customizable):
- `Ctrl + Z` – Toggle Zen Mode
- `Ctrl + /` – Open Spotlight
- `Ctrl + D` – Debug overlay (in applicable modes)
- `Arrow Keys` – Navigate
- `Enter` – Submit / Select
- `Esc` – Back / Close overlay

---

## File Structure

```
~/.config/prismx/
├── zen_scratchpad.md
├── config.toml
./data/
├── inbox.json
├── mindmaps.json
├── dashboard_config.json
├── widget_themes.json
./logs/
├── zen_debug.log
├── extension_host.log
├── spotlight.log
```

---

## Plugin Development

Plugins are compiled as WASM modules with a `.prismx-ext` manifest.

### Extension Layout:

```
example-plugin.prismx-ext/
├── plugin.wasm
└── prismx-plugin.json
```

### Manifest Example:

```json
{
  "name": "Focus Timer",
  "description": "Pomodoro-style timer plugin.",
  "author": "BrandonEssex",
  "version": "1.0",
  "entrypoint": "focus::run"
}
```

### Writing Plugins

See `/extension_host/plugin.rs` and `/plugin_api/` (coming soon) for base traits.

---

## Contributing

Pull requests are welcome. For major changes, please open an issue first.

Visit the GitHub repository:  
**[https://github.com/BrandonEssex/prismx](https://github.com/BrandonEssex/prismx)**

---

## License

MIT