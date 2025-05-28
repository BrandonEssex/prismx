#!/bin/zsh
set -e
echo "🧪 Patch 25.46c Test Plan: Spotlight Hotkey Fix"

grep -q "Alt+Shift+S" src/render/keymap.rs && echo "✅ keymap updated"

grep -q "Alt + Shift + S" README.md && echo "✅ docs updated"

grep -q "show_spotlight" src/tui/mod.rs && echo "✅ toggle logic present"
