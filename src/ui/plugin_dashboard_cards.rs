use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct PluginCard {
    pub name: String,
    pub status: String,
    pub color: Color,
}

pub fn render_plugin_dashboard<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    cards: &[PluginCard],
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3); cards.len()])
        .split(area);

    for (i, card) in cards.iter().enumerate() {
        let block = Block::default()
            .title(card.name.clone())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(card.color).add_modifier(Modifier::BOLD));

        let text = Paragraph::new(format!("Status: {}", card.status)).block(block);
        f.render_widget(text, layout[i]);
    }
}
