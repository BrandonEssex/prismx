#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-d Test Plan: Subtree Packing"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic patched"

echo "⚠️ Add wide subtrees to leftmost children"
echo "⚠️ Confirm layout spreads naturally without collision"
echo "⚠️ Confirm parent remains centered across siblings"

