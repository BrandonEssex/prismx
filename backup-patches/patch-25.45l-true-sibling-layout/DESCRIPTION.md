# Patch 25.45l – True Sibling Layout (Void-RS Style)

## Goals
- Eliminate horizontal sibling collisions
- Center siblings horizontally around parent
- Anchor each sibling with horizontal spacing: `parent.x ± offset`
- Stack children vertically beneath their parent
- Use layout_nodes() for all auto-arrange logic
- Remove fallback sibling layout from spawn logic

## Files
- src/layout.rs (primary)
- src/gemx/interaction.rs (verify drill/spawn respects layout)
- src/state/mod.rs (confirm auto_arrange disables layout override)

