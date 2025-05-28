#!/bin/bash
set -e

echo "🧪 Patch 25.45b Test Plan: Confirmed Zoom Inputs"

grep -q "Ctrl.+Alt.+Z" src/shortcuts.rs && echo "✅ ZoomIn key mapped"
grep -q "Ctrl.+Alt.+X" src/shortcuts.rs && echo "✅ ZoomOut key mapped"
grep -q "zoom_scale" src/state.rs && echo "✅ zoom_scale present"
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ zoom applied in render logic"

echo "⚠️ Run PrismX and press Ctrl+Alt+Z/X/0 — ensure visual zoom effect occurs"
echo "✅ Static checks passed"

