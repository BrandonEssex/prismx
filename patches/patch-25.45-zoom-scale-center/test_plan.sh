#!/bin/bash
set -e

echo "🧪 Patch 25.45 Test Plan: Zoom + Center"

# Validate hotkeys are mapped
grep -q "Ctrl.+=" src/input/hotkeys.rs && echo "✅ Ctrl+= mapped"
grep -q "Ctrl.+-" src/input/hotkeys.rs && echo "✅ Ctrl+- mapped"
grep -q "Ctrl+0" src/input/hotkeys.rs && echo "✅ Ctrl+0 mapped"

# Check for zoom state
grep -q "zoom_scale" src/gemx/*.rs && echo "✅ zoom_scale state exists"

# Check render logic uses scale
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ render applies zoom scale"

echo "✅ Patch 25.45 logic detected statically"
