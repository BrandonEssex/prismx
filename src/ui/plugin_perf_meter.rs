use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

pub fn render_perf_meter<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    label: &str,
    percent: f64,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3)])
        .split(area);

    let gauge = Gauge::default()
        .block(Block::default().title(label).borders(Borders::ALL))
        .gauge_style(
            Style::default()
                .fg(Color::LightMagenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .label(format!("{:.0}%", percent * 100.0))
        .ratio(percent);

    f.render_widget(gauge, layout[0]);
}
