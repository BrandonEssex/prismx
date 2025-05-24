## Code Changes

- In `bin/generate-regression-matrix.sh`, replace final line with:
```bash
echo "matrix<<EOF" >> "$GITHUB_OUTPUT"
echo "$matrix" >> "$GITHUB_OUTPUT"
echo "EOF" >> "$GITHUB_OUTPUT"
In .github/workflows/prismx-regression-audit.yml, update matrix step:
- name: 🧮 Generate Patch Matrix
  id: matrix
  run: |
    chmod +x ./bin/generate-regression-matrix.sh
    ./bin/generate-regression-matrix.sh
Ensure next job references matrix with:
matrix:
  patch: ${{ fromJson(needs.matrix.outputs.matrix) }}
