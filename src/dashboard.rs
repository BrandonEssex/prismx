use crate::dashboard_widgets::{render_clock_widget, render_shortcuts};
use ratatui::backend::Backend;
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::Frame;

pub fn render_dashboard<B: Backend>(f: &mut Frame<B>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    render_clock_widget(f, layout[0]);
    render_shortcuts(f, layout[1]);
}
