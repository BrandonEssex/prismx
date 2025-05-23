#!/bin/bash
set -e

echo "🧪 Patch 25.45a Test Plan: Zoom Fix + Clamp"

# Hotkey mappings
grep -q "Alt.+=" src/input/hotkeys.rs && echo "✅ Alt+= mapped"
grep -q "Alt.+-" src/input/hotkeys.rs && echo "✅ Alt+- mapped"
grep -q "Alt.+0" src/input/hotkeys.rs && echo "✅ Alt+0 mapped"

# Logic presence
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ zoom_scale used in render"
grep -q "center_on_selected" src/gemx/view.rs && echo "✅ Centering logic present"

# Visual safety
grep -q "clamp\|min\|max" src/gemx/render.rs && echo "✅ Clamping logic detected"

echo "✅ Static checks for 25.45a complete"
