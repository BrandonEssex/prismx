use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
};

use crate::actions::Action;
use crate::state::AppState;
use crate::view_mindmap::render_mindmap;
use crate::view_triage::render_triage;
use crate::zen_mode::ZenModeState;
use crate::dashboard::Dashboard;
use crate::log_viewer::render_log_viewer;
use crate::shortcut_overlay::render_shortcuts;

#[derive(Debug)]
pub enum ActiveView {
    Mindmap,
    Triage,
    Zen,
    Log,
}

pub struct Screen {
    pub view: ActiveView,
    pub zen: ZenModeState,
    pub dashboard: Dashboard,
}

impl Screen {
    pub fn new(config: crate::config::Config, _spotlight: crate::spotlight::SpotlightModule) -> Self {
        Screen {
            view: ActiveView::Mindmap,
            zen: ZenModeState::new(&config.zen_mode),
            dashboard: Dashboard::default(),
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, state: &mut AppState) {
        let area = f.size();
        match self.view {
            ActiveView::Mindmap => render_mindmap(f, area, &state.mindmap),
            ActiveView::Triage => render_triage(f, area, &state.inbox, false),
            ActiveView::Zen => self.zen.render(f, area),
            ActiveView::Log => render_log_viewer(f, area),
        }

        if state.export.format == "json" {
            render_shortcuts(f, area, true);
        }
    }

    pub fn handle_action(&mut self, action: Action, state: &mut AppState) {
        match action {
            Action::ToggleZenMode => {
                self.view = if matches!(self.view, ActiveView::Zen) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Zen
                };
            }
            Action::ToggleTriage => {
                self.view = if matches!(self.view, ActiveView::Triage) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Triage
                };
            }
            Action::ToggleLogViewer => {
                self.view = if matches!(self.view, ActiveView::Log) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Log
                };
            }
            Action::Quit => state.quit(),
            _ => {}
        }
    }
}