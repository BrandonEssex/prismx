# 🧠 PrismX Patch Tracking & Review

This file tracks feature-level verification across unstable or partially verified patches. Use it as a dev sanity checklist.

---

## ✅ GemX (Mindmap)

### 🔲 Zoom / Scroll
- [ ] Zoom-to-cursor works without jitter (`25.45x`)
- [ ] Scroll range reflects node positions

### 🔲 Auto-Arrange
- [ ] Siblings no longer overlap (`25.45g`)
- [ ] Free nodes render (`Ctrl+N`)
- [ ] Ghost nodes ignored

### 🔲 Drill / Pop
- [ ] `Ctrl+W/Q` isolates tree properly
- [ ] Root visual layout restored (`25.45n-i`)

### 🔲 Node Roles
- [ ] Orphan render + dim
- [ ] Ghost render skipped
- [ ] Free render glyph visible

---

## 📌 Notes
- `patch-25.45n-*` series includes fragile layout logic
- `patch-25.45x` zoom logic might be incomplete
- Next: Zen overlay, plugin focus stack, settings split
