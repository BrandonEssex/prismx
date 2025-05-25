# Patch 25.51u-n – Aggressive Root Lock + Direct Draw Enforcement

## Goals
- Force root-promoted nodes to render, even if they are isolated
- Guarantee root layout role is assigned
- Skip unreachable fallback on already visible or promoted nodes
- Directly patch any failure in node render visibility
