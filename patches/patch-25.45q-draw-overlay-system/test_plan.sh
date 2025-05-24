#!/bin/zsh
set -e
echo "🧪 Patch 25.45q Test Plan: Overlay Drawing System"
grep -q "draw_links" src/tui/mod.rs && echo "✅ draw_links exists"
grep -q "draw_ephemeral" src/tui/mod.rs && echo "✅ draw_ephemeral exists"
echo "⚠️ Visually confirm arrows and highlights do not affect layout"
