#!/bin/bash
set -euo pipefail

for dir in patches/patch-*; do
  patch_name=$(basename "$dir")
  test_plan="$dir/test_plan.sh"
  if [[ ! -f "$test_plan" ]]; then
    echo "❌ $patch_name – missing test_plan.sh"
    continue
  fi

  # file exists
  if [[ ! -s "$test_plan" ]]; then
    echo "⚠️ $patch_name – placeholder only"
    continue
  fi

  first_line=$(head -n 1 "$test_plan" | tr -d '\r')
  second_line=$(sed -n '2p' "$test_plan" | tr -d '\r')
  line_count=$(wc -l < "$test_plan")

  if [[ "$first_line" == "#!/bin/bash" && "$second_line" == "exit 0" && $line_count -le 2 ]]; then
    echo "⚠️ $patch_name – placeholder only"
  else
    echo "✅ $patch_name – test plan present"
  fi

done
