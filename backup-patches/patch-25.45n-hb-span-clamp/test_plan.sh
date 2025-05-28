#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-hb Test Plan: Clamp + Label-Safe Sibling Span"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic patched"

echo "⚠️ Add narrow siblings — confirm they no longer overlap"
echo "⚠️ Add long-label siblings — spacing adapts"
echo "⚠️ Confirm tree structure still centered and anchored"
