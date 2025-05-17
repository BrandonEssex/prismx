# PrismX Cheat Sheet – VIM Mode (v10.1.0+Final)

---

## 🧠 Normal Mode

| Key       | Action                       |
|-----------|------------------------------|
| `j`       | Move down                    |
| `k`       | Move up                      |
| `l`       | Move to child node           |
| `h`       | Move to parent node          |
| `o`       | Create new sibling node      |
| `O`       | Create new root node         |
| `a`       | Append to current node       |
| `dd`      | Delete node                  |
| `yy`      | Copy node                    |
| `p`       | Paste node                   |
| `:`       | Open command mode (Spotlight)|

---

## ✍ Insert Mode (Edit Node)

| Key       | Action                       |
|-----------|------------------------------|
| `i`       | Insert at start              |
| `a`       | Insert at end                |
| `Esc`     | Exit to Normal Mode          |
| `Ctrl + E`| Export Zen Journal           |

---

## 🧘 Zen Mode

| Key Combo     | Action                   |
|---------------|--------------------------|
| `zz`          | Toggle Zen Mode          |
| `zf`          | Switch Zen Focus Profile |
| `zp`          | Switch Zen Personal Mode |

---

## 📊 UI / System

| Key Combo     | Action                   |
|---------------|--------------------------|
| `:tags`       | Tag viewer               |
| `:dash`       | Rearrange dashboard      |
| `:reload`     | Reload theme/config      |
| `:q`          | Quit PrismX              |

---

## 🔌 Plugins (VIM Mode)

| Key            | Plugin Action           |
|----------------|--------------------------|
| `gp`           | Launch GemDrop plugin   |
| `gb`           | Bookmark current node   |
| `gk`           | Show plugin overlay     |

---

## 🔍 Spotlight & Command

| Key            | Action                  |
|----------------|--------------------------|
| `/`            | Open Spotlight           |
| `:`            | Command Mode             |
| `Enter`        | Run or Jump              |
| `Esc`          | Exit                     |

---

## 🧩 Developer Mode

| Key           | Action                   |
|---------------|---------------------------|
| `:dump`       | Dump JSON shard           |
| `:aether`     | Open override log         |
| `:debug`      | Toggle runtime metrics    |

---

## ⚙️ Switching Modes

Edit `config/keymap.rs` and toggle `vim_mode = true`

You can bind modal switches as well:

```rust
bind_mode("vim", default=true);
bind_mode("ctrl", fallback=true);
