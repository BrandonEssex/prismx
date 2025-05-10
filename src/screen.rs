use crate::{
    actions::Action,
    state::AppState,
    view_mindmap::render_mindmap,
};

use ratatui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
};

pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Screen
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, state: &mut AppState) {
        let area = f.size();
        render_mindmap(f, area, &state.mindmap);
    }

    pub fn handle_action(&mut self, action: Action, state: &mut AppState) {
        match action {
            Action::EnterEditNode => state.mindmap.start_edit(),
            Action::PushEditChar(c) => state.mindmap.push_edit_char(c),
            Action::PopEditChar => state.mindmap.pop_edit_char(),
            Action::CommitEdit => state.mindmap.commit_edit(),
            Action::CancelEdit => state.mindmap.cancel_edit(),
            Action::CreateSiblingNode => state.mindmap.create_sibling(),
            Action::CreateChildNode => state.mindmap.create_child(),
            Action::NavigateNext => state.mindmap.select_next(),
            Action::NavigatePrev => state.mindmap.select_prev(),
            _ => {}
        }
    }
}