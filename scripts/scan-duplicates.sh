#!/bin/bash
# Diagnostic script to find duplicate public items
# Scans for:
#   - Duplicate `pub fn` definitions across modules
#   - Conflicting `pub mod` declarations
#   - Redundant `pub use` re-exports in mod.rs files

set -euo pipefail

ROOT_DIR=${1:-src}

if command -v rg >/dev/null 2>&1; then
    SEARCH="rg -n --no-heading"
else
    SEARCH="grep -Rn"
fi

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

# helper to process duplicates
process_dups() {
    local input=$1
    local message=$2
    local item

    cut -d' ' -f1 "$input" | uniq -d | sort | while read -r item; do
        [ -z "$item" ] && continue
        echo "$message $item found in:"
        grep "^$item " "$input" | cut -d' ' -f2- | sort
        echo
    done
}

# Collect public function definitions
$SEARCH -e "^\s*pub fn\s+[A-Za-z0-9_]+" "$ROOT_DIR" \
    | sed -E 's/^([^:]+):[0-9]+:.*pub fn ([A-Za-z0-9_]+).*/\2 \1/' \
    | sort > "$TMP_DIR/pub_fns.txt"
process_dups "$TMP_DIR/pub_fns.txt" "DUPLICATE pub fn"

# Collect pub mod declarations
$SEARCH -e "^\s*pub mod\s+[A-Za-z0-9_]+" "$ROOT_DIR" \
    | sed -E 's/^([^:]+):[0-9]+:.*pub mod ([A-Za-z0-9_]+).*/\2 \1/' \
    | sort > "$TMP_DIR/pub_mods.txt"
process_dups "$TMP_DIR/pub_mods.txt" "CONFLICTING pub mod"

# Collect pub use statements (re-exports)
$SEARCH -e "^\s*pub use .*::([A-Za-z0-9_]+)" "$ROOT_DIR" \
    | sed -E 's/^([^:]+):[0-9]+:.*::([A-Za-z0-9_]+).*/\2 \1/' \
    | sort > "$TMP_DIR/pub_use.txt"

# Only look at mod.rs files for redundant re-exports
grep 'mod.rs' "$TMP_DIR/pub_use.txt" > "$TMP_DIR/mod_reexports.txt" || true
process_dups "$TMP_DIR/mod_reexports.txt" "REDUNDANT pub use"

