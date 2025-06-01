#!/bin/bash
set -e

# Determine toggle source file
TOGGLE_FILE="src/settings/toggles.rs"
if [ ! -f "$TOGGLE_FILE" ]; then
  TOGGLE_FILE="src/settings/toggle.rs"
fi

if [ ! -f "$TOGGLE_FILE" ]; then
  echo "Toggle file not found" >&2
  exit 1
fi

# Minimum toggle count if provided
MIN_COUNT=${MIN_TOGGLE_COUNT:-0}
TOGGLE_COUNT=$(grep -c "SettingToggle" "$TOGGLE_FILE" || echo 0)
if [ "$MIN_COUNT" -gt 0 ] && [ "$TOGGLE_COUNT" -lt "$MIN_COUNT" ]; then
  echo "Found $TOGGLE_COUNT toggles, expected at least $MIN_COUNT" >&2
  exit 1
fi

# Extract referenced state fields from toggle definitions
FIELDS=$(grep -o '\b\(s\|state\)\.[A-Za-z_][A-Za-z0-9_]*\b' "$TOGGLE_FILE" | sed 's/.*\.//' | sort -u)

unused=()
for field in $FIELDS; do
  count=$(grep -R "$field" src | grep -v "$TOGGLE_FILE" | wc -l)
  if [ "$count" -gt 0 ]; then
    echo "✅ $field"
  else
    echo "❌ $field"
    unused+=("$field")
  fi
done

if [ "${#unused[@]}" -ne 0 ]; then
  echo "Unused toggles: ${unused[*]}" >&2
  exit 1
fi

echo "All toggles in use"
