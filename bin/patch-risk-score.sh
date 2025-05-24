#!/bin/bash
set -e

PATCH="$1"
[ -z "$PATCH" ] && echo "Usage: bin/patch-risk-score.sh patch-dir" && exit 1

DIR="patches/$PATCH"
if [ ! -d "$DIR" ]; then
  echo "❌ Patch directory not found: $PATCH" >&2
  exit 1
fi

cd "$DIR"

RISK_FILES=(src/beamx.rs src/state.rs src/plugin.rs src/app.rs)
RISK_HIT=0
for file in "${RISK_FILES[@]}"; do
  if grep -q "$file" CODE_CHANGES.md 2>/dev/null; then
    echo "❌ High-risk file modified: $file"
    RISK_HIT=1
  fi
done

if [ "$RISK_HIT" -eq 1 ]; then
  PLAN=$(cat test_plan.sh 2>/dev/null || true)
  if ! echo "$PLAN" | grep -qiE "plugin|beam|render|triage"; then
    echo "⚠️ Test plan does not mention \"plugin\", \"beam\", \"render\", or \"triage\""
    exit 1
  fi
fi

echo "✅ Patch risk score check passed"
