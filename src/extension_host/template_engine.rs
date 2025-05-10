use crate::storage::inbox_storage::InboxState;
use crate::state::AppState;

pub fn push_template_task(inbox: &mut InboxState, item: String) {
    inbox.tasks.push(item);
}

pub fn apply_task_templates(app_state: &mut AppState) {
    let mut inbox = InboxState::new();

    let template_items = vec!["Email client", "Write spec doc", "Review PRs"];
    for item in template_items {
        push_template_task(&mut inbox, item.to_string());
    }

    app_state.sidebar_visible = true; // simulate impact of task update
    // Assume this would be app_state.inbox = inbox; in real logic
}