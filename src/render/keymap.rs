use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_keymap_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let keys = vec![
        "Ctrl+C  = Quit",
        "Ctrl+Z  = Undo",
        "Ctrl+Shift+Z = Redo",
        "Ctrl+R  = Start/Finish Drag",
        "Ctrl+L  = Start/Finish Link",
        "Ctrl+N  = Free Node",
        "Tab     = Add Child",
        "Enter   = Add Sibling",
        "Ctrl+D  = Delete Node",
        "Ctrl+W  = Drill Down",
        "Ctrl+Q  = Pop Up",
        "Ctrl+T  = Toggle Collapse",
        "Ctrl+X  = Save Zen",
        "Ctrl+P  = Toggle Auto-Arrange",
        "Alt+← / Alt+→ = Horizontal Scroll",
        "Ctrl+Space = Module Switcher",
        "Ctrl+.  = Settings",
        "Alt+Space = Spotlight",
        "Ctrl+Y  = Triage",
        "Ctrl+H  = Help",
        "Esc     = Close Overlay / Exit",
    ];

    let content = Paragraph::new(keys.join("\n"))
        .block(Block::default().title("Keymap").borders(Borders::ALL));

    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2));
}
