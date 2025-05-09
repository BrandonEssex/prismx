PrismX Plugin & Extension Developer Guide (v1.0.0-dev0)

Welcome to PrismX — a modular, offline-capable terminal productivity suite. This guide documents everything needed to develop plugins, themes, and extensions for PrismX.

⸻

1. Plugin Architecture Overview

1.1 Plugin Format

Plugins are WASM modules defined in an extension folder:

/extensions/example-plugin.prismx-ext/
├── plugin.wasm
└── prismx-plugin.json

1.2 Plugin Manifest Example

{
  "name": "hello-widget",
  "version": "1.0.0",
  "capabilities": ["render:dashboard", "log:info"],
  "entry": "plugin.wasm"
}

1.3 Lifecycle Hooks
	•	on_start() — called once when PrismX boots
	•	on_tick() — called once per heartbeat cycle (default 750ms)
	•	on_signal(signal: &str) — called when a SignalBus event occurs (e.g. TaskAdded)
	•	render_dashboard(f: &mut Frame, area: Rect) — if render:dashboard declared

⸻

2. Capability-Based Permissions

All plugins must declare capabilities. These are strictly enforced by PrismX at runtime.

Capability	Description
log:info	Allow plugin to write to system log
file:read	Read-only access to the filesystem
file:write	Write access (restricted to data/extensions)
render:dashboard	Permit TUI slot rendering
signal:emit	Plugin can emit custom SignalBus events
input:bind	Plugin can register shortcut keys

Plugins without proper capabilities will be sandboxed and blocked.

⸻

3. Extension Folder Layout

/extensions/
└── my-plugin.prismx-ext/
    ├── plugin.wasm
    ├── prismx-plugin.json
    └── plugin_config.json      # optional

	•	Extensions can be hot-reloaded.
	•	All logs are routed to: logs/plugins/<plugin_name>.log
	•	Extensions are profiled for memory and tick execution time.

⸻

4. Signals & Bus Events

Use emit_signal("TaskAdded") to send events to:
	•	Other plugins
	•	Core systems (Inbox, Dashboard, RoutineForge)

Built-in signal examples:
	•	TaskAdded
	•	RoutineTriggered
	•	UnlockSuccess
	•	DashboardRefresh

⸻

5. Theme Development

5.1 Structure

{
  "palette": {
    "primary": "#00caff",
    "accent": "#ffe277",
    "danger": "#ff0055"
  },
  "border_style": "double",
  "font_size": "normal",
  "contrast": "high"
}

	•	Stored in: assets/theme.json
	•	Reloaded at runtime via SIGUSR1
	•	Affects dashboard, mindmap, triage, and overlays

⸻

6. Developer Tools

CLI Commands

prismx check-plugin extensions/my-plugin.prismx-ext/
prismx diagnostics
prismx run-routine morning-routine

Log Access
	•	Main runtime log: logs/qa_runtime.log
	•	Plugin logs: logs/plugins/
	•	Audit trail: .audit/session_<timestamp>.log

⸻

7. Writing a Simple Plugin (Rust → WASM)
	1.	Use wasm32-unknown-unknown target
	2.	Export symbols using #[no_mangle]
	3.	Avoid stdlib IO or threads — use capabilities

#[no_mangle]
pub extern "C" fn on_tick() {
    prismx_log("Hello from plugin on_tick()");
}


⸻

8. Plugin Best Practices
	•	Use log: capability instead of println!
	•	Avoid long-running work in on_tick()
	•	Always check for capability before accessing data or IO
	•	Validate prismx-plugin.json with prismx check-plugin

⸻

9. Security and Trust
	•	Plugin registry (registry.json) can mark known/trusted plugins
	•	Untrusted plugins cannot render overlays or emit signals
	•	Plugins are sandboxed and profiled by default

⸻

10. Version Compatibility
	•	manifest_version: defaults to 2
	•	Compatible with PrismX v1.0.0 and above
	•	Backward-compatible plugins should avoid capabilities added post-1.0

⸻

Need Help?
	•	Join the PrismX plugin developer community on GitHub Discussions
	•	Open an issue with your plugin or extension folder attached
	•	Consult the plugin example at: /extensions/hello-widget.prismx-ext/

⸻

Happy extending PrismX!