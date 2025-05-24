#!/bin/bash
echo "🧪 Checking if this PR touches a patch folder:"
git diff --name-only origin/main...HEAD | grep '^patches/patch-' && echo "✅ Patch change detected" && exit 0
echo "🛑 No patch folder changes found. Patch-test CI not required."
exit 1
