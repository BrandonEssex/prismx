# Patch 25.50a-x – Regression Matrix Output Fix

## Goals
- Resolve GitHub Actions error when writing matrix JSON to $GITHUB_OUTPUT
- Use correct format:
  echo "matrix=<json>" >> $GITHUB_OUTPUT
- Remove broken heredoc and multiline techniques
