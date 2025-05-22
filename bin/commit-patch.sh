#!/bin/bash
set -e

# ✅ Usage:
# ./bin/commit-patch.sh 25.43 "Link Arrows and Mac Scroll Fallback"

PATCH_NUM="$1"
PATCH_MSG="$2"

if [[ -z "$PATCH_NUM" || -z "$PATCH_MSG" ]]; then
  echo "❌ Usage: $0 <patch_number> <commit_message>"
  echo "Example: $0 25.43 \"Link Arrows and Mac Scroll Fallback\""
  exit 1
fi

BRANCH="patch/${PATCH_NUM// /-}" # sanitize spacing
FULL_MSG="Patch $PATCH_NUM: $PATCH_MSG"

echo "🔍 Checking for changes..."
if [[ -z "$(git status --porcelain)" ]]; then
  echo "✅ No changes to commit."
  exit 0
fi

echo "📦 Staging and committing changes..."
git add .
git commit -m "$FULL_MSG"

echo "🌿 Creating and pushing branch: $BRANCH"
git checkout -b "$BRANCH"
git push -u origin "$BRANCH"

echo "✅ Patch $PATCH_NUM committed and pushed to $BRANCH"
