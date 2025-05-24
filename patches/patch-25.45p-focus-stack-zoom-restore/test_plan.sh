#!/bin/zsh
set -e
echo "🧪 Patch 25.45p Test Plan: Focus Stack + Zoom Context"
grep -q "focus_stack" src/state/mod.rs && echo "✅ focus_stack field exists"
echo "⚠️ Drill in/out and confirm scroll/selection is preserved"
