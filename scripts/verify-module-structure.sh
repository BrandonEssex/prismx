#!/bin/bash
set -e

SRC_DIR="src"
RED="\e[31m"
GREEN="\e[32m"
RESET="\e[0m"

status=0

# Check each top-level directory in src/
for dir in "$SRC_DIR"/*/; do
  [ -d "$dir" ] || continue
  module=$(basename "$dir")
  mod_file="$dir/mod.rs"
  if [ ! -f "$mod_file" ]; then
    echo -e "${RED}${module} missing mod.rs${RESET}"
    status=1
    continue
  fi

  found=false
  for f in view.rs state.rs input.rs ui.rs; do
    if [ -f "$dir/$f" ]; then
      found=true
      break
    fi
  done
  if ! $found; then
    echo -e "${RED}${module} missing view.rs/state.rs/input.rs/ui.rs${RESET}"
    status=1
  else
    echo -e "${GREEN}${module} structure OK${RESET}"
  fi
done

# Determine enum file (AppState or Module)
enum_file=$(grep -rl "enum AppState" "$SRC_DIR" 2>/dev/null | head -n 1)
enum_name="AppState"
if [ -z "$enum_file" ]; then
  enum_file=$(grep -rl "enum Module" "$SRC_DIR" 2>/dev/null | head -n 1)
  enum_name="Module"
fi

if [ -n "$enum_file" ]; then
  variants=$(awk '/enum '"$enum_name"'/ {flag=1;next} /}/ {flag=flag&&0} flag {gsub(/,/, ""); print}' "$enum_file")
  for var in $variants; do
    name=$(echo "$var" | tr 'A-Z' 'a-z')
    if [ ! -d "$SRC_DIR/$name" ]; then
      echo -e "${RED}Enum variant $var has no matching directory $SRC_DIR/$name${RESET}"
      status=1
    else
      echo -e "${GREEN}Enum variant $var matches directory $name${RESET}"
    fi
  done
else
  echo -e "${RED}AppState enum not found${RESET}"
  status=1
fi

# Validate spotlight module references
spotlight_file="src/state/spotlight.rs"
if [ -f "$spotlight_file" ]; then
  mods=$(grep -Po '"[a-zA-Z0-9_]+" => self.mode' "$spotlight_file" | cut -d'"' -f2 | sort -u)
  for m in $mods; do
    if [ ! -d "$SRC_DIR/$m" ]; then
      echo -e "${RED}Spotlight references missing module: $m${RESET}"
      status=1
    else
      echo -e "${GREEN}Spotlight module $m exists${RESET}"
    fi
  done
fi

exit $status
