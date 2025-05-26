## Code Changes

- Adds `automerge-when-ready` label trigger (in addition to `automerge`)
- Logs PR number, label, status, and branch before attempting merge
- Adds fallback check to allow skipping merge if WIP/draft
- Auto-merge now works with Codex-created or ChatGPT-tagged PRs
