use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageItem {
    pub id: String,
    pub title: String,
    pub shard: String,
    pub tags: Vec<String>,
    pub priority: u8,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxState {
    pub items: Vec<TriageItem>,
    pub selected: usize,
    pub context_open: bool,
}

impl InboxState {
    pub fn new() -> Self {
        let items = vec![
            TriageItem {
                id: "a1".into(),
                title: "Urgent network alert".into(),
                shard: "infra".into(),
                tags: vec!["alert".into(), "urgent".into()],
                priority: 1,
                created: Utc::now(),
            },
            TriageItem {
                id: "a2".into(),
                title: "Pending code review".into(),
                shard: "dev".into(),
                tags: vec!["review".into()],
                priority: 3,
                created: Utc::now(),
            },
        ];
        Self {
            items,
            selected: 0,
            context_open: false,
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn prev(&mut self) {
        if self.items.is_empty() { return; }
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn toggle_context(&mut self) {
        self.context_open = !self.context_open;
    }
}