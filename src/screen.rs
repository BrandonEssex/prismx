use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::mindmap_state::MindmapState;
use crate::storage::inbox_storage::InboxState;
use crate::view_mindmap::render_mindmap;
use crate::view_triage::render_triage;
use crate::zen_mode::ZenModeState;
use crate::dashboard::Dashboard;
use crate::shortcut_overlay::render_shortcuts;
use crate::actions::Action;
use ratatui::{backend::Backend, Frame};

pub enum ActiveView {
    Mindmap,
    Triage,
    Dashboard,
    Zen,
}

pub struct Screen {
    pub mindmap: MindmapState,
    pub inbox: InboxState,
    pub zen: ZenModeState,
    pub dashboard: Dashboard,
    pub active: ActiveView,
    pub shortcut_overlay: bool,
}

impl Screen {
    pub fn new(config: crate::config::Config, _spotlight: SpotlightModule) -> Self {
        Self {
            mindmap: MindmapState::new(),
            inbox: InboxState::new(),
            zen: ZenModeState::new(&config),
            dashboard: Dashboard::new(),
            active: ActiveView::Mindmap,
            shortcut_overlay: false,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<'_>, _state: &mut AppState) {
        let area = f.size();

        match self.active {
            ActiveView::Mindmap => render_mindmap(f, area, &self.mindmap),
            ActiveView::Triage => render_triage(f, area, &self.inbox, self.inbox.context_open),
            ActiveView::Zen => self.zen.render::<B>(f, area),
            ActiveView::Dashboard => self.dashboard.render(f, area),
        }

        render_shortcuts::<B>(f, area, self.shortcut_overlay);
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::ToggleTriage => {
                self.active = match self.active {
                    ActiveView::Triage => ActiveView::Mindmap,
                    _ => ActiveView::Triage,
                }
            }
            Action::ToggleZenMode => {
                self.zen.toggle();
                self.active = if self.zen.enabled {
                    ActiveView::Zen
                } else {
                    ActiveView::Mindmap
                };
            }
            Action::ToggleShortcuts => {
                self.shortcut_overlay = !self.shortcut_overlay;
            }
            Action::Tick => {
                self.zen.tick();
            }