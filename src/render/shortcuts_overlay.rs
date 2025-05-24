use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::shortcuts::SHORTCUTS;

pub fn render_shortcuts_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let keys: Vec<String> = SHORTCUTS
        .iter()
        .map(|(k, v)| format!("{} = {}", k, v))
        .collect();

    let content = Paragraph::new(keys.join("\n"))
        .block(Block::default().title("Shortcuts").borders(Borders::ALL));

    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2));
}
