#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-i Test Plan: LayoutRole Assignment"

grep -q "LayoutRole" src/layout.rs && echo "✅ Role enum defined"
echo "⚠️ Assign roles to nodes in layout pass"
echo "⚠️ Confirm free nodes get LayoutRole::Free"
echo "⚠️ Confirm tree roots get LayoutRole::Root"
echo "⚠️ Confirm nodes with missing parents get LayoutRole::Orphan"
