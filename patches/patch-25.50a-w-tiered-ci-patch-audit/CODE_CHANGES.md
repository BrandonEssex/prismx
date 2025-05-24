## Code Changes

- Created `.github/workflows/prismx-patch-tests.yml`
  - Only runs the patch triggered in the PR (via matrix)

- Created `.github/workflows/prismx-regression-audit.yml`
  - Runs all `patch-*` folders except those in `patch-ignore.txt`
  - Can be triggered manually or nightly

- Added:
  - `patch-ignore.txt`
    (e.g. legacy patches like patch-25.43-link-arrows-mac-scroll)
  - `bin/generate-regression-matrix.sh`
    Scans `patches/`, excludes ignored, outputs JSON matrix
