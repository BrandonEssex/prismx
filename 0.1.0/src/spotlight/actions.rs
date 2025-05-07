use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use chrono::Local;
use log::info;

use super::state::SpotlightState;

pub fn perform_move(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        let target_shard = prompt_shard();
        info!(
            "[{}] Move: {} → {}",
            timestamp(),
            item.uid,
            target_shard
        );
    }
}

pub fn perform_delete(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        if confirm(&format!("Delete {}?", item.display_title)) {
            info!(
                "[{}] Delete: {} ({})",
                timestamp(),
                item.uid,
                item.display_title
            );
        }
    }
}

pub fn perform_export(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        let path = export_path(&item.uid);
        if let Ok(mut file) = File::create(&path) {
            writeln!(file, "# {}\n\nExported from Spotlight", item.display_title).ok();
            info!(
                "[{}] Export: {} → {}",
                timestamp(),
                item.uid,
                path.display()
            );
        }
    }
}

pub fn toggle_favorite(state: &mut SpotlightState) {
    if let Some(item) = state.matched.get(state.selected) {
        info!(
            "[{}] Toggle Favorite: {} ({})",
            timestamp(),
            item.uid,
            item.display_title
        );
    }
}

// Helper stubs
fn timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn prompt_shard() -> String {
    "archive".to_string()
}

fn confirm(_msg: &str) -> bool {
    true
}

fn export_path(uid: &str) -> PathBuf {
    let mut path = PathBuf::from("exports");
    fs::create_dir_all(&path).ok();
    path.push(format!("{}.md", uid));
    path
}