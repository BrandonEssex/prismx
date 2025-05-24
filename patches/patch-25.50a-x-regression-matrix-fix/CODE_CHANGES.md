## Code Changes

- Updated `bin/generate-regression-matrix.sh`:

```bash
echo "matrix=$matrix" >> "$GITHUB_OUTPUT"
Removed unsafe heredoc block:
echo "matrix<<EOF" >> ...
