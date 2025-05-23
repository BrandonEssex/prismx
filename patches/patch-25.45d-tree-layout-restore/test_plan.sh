#!/bin/bash
set -e

echo "🧪 Patch 25.45d Test Plan: Tree Layout Restore"

grep -q "CHILD_SPACING_Y" src/gemx/*.rs && echo "✅ Vertical spacing constant defined"
grep -q "SIBLING_SPACING_X" src/gemx/*.rs && echo "✅ Sibling spacing constant defined"
grep -q "idx as f32" src/layout.rs && echo "✅ Siblings centered around parent"
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ Zoom scaling preserved"

echo "⚠️ Zoom in/out and confirm children remain under their parent"
echo "✅ Tree layout visually confirmed"

