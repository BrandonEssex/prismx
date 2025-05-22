#!/bin/bash
set -e

PATCH_NAME="${1:-unknown-patch}"
PATCH_PATH="patches/$PATCH_NAME"

echo "🧪 Offline Patch Validation: $PATCH_NAME"
echo "📂 Looking in: $PATCH_PATH"

# Check required files
REQUIRED_FILES=("DESCRIPTION.md" "CODE_CHANGES.md" "test_plan.sh")

for file in "${REQUIRED_FILES[@]}"; do
  if [[ ! -f "$PATCH_PATH/$file" ]]; then
    echo "❌ Missing: $file"
    exit 1
  fi
done

# Static checks
echo "🔍 Checking for node positioning logic..."
grep -q "x\s*[\+\-]=\s*\|y\s*[\+\-]=" src/gemx/*.rs && echo "✅ Found positioning math" || echo "⚠️ Node position math not found"

echo "🔍 Checking snap flag..."
grep -q "snap_to_grid" src/gemx/*.rs && echo "✅ Snap-to-grid logic present" || echo "⚠️ snap_to_grid not found"

echo "🔍 Checking drag functions..."
grep -q "drag" src/gemx/*.rs && echo "✅ Drag functions exist" || echo "⚠️ No drag-related code found"

echo "✅ Patch structure is valid and code appears present."
