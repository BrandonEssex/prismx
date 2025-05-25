# Patch 25.51u-t – Layout Trace Cleanup + Debug Reset

## Goals
- Remove verbose debug output (e.g. [INSERT], [RENDER TREE])
- Strip fallback `[F]` labels from nodes
- Optionally retain minimal draw summary in debug mode
- Finalize layout engine for snapshotting (25.51v)
