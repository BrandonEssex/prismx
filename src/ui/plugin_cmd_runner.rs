use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct CommandRunnerOverlay {
    pub input: String,
    pub visible: bool,
}

impl CommandRunnerOverlay {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            visible: false,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn update_input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        if !self.visible {
            return;
        }

        let block = Block::default()
            .title("Command")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD));

        let content = Paragraph::new(format!(":{}", self.input.clone()))
            .block(block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        f.render_widget(content, centered_rect(60, 15, area));
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
