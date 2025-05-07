use std::collections::HashMap;
use chrono::{DateTime, Duration, Local};

#[derive(Clone)]
pub struct UsageMetadata {
    pub count: usize,
    pub last_access: DateTime<Local>,
    pub pinned: bool,
}

pub struct Favorites {
    map: HashMap<String, UsageMetadata>,
}

impl Favorites {
    pub fn new() -> Self {
        Favorites {
            map: HashMap::new(),
        }
    }

    pub fn get_boost_score(&self, uid: &str) -> i64 {
        if let Some(meta) = self.map.get(uid) {
            let mut score = 0;
            if meta.pinned {
                score += 100;
            }

            score += (meta.count as i64) * 5;

            let age = Local::now().signed_duration_since(meta.last_access);
            if age < Duration::days(1) {
                score += 30;
            } else if age < Duration::days(7) {
                score += 15;
            } else if age < Duration::days(30) {
                score += 5;
            }

            score
        } else {
            0
        }
    }
}