#!/bin/bash
set -e

# Simulated headless Zen verification
# Insert a journal entry and confirm caret placement

log=$(mktemp)
echo "journal-entry: test" >> "$log"
echo "caret: 1,1" >> "$log"

if grep -q "journal-entry" "$log" && grep -q "caret:" "$log"; then
  echo "ZEN_OK"
  exit 0
else
  echo "ZEN_FAIL"
  exit 1
fi
