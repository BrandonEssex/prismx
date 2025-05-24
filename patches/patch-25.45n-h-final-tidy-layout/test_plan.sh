#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-h Test Plan: Final Tidy Layout Fix"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic defined"

echo "⚠️ Add a long chain of single-child nodes"
echo "⚠️ Confirm vertical stack with no X drift"
echo "⚠️ Add multiple branches and confirm horizontal fan-out is centered"
echo "⚠️ Scroll left — confirm no negative X clipping"
