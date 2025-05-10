# PrismX

**Version:** `v1.0.0-beta`  
**Mode:** Terminal-based mindmap and project navigation system for engineers, thinkers, and builders.

---

## Overview

PrismX is a modular, offline-first planning system centered on a dynamic mindmap interface. It includes:

- Tree-based node editor (GemX-inspired)
- Secure, encrypted workspaces with Vault support
- Dynamic sidebar system with plugin extensions
- Zettelkasten-style link tracing (MindTrace)
- Comments, bookmarks, breadcrumbs, tags, and shards
- Timeline heatmaps and activity logs
- Command Palette and custom keymap control
- Multiple workspaces, templates, and autosave

---

## Installation

```bash
git clone https://github.com/YourRepo/prismx
cd prismx
cargo build --release
./target/release/prismx
Dependencies:

Rust 1.74+
Terminal with UTF-8 + TUI support (Alacritty, Kitty, Wezterm recommended)
Usage

Enter → Add child node
Tab → Add sibling
Ctrl+X → Save edit
/ → Open Command Palette
Ctrl+B → Bookmark node
Ctrl+. → Toggle sidebar
Ctrl+Alt+1...9 → Load layout profile
Ctrl+W → Switch workspace
Ctrl+R → Reload plugins
Ctrl+/ → Toggle comments
Ctrl+L → Lock/unlock node
>export template my_template → Save current workspace as reusable config
Plugin System

Plugins can:

Inject sidebar panels
Register their own shortcuts
Read/write app state
Hook into layouts, comments, and link logic
Each plugin must define plugin.toml and be enabled in config/plugins.toml.

Workspaces & Templates

Each workspace is fully isolated:

Mindmap
Vault state
Plugin config
Sidebar layout
Focus + scroll state
Use CmdPalette:

>create workspace design_2025
>switch workspace main
>export template sprint_kit
Encryption

Enable in config/prismx.toml:

[encryption]
enabled = true
key_file = "config/secret.key"
You can also define:

Vault password
Node locking
Per-node encryption with autosave
AI Assistant

If enabled (via plugin), AI features include:

>summarize node
>suggest links
>ask AI <question>
Plugins can implement AiAssistant trait.

File Structure

├── src/
│   ├── app.rs
│   ├── screen.rs
│   ├── config.rs
│   ├── ...
├── plugins/
│   ├── mindtrace/
│   └── ...
├── config/
│   ├── prismx.toml
│   ├── plugins.toml
│   └── layouts.toml
├── workspaces/
│   └── main/
└── templates/
Recommended Terminal

Wezterm or Alacritty with font supporting box-drawing and block symbols
Monospaced font with ligature support for comment rendering (e.g., Fira Code)
Authors & License

Originally inspired by void-rs.
Maintained by @BrandonEssex.
Licensed under MIT.

Contribute

Want to build a plugin or UI mode?

See /examples/ for plugin stubs
Open an issue or send a PR
Fork and define your own TUI layout!