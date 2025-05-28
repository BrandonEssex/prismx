#!/bin/zsh
set -e
echo "🧪 Patch 25.45n Test Plan: Subtree-Aware Sibling Layout"
grep -q "get_subtree_span" src/layout.rs && echo "✅ Subtree span logic present"
echo "⚠️ Visually confirm siblings do not overlap under long labels"
