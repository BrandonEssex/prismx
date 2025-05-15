use crate::dashboard_widgets::{render_clock_widget, render_shortcuts};
use crate::icon::render_prismx_icon;
use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    Frame,
};

pub fn render_dashboard(f: &mut Frame, show_shortcuts: bool) {
    let size = f.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(size);

    let icon_area = Rect {
        x: size.width.saturating_sub(11),
        y: size.y + 1,
        width: 10,
        height: 3,
    };
    render_prismx_icon(f, icon_area);

    render_clock_widget(f, layout[1]);
    if show_shortcuts {
        render_shortcuts(f, layout[2]);
    }
}
