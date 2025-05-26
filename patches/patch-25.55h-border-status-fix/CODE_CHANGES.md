## Code Changes

- Adjust render bounds for Spotlight, Settings, and other panels to never exceed terminal height minus footer/status height
- Apply consistent padding or clamping logic to outer border and shortcut icon renderer
- Ensure bottom row is not used for overlays if `StatusBar` is visible
- Ensure box borders are consistently visible in all views
