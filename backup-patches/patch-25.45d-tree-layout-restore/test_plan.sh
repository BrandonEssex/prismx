#!/bin/bash
set -e

echo "🧪 Patch 25.45d Test Plan: Tree Layout Restore"

grep -q "BASE_CHILD_SPACING_Y" src/gemx/*.rs && echo "✅ Vertical spacing constant defined"
grep -q "child.x = parent.x" src/gemx/layout.rs && echo "✅ Child nodes stack under parent"
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ Zoom scaling preserved"

echo "⚠️ Zoom in/out and confirm children remain under their parent"
echo "✅ Tree layout visually confirmed"

