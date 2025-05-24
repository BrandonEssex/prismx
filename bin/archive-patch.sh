#!/bin/bash
set -e

patch_dir="\$1"
[ -z "\$patch_dir" ] && echo "Usage: bin/archive-patch.sh patch-folder-name" && exit 1
zip -r patches/\$patch_dir.zip patches/\$patch_dir
echo "✅ Archived as patches/\$patch_dir.zip"
