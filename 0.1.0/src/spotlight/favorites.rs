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

    pub fn mark_accessed(&mut self, uid: &str) {
        let entry = self.map.entry(uid.to_string()).or_insert(UsageMetadata {
            count: 0,
            last_access: Local::now(),
            pinned: false,
        });

        entry.count += 1;
        entry.last_access = Local::now();
    }

    pub fn toggle_pin(&mut self, uid: &str) {
        let entry = self.map.entry(uid.to_string()).or_insert(UsageMetadata {
            count: 0,
            last_access: Local::now(),
            pinned: false,
        });

        entry.pinned = !entry.pinned;
    }

    pub fn is_pinned(&self, uid: &str) -> bool {
        self.map.get(uid).map_or(false, |m| m.pinned)
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

    pub fn merge_boosts(&self, base_results: &mut Vec<(String, String, i64)>) {
        for res in base_results.iter_mut() {
            let uid = &res.0;
            let boost = self.get_boost_score(uid);
            res.2 += boost;
        }

        base_results.sort_by(|a, b| b.2.cmp(&a.2));
    }
}