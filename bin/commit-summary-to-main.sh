#!/bin/bash
set -e

BRANCH=$(git rev-parse --abbrev-ref HEAD)
[ "$BRANCH" != "main" ] && echo "❌ You must be on 'main' to archive patch summaries." && exit 1

echo "📋 Scanning for uncommitted PATCH_SUMMARY.md files..."
patches=$(find changelog/patches -name '*.md' -newer .git/index || true)

if [ -z "$patches" ]; then
  echo "✅ No new summaries to commit."
  exit 0
fi

echo "✅ Found new summaries:"
echo "$patches"

echo "🔀 Updating CHANGELOG.md..."
{
  echo "# PrismX Patch Changelog"
  echo
  for file in $(ls changelog/patches/*.md | sort); do
    echo "---"
    cat "$file"
    echo
  done
} > CHANGELOG.md

git add changelog/patches/*.md CHANGELOG.md
git commit -m "changelog: add patch summaries and regenerate master CHANGELOG.md"
git push origin main

echo "✅ Patch summaries committed and changelog updated."
