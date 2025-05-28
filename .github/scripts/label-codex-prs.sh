#!/bin/bash
set -e

pr_number="$1"
label="codex-pr"

info=$(gh pr view "$pr_number" --json headRefName,author,labels)
author=$(echo "$info" | jq -r '.author.login')
head_ref=$(echo "$info" | jq -r '.headRefName')
labels=$(echo "$info" | jq -r '.labels[].name')

echo "PR #$pr_number from $author on $head_ref"

# Create label if it doesn't exist
if ! gh label list --limit 100 | grep -q "^$label"; then
  echo "Label '$label' not found — creating it..."
  gh label create "$label" --description "Auto-applied to Codex-generated PRs" --color "1D76DB"
fi

# Apply if not present
if echo "$labels" | grep -q "^$label$"; then
  echo "Label already present; skipping"
  exit 0
fi

if [[ "$author" == "ChatGPT Connector" || "$head_ref" == codex/* ]]; then
  gh pr edit "$pr_number" --add-label "$label"
  echo "Applied label '$label' to PR #$pr_number"
else
  echo "No labeling criteria met"
fi
