#!/bin/bash
set -e

patches=()
while IFS= read -r patch; do
  [ -z "$patch" ] && continue
  if scripts/ci/should-run-patch.sh "$patch" >/dev/null 2>&1; then
    patches+=("$patch")
  fi
done

printf "%s\n" "${patches[@]}" | jq -R -s -c 'split("\n") | map(select(length > 0))'
