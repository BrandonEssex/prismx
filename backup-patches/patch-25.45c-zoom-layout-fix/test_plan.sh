#!/bin/bash
set -e

echo "🧪 Patch 25.45c Test Plan: Zoom Layout Collision Fix"

grep -q "BASE_SPACING_X" src/gemx/*.rs && echo "✅ X spacing constant defined"
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ zoom scale used in layout"
grep -q "node.label.len()" src/gemx/render.rs && echo "✅ Label-aware spacing logic found"

echo "⚠️ Manually zoom in/out and verify nodes do not overlap or tangle"
echo "✅ Static test complete"

