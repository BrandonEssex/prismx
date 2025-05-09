use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
};
use crate::state::AppState;
use crate::mindmap_state::MindmapState;
use crate::view_mindmap::render_mindmap;
use crate::view_triage::render_triage;
use crate::zen_mode::ZenModeState;
use crate::dashboard::Dashboard;
use crate::log_viewer::render_log_viewer;
use crate::shortcut_overlay::render_shortcuts;
use crate::actions::Action;

pub enum ActiveView {
    Mindmap,
    Triage,
    Zen,
    Dashboard,
    Log,
}

pub struct Screen {
    pub mindmap: MindmapState,
    pub zen: ZenModeState,
    pub dashboard: Dashboard,
    pub view: ActiveView,
    pub shortcut_overlay: bool,
}

impl Screen {
    pub fn new(config: crate::config::Config, _spotlight: crate::spotlight::SpotlightModule) -> Self {
        Self {
            mindmap: MindmapState::new(),
            zen: ZenModeState::new(&config.zen_mode),
            dashboard: Dashboard::new(),
            view: ActiveView::Mindmap,
            shortcut_overlay: false,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, _state: &mut AppState) {
        let area = f.size();
        match self.view {
            ActiveView::Mindmap => render_mindmap(f, area, &self.mindmap),
            ActiveView::Triage => render_triage(f, area, &self.mindmap), // Simplified to reuse
            ActiveView::Zen => self.zen.render(f, area),
            ActiveView::Dashboard => self.dashboard.render(f, area),
            ActiveView::Log => render_log_viewer(f, area),
        }

        if self.shortcut_overlay {
            render_shortcuts(f, area, self.shortcut_overlay);
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::ToggleShortcuts => {
                self.shortcut_overlay = !self.shortcut_overlay;
            }
            Action::ToggleZenMode => {
                self.view = if matches!(self.view, ActiveView::Zen) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Zen
                };
            }
            Action::ToggleLogViewer => {
                self.view = if matches!(self.view, ActiveView::Log) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Log
                };
            }
            Action::ToggleTriage => {
                self.view = if matches!(self.view, ActiveView::Triage) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Triage
                };
            }
            Action::ToggleDashboard => {
                self.view = if matches!(self.view, ActiveView::Dashboard) {
                    ActiveView::Mindmap
                } else {
                    ActiveView::Dashboard
                };
            }
            _ => {
                self.mindmap.handle_action(action);
            }
        }
    }
}