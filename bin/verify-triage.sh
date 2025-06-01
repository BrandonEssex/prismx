#!/bin/bash
set -e

# Simulated triage feed verification

feed=$(mktemp)
cat <<EOT > "$feed"
[tag-a] entry1
[tag-b] entry2
[tag-c] entry3
EOT

filter="Filter: ALL"

if echo "$filter" | grep -q "Filter: ALL" && [ $(grep -c '\[tag-' "$feed") -eq 3 ]; then
  echo "TRIAGE_OK"
  exit 0
else
  echo "TRIAGE_FAIL"
  exit 1
fi
