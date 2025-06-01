#!/bin/bash
set -e

# Simulated dock alignment verification

output="$(printf '%80s' '')GemX"

if echo "$output" | grep -E ".{0,12}(GemX|Zen)$" >/dev/null; then
  echo "DOCK_OK"
  exit 0
else
  echo "DOCK_FAIL"
  exit 1
fi
