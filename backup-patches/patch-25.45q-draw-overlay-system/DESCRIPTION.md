# Patch 25.45q – Split Draw System: Nodes + Overlays

## Goals
- Modularize the draw pipeline for better maintenance and layering
- Isolate node drawing, arrows, highlights into separate render passes
- Ensure arrows and ephemeral highlights do not affect layout logic
- Prepare for live visual feedback during drag/link/search
