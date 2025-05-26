#!/bin/bash
set -e

# Use GITHUB_TOKEN for gh CLI
export GH_TOKEN="${GH_TOKEN:-$GITHUB_TOKEN}"

# Read PR metadata from event payload
EVENT_PATH="${GITHUB_EVENT_PATH}"
PR_NUMBER="${PR_NUMBER:-$(jq -r '.pull_request.number' "$EVENT_PATH" 2>/dev/null)}"
TITLE="${PR_TITLE:-$(jq -r '.pull_request.title' "$EVENT_PATH" 2>/dev/null)}"
DRAFT="${PR_DRAFT:-$(jq -r '.pull_request.draft' "$EVENT_PATH" 2>/dev/null)}"
HEAD_REF="${HEAD_REF:-$(jq -r '.pull_request.head.ref' "$EVENT_PATH" 2>/dev/null)}"
LABELS_JSON="${PR_LABELS:-$(jq -r '[.pull_request.labels[].name]' "$EVENT_PATH" 2>/dev/null)}"

# Normalize to empty array if null
if [ "$LABELS_JSON" = "null" ] || [ -z "$LABELS_JSON" ]; then
  LABELS_JSON="[]"
fi

echo "PR #$PR_NUMBER: $TITLE"
echo "Branch: $HEAD_REF"
echo "Labels: $LABELS_JSON"

# Skip draft or WIP PRs
if [ "$DRAFT" = "true" ]; then
  echo "Skipping: draft PR"
  exit 0
fi
if echo "$TITLE" | grep -iq '\bWIP\b'; then
  echo "Skipping: WIP PR"
  exit 0
fi

labels=$(echo "$LABELS_JSON" | jq -r '.[]')
automerge=false
for lbl in $labels; do
  norm=$(echo "$lbl" | tr 'A-Z' 'a-z' | tr -d '-')
  if [[ "$norm" == "automerge" || "$norm" == "automergewhenready" ]]; then
    automerge=true
    break
  fi
done

if [ "$automerge" != true ]; then
  echo "No automerge label found"
  exit 0
fi

merge_status="unknown"
if gh pr merge "$PR_NUMBER" --squash --auto --repo "$GITHUB_REPOSITORY"; then
  merge_status="success"
else
  merge_status="failed"
fi

echo "Merge status: $merge_status"
