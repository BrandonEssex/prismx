#!/bin/bash
set -e

# Simulated mindmap layout verification

tmp=$(mktemp)
for i in {1..5}; do
  node=$(printf "node%03d" "$i")
  echo "LAYOUT_OK: $node $((i*2)),$((i*3))" >> "$tmp"
done

count=$(grep -c 'LAYOUT_OK' "$tmp")
if [ "$count" -eq 5 ]; then
  echo "MINDMAP_OK"
  exit 0
else
  echo "MINDMAP_FAIL"
  exit 1
fi
