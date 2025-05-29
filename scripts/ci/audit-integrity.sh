#!/bin/bash
set -e

passed=true

printf "\n🔍 Auditing PrismX patch completeness...\n"

# 1. init_logger check
if grep -q "init_logger()" src/main.rs src/bootstrap.rs; then
  echo "✅ init_logger wired"
else
  echo "❌ init_logger() call missing"
  passed=false
fi

# 2. load_plugins check
if grep -q -E "\bload_plugins\(" src/state/init.rs; then
  echo "✅ load_plugins wired"
else
  echo "❌ load_plugins call missing"
  passed=false
fi

# 3. zen editor toggle or /scroll
if grep -q "toggle_zen_view" src/zen/editor.rs || grep -q "/scroll" src/zen/editor.rs; then
  echo "✅ zen editor toggle present"
else
  echo "❌ toggle_zen_view or /scroll missing"
  passed=false
fi

# 4. ZenViewMode and ZenLayoutMode usage
if grep -R "ZenViewMode" src >/dev/null && grep -R "ZenLayoutMode" src >/dev/null; then
  echo "✅ Zen modes detected"
else
  echo "❌ Zen mode usage missing"
  passed=false
fi

# 5. removed fields check
if grep -R "entry\.entry" src | grep -v "src/bin/audit.rs" >/dev/null; then
  echo "❌ old entry.entry reference found"
  passed=false
else
  echo "✅ no old entry.entry references"
fi

if [ "$passed" = true ]; then
  echo "\n✅ Audit complete"
else
  echo "\n❌ Audit completed with issues"
  exit 1
fi
