use ratatui::style::{Color, Style};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static CURRENT_THEME: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("dark".to_string()));

pub fn toggle_theme() {
    let mut theme = CURRENT_THEME.lock().unwrap();
    if theme.as_str() == "dark" {
        *theme = "light".to_string();
    } else {
        *theme = "dark".to_string();
    }
    tracing::debug!("[THEME] Switched to: {}", *theme);
}

pub fn set_theme(theme: &str) {
    let mut current = CURRENT_THEME.lock().unwrap();
    *current = theme.to_string();
    tracing::debug!("[THEME] Set to: {}", *current);
}


pub fn get_style(target: &str) -> Style {
    let default = Style::default().fg(Color::White).bg(Color::Black);
    let config = fs::read_to_string("theme/theme.toml").unwrap_or_default();
    let parsed: HashMap<String, HashMap<String, String>> = toml::from_str(&config).unwrap_or_default();

    if let Some(theme) = parsed.get("theme") {
        match target {
            "mindmap" => color_from_map(theme, "mindmap").unwrap_or(default),
            "spotlight" => color_from_map(theme, "spotlight").unwrap_or(default),
            "keymap" => color_from_map(theme, "keymap").unwrap_or(default),
            "clipboard" => color_from_map(theme, "clipboard").unwrap_or(default),
            "dashboard" => color_from_map(theme, "dashboard").unwrap_or(default),
            "zen" => color_from_map(theme, "zen").unwrap_or(default),
            "status" => color_from_map(theme, "status").unwrap_or(default),
            _ => default,
        }
    } else {
        default
    }
}

fn color_from_map(map: &HashMap<String, String>, key: &str) -> Option<Style> {
    let fg = map.get(&format!("{}.fg", key)).and_then(|s| parse_color(s));
    let bg = map.get(&format!("{}.bg", key)).and_then(|s| parse_color(s));
    Some(Style::default().fg(fg?).bg(bg?))
}

fn parse_color(s: &str) -> Option<Color> {
    match s.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "white" => Some(Color::White),
        "gray" => Some(Color::Gray),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        "yellow" => Some(Color::Yellow),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        _ => None,
    }
}
