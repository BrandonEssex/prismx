## Code Changes

- Updated `layout_nodes()`:
  - Increment sibling `x` on each insert
  - Use `BASE_SPACING_X` when laying out same-level children

- `PackRegion::insert()`:
  - Adjusted spacing buffer between packed clusters

- Modified child placement (Tab):
  - New child node y-offsets from parent
  - Initial position no longer identical to parent
