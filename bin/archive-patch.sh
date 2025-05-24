#!/bin/bash
set -e

patch_dir="\$1"
[ -z "\$patch_dir" ] && echo "Usage: bin/archive-patch.sh patch-folder-name" && exit 1
# Remove macOS and editor junk before zipping
find patches/\$patch_dir -name .DS_Store -delete
find patches/\$patch_dir \(
    -name "*.swp" -o -name "*.swo" -o -name "*.tmp" -o -name "*.log" -o -name "*.orig"\
\) -delete
zip -r patches/\$patch_dir.zip patches/\$patch_dir
echo "✅ Archived as patches/\$patch_dir.zip"
