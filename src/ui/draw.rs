use ratatui::Frame;
use crate::state::AppState;
use crate::ui::sidebar::render_sidebar;
use crate::ui::status_bar::render_status_bar;
use crate::ui::status_icon::render_status_icon;
use ratatui::layout::{Layout, Constraint, Direction};

pub fn draw_ui<B>(f: &mut Frame<B>, state: &mut AppState)
where
    B: ratatui::backend::Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(f.size());

    render_sidebar(f, chunks[0], state);
    let status = render_status_bar(state);
    f.render_widget(status, chunks[1]);

    let icon_area = ratatui::layout::Rect {
        x: chunks[1].x + chunks[1].width.saturating_sub(8),
        y: chunks[1].y,
        width: 7,
        height: 1,
    };

    render_status_icon(f, icon_area, &state.icon_state);
}