# Patch 25.45n-d – Subtree Packing (Span-Aware Child Offset)

## Goals
- Prevent overlap from long or uneven subtrees
- Allocate horizontal space per child based on subtree span
- Accumulate width as children are laid out
- Center the final layout block under the parent

## Dependencies
- Builds on 25.45n, 25.45n-b, 25.45n-c

