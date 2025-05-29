use chrono::{Local, Duration};
use crate::triage::state::TriageEntry;

/// Calculate consecutive days with at least one `#DONE` entry.
pub fn completion_streak(entries: &[TriageEntry]) -> usize {
    use std::collections::HashSet;
    let days: HashSet<_> = entries
        .iter()
        .filter(|e| e.tags.iter().any(|t| t == "#done"))
        .map(|e| e.created.date_naive())
        .collect();
    let mut streak = 0usize;
    let mut day = Local::now().date_naive();
    loop {
        if days.contains(&day) {
            streak += 1;
            day -= Duration::days(1);
        } else {
            break;
        }
    }
    streak
}

/// Return counts of `#DONE` entries for the last `n` days.
fn done_counts_last_n_days(entries: &[TriageEntry], n: usize) -> Vec<usize> {
    let today = Local::now().date_naive();
    (0..n)
        .map(|i| {
            let day = today - Duration::days((n - 1 - i) as i64);
            entries
                .iter()
                .filter(|e| {
                    e.tags.iter().any(|t| t == "#done") && e.created.date_naive() == day
                })
                .count()
        })
        .collect()
}

/// Render a small sparkline using Unicode blocks for the provided counts.
fn sparkline_from_counts(counts: &[usize]) -> String {
    const BLOCKS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let max = counts.iter().copied().max().unwrap_or(0);
    counts
        .iter()
        .map(|&c| {
            let idx = if max == 0 {
                0
            } else {
                (c * (BLOCKS.len() - 1) + max - 1) / max
            };
            BLOCKS[idx] as char
        })
        .collect()
}

/// Generate a sparkline string for the last `n` days of completions.
pub fn done_sparkline(entries: &[TriageEntry], n: usize) -> String {
    let counts = done_counts_last_n_days(entries, n);
    sparkline_from_counts(&counts)
}

/// Progress bar representing ratio of completed tasks.
pub fn progress_bar(now: usize, triton: usize, done: usize) -> String {
    let total = now + triton + done;
    let len = 10usize;
    if total == 0 {
        return format!("[{}]", " ".repeat(len));
    }
    let filled = ((done as f32 / total as f32) * len as f32).round() as usize;
    let filled = filled.min(len);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(len - filled))
}
