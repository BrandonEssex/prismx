use ratatui::Frame;
use crate::actions::Action;
use crate::dashboard::Dashboard;
use crate::log_viewer::render_log_viewer;
use crate::mindmap_state::MindmapState;
use crate::shortcut_overlay::render_shortcuts;
use crate::spotlight::SpotlightModule;
use crate::state::AppState;
use crate::view_mindmap::render_mindmap;
use crate::view_triage::render_triage;
use crate::zen_mode::ZenModeState;
use crate::storage::inbox_storage::InboxState;

#[derive(Debug)]
pub enum ActiveView {
    Mindmap,
    Triage,
    Zen,
    Log,
}

pub struct Screen {
    pub mindmap: MindmapState,
    pub inbox: InboxState,
    pub zen: ZenModeState,
    pub dashboard: Dashboard,
    pub view: ActiveView,
    pub shortcut_overlay: bool,
}

impl Screen {
    pub fn new(config: crate::config::Config, _spotlight: SpotlightModule) -> Self {
        Self {
            mindmap: MindmapState::new(),
            inbox: InboxState::default(),
            zen: ZenModeState::new(&config.zen_mode),
            dashboard: Dashboard::new(),
            view: ActiveView::Mindmap,
            shortcut_overlay: false,
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, _state: &mut AppState) {
        log::info!("[DRAW] Redrawing active view: {:?}", self.view);
        let area = f.size();
        match self.view {
            ActiveView::Mindmap => render_mindmap(f, area, &self.mindmap),
            ActiveView::Triage => render_triage(f, area, &self.inbox, self.inbox.context_open),
            ActiveView::Zen => self.zen.render(f, area),
            ActiveView::Log => render_log_viewer(f, area),
        }
        if self.shortcut_overlay {
            render_shortcuts(f, area, true);
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
            _ => {
                self.mindmap.handle_action(action);
            }
        }
    }
}