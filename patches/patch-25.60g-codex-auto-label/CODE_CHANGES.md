## Code Changes

- Adds workflow to trigger on `pull_request.opened`
- Detects PRs with author `ChatGPT Connector` or branch prefix `codex/`
- Automatically applies `automerge-when-ready` label
- Skips PRs already labeled manually
