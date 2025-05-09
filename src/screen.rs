use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::mindmap_state::MindmapState;
use crate::view_mindmap::render_mindmap;
use crate::actions::Action;
use ratatui::Frame;

pub struct Screen {
    pub mindmap: MindmapState,
}

impl Screen {
    pub fn new(_config: crate::config::Config, _spotlight: SpotlightModule) -> Self {
        use std::path::Path;
        use crate::storage::mindmap_disk;
        let mindmap = mindmap_disk::load_from_file(Path::new("data/mindmap.json"))
            .unwrap_or_else(|_| MindmapState::new());
        Self { mindmap }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, _state: &mut AppState) {
        let area = f.size();
        render_mindmap(f, area, &self.mindmap);
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::ToggleMindmapLayout => self.mindmap.toggle_layout(),
            Action::EnterEditNode => self.mindmap.start_edit(),
            Action::CancelEdit => self.mindmap.cancel_edit(),
            Action::CommitEdit => self.mindmap.commit_edit(),
            Action::PushEditChar(c) => self.mindmap.push_edit_char(c),
            Action::PopEditChar => self.mindmap.pop_edit_char(),
            Action::NavigateNext => self.mindmap.select_next(),
            Action::NavigatePrev => self.mindmap.select_prev(),
            Action::OpenContextMenu => self.mindmap.toggle_context_menu(),
            _ => {}
        }
    }
}