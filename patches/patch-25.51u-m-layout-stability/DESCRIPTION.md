# Patch 25.51u-m – Safe Coordinates & Layout Stability

This patch improves fallback root promotion and eliminates parent-assignment loops by enforcing stricter layout logic and halting edge-case recursion early. It replaces prior default-parent selection logic with coordinate-clamped checks and recursion depth limits.
