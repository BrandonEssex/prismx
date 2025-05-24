# Patch 25.50a-y – Regression Matrix Output Fix (GitHub EOF Format)

## Goals
- Fix `$GITHUB_OUTPUT` formatting for GitHub Actions
- Use EOF-style multi-line output to pass full JSON matrix
- Avoid quote parsing and newlines triggering GitHub CI failure
