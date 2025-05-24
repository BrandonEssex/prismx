#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-g Test Plan: Horizontal Line for Siblings"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic found"

echo "⚠️ Add 5+ siblings under one node"
echo "⚠️ Confirm they appear on the same row (Y)"
echo "⚠️ Confirm children of those siblings go one row deeper"

