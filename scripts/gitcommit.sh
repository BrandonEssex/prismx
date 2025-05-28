#!/bin/bash

# Git + Build Runner for PrismX
# Author: Brandon Essex
# Version: 1.0.0
# License: MIT

set -euo pipefail

# Ensure a commit message is passed
if [ $# -eq 0 ]; then
  echo "❌ Error: Commit message required."
  echo "Usage: ./gitcommit.sh \"v2.4.0-beta5: RefractPack-v1.0.0\""
  exit 1
fi

COMMIT_MSG="$1"

# Step 1: Git Operations
echo "🔃 Staging and committing changes..."
git add .
git commit -m "$COMMIT_MSG"
git push

