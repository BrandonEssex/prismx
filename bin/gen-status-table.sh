#!/bin/bash
set -e

OUT="STATUS_TABLE.md"

echo "| Patch ID     | Title                     | Status  |" > "$OUT"
echo "|--------------|---------------------------|---------|" >> "$OUT"

for file in patches/*/PATCH_SUMMARY.md; do
  [ -f "$file" ] || continue
  patch_id=$(basename "$(dirname "$file")")
  title=$(grep -m1 '^## Title' -A1 "$file" | tail -n1 | tr -d '\r')
  status=$(grep -m1 '^## Status' -A1 "$file" | tail -n1 | tr -d '\r')
  [ -z "$status" ] && status="✅ OK"
  echo "| $patch_id | $title | $status |" >> "$OUT"
done

echo "✅ STATUS_TABLE.md generated."
