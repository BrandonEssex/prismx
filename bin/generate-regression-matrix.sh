#!/bin/bash
set -e

echo "🔍 Generating patch matrix excluding ignored patches..."

ignored=$(cat patch-ignore.txt | sort)
all=$(find patches -maxdepth 1 -type d -name 'patch-*' | sed 's|patches/||' | sort)

matrix=$(comm -23 <(echo "$all") <(echo "$ignored") | jq -R -s -c 'split("\n") | map(select(length > 0))')

echo "✅ Included patches:"
echo "$matrix" | jq '.[]'

# Secure GitHub Actions output
echo "matrix=$matrix" >> $GITHUB_OUTPUT
