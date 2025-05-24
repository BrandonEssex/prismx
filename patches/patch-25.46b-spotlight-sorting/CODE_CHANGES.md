## Code Changes

- In `src/spotlight.rs`:
  - Add `score_match(query: &str, label: &str) -> u8`
  - For each SpotlightResult:
    - Compute match score
    - Group by enum variant (Command, Node, Tag, etc.)
  - Sort each group by score descending
  - Rebuild result_list

- In `render_gemx()` or Spotlight draw:
  - Optionally show group headers like `[Commands]`, `[Tags]`

## Visual Option:
- Highlight top result with bold/yellow
- Underline match term in result string (optional)
