#!/bin/zsh
set -e
echo "🧪 Patch 25.45x Test Plan: Zoom-to-Cursor"

grep -q "zoom_scale" src/state/mod.rs && echo "✅ zoom_scale present"
grep -q "zoom_to_anchor" src/layout.rs && echo "✅ zoom centering logic defined"

echo "⚠️ Zoom in/out on selected node and verify it stays centered"
echo "⚠️ Drag at various zoom levels and confirm logical movement is preserved"

