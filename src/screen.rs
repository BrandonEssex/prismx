use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::mindmap_state::MindmapState;
use crate::storage::inbox_storage::InboxState;
use crate::view_mindmap::render_mindmap;
use crate::view_triage::render_triage;
use crate::actions::Action;
use ratatui::Frame;

pub enum ActiveView {
    Mindmap,
    Triage,
}

pub struct Screen {
    pub mindmap: MindmapState,
    pub inbox: InboxState,
    pub active: ActiveView,
}

impl Screen {
    pub fn new(_config: crate::config::Config, _spotlight: SpotlightModule) -> Self {
        let mindmap = MindmapState::new();
        let inbox = InboxState::new();
        Self {
            mindmap,
            inbox,
            active: ActiveView::Mindmap,
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, _state: &mut AppState) {
        let area = f.size();
        match self.active {
            ActiveView::Mindmap => render_mindmap(f, area, &self.mindmap),
            ActiveView::Triage => render_triage(f, area, &self.inbox, self.inbox.context_open),
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match self.active {
            ActiveView::Mindmap => match action {
                Action::ToggleTriage => self.active = ActiveView::Triage,
                _ => self.mindmap.handle_action(action),
            },
            ActiveView::Triage => match action {
                Action::ToggleTriage => self.active = ActiveView::Mindmap,
                Action::NavigateNext => self.inbox.next(),
                Action::NavigatePrev => self.inbox.prev(),
                Action::OpenContextMenu => self.inbox.toggle_context(),
                _ => {}
            },
        }
    }
}