#!/bin/bash
set -e

echo "🧪 PrismX Regression Check Suite"
echo "-------------------------------"

declare -A checks=(
  ["ZoomIn (Ctrl+Alt+Z)"]="grep -r \"Ctrl.+Alt.+Z\" src/shortcuts.rs"
  ["ZoomOut (Ctrl+Alt+X)"]="grep -r \"Ctrl.+Alt.+X\" src/shortcuts.rs"
  ["ZoomReset (Ctrl+Alt+0)"]="grep -r \"Ctrl.+Alt.+0\" src/shortcuts.rs"
  ["Auto-Arrange Toggle"]="grep -r \"Ctrl.+P\" src/shortcuts.rs"
  ["Add Free Node"]="grep -r \"Ctrl.+N\" src/shortcuts.rs"
  ["Drill Down"]="grep -r \"Ctrl.+W\" src/shortcuts.rs"
  ["Toggle Debug Input"]="grep -r \"ToggleDebugInput\" src/shortcuts.rs"
  ["Status Bar Logging"]="grep -r \"status_message\" src/state.rs"
  ["Drag Recursive"]="grep -r \"drag_recursive\" src/gemx/"
  ["Zoom Scale Applied"]="grep -r \"zoom_scale\" src/gemx/render.rs"
)

failures=0

for label in "${!checks[@]}"; do
  echo -n "🔍 $label... "
  if eval "${checks[$label]}" > /dev/null; then
    echo "✅"
  else
    echo "❌ MISSING"
    failures=$((failures + 1))
  fi
done

echo "-------------------------------"

if [[ "$failures" -gt 0 ]]; then
  echo "❌ $failures regression issue(s) detected!"
  exit 1
else
  echo "✅ All static regression checks passed!"
fi
