#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-i Test Plan: Root Cluster Packing"

grep -q "PackRegion" src/layout.rs && echo "✅ Pack system added"

echo "⚠️ Add 3+ root nodes"
echo "⚠️ Confirm they’re spaced in grid pattern"
echo "⚠️ Confirm no roots overlap"
echo "⚠️ Confirm layout does not clip off screen"
