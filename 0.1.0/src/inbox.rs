use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Inbox,
    Triaged,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub shard: Option<String>,
    pub tags: Vec<String>,
    pub priority: Priority,
    pub status: TaskStatus,
    pub assigned_to: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl InboxTask {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        InboxTask {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            shard: None,
            tags: vec![],
            priority: Priority::Medium,
            status: TaskStatus::Inbox,
            assigned_to: None,
            created: now,
            modified: now,
        }
    }

    pub fn assign_to(&mut self, assignee: String) {
        self.assigned_to = Some(assignee);
        self.touch();
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.touch();
    }

    pub fn add_tags(&mut self, new_tags: Vec<String>) {
        for tag in new_tags {
            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }
        self.touch();
    }

    pub fn archive(&mut self) {
        self.status = TaskStatus::Archived;
        self.touch();
    }

    pub fn triage(&mut self) {
        self.status = TaskStatus::Triaged;
        self.touch();
    }

    pub fn touch(&mut self) {
        self.modified = Utc::now();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InboxState {
    pub tasks: Vec<InboxTask>,
}

impl InboxState {
    pub fn new() -> Self {
        InboxState { tasks: vec![] }
    }

    pub fn add_task(&mut self, title: String, description: Option<String>) {
        let task = InboxTask::new(title, description);
        self.tasks.push(task);
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut InboxTask> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn archive_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.archive();
            return true;
        }
        false
    }

    pub fn triage_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.triage();
            return true;
        }
        false
    }

    pub fn assign_task(&mut self, id: &str, assignee: String) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.assign_to(assignee);
            return true;
        }
        false
    }

    pub fn set_task_priority(&mut self, id: &str, priority: Priority) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.set_priority(priority);
            return true;
        }
        false
    }

    pub fn tag_task(&mut self, id: &str, tags: Vec<String>) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.add_tags(tags);
            return true;
        }
        false
    }

    pub fn list_by_status(&self, status: TaskStatus) -> Vec<&InboxTask> {
        self.tasks.iter().filter(|t| t.status == status).collect()
    }

    pub fn all_tasks(&self) -> &Vec<InboxTask> {
        &self.tasks
    }
}