# Patch 25.51s – Safe Tree Injection + Root Fallback

## Goals
- Prevent layout_nodes() from returning no visible nodes
- Ensure child nodes are properly reachable from root
- Add fallback for orphaned trees
- Show visual warning if layout fails
