#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-c Test Plan: Span Return & Anchor Fix"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic patch point located"

echo "⚠️ Add multiple child branches of varying width/depth"
echo "⚠️ Confirm parent stays centered and siblings are symmetric"
echo "⚠️ No overlapping labels or collapsed layouts"

