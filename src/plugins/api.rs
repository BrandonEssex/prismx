use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

static TASK_QUEUE: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

/// Publish a plugin-generated task string for triage ingestion.
pub fn publish_task(text: impl Into<String>) {
    let queue = TASK_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()));
    queue.lock().unwrap().push_back(text.into());
}

/// Drain all queued plugin tasks.
pub fn drain_tasks() -> Vec<String> {
    let queue = TASK_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()));
    queue.lock().unwrap().drain(..).collect()
}
