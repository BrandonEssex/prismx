## Code Changes

### 1. Redesign Module Switcher Panel UI

Replace current `[ Module ]` style with icon-annotated list:
```text
Switch Module

  💭 Mindmap
  ✍️  Zen Mode
  🧭 Triage
  🔍 Spotlight
2. Highlight current selection with:
> arrow
Color accent (cyan, bright white, etc.)
Possibly bold or inverse
Example render logic:

let modules = vec![
    ("💭", "Mindmap"),
    ("✍️", "Zen Mode"),
    ("🧭", "Triage"),
    ("🔍", "Spotlight"),
];

for (i, (icon, label)) in modules.iter().enumerate() {
    let prefix = if i == state.active_module_index { "> " } else { "  " };
    let style = if i == state.active_module_index {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let line = format!("{}{} {}", prefix, icon, label);
    f.render_widget(Paragraph::new(line).style(style), ...)
}
3. Add vertical padding or centering if space permits
