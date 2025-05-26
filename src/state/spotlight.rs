use super::core::{AppState, DockLayout, SimInput};
use crate::state::ZenViewMode;

impl AppState {
    pub fn exit_spotlight(&mut self) {
        self.spotlight_input.clear();
        self.show_spotlight = false;
        self.spotlight_just_opened = false;
        self.spotlight_suggestion_index = None;
    }

    pub fn execute_spotlight_command(&mut self) {
        let command = self.spotlight_input.clone();
        let input = command.trim();
        let cmd = input.trim_start_matches('/');
        crate::log_debug!(self, "Executing spotlight: {}", input);
        if !input.is_empty() {
            self.spotlight_history.push_front(input.to_string());
            while self.spotlight_history.len() > 10 {
                self.spotlight_history.pop_back();
            }
        }
        self.spotlight_history_index = None;
        self.spotlight_suggestion_index = None;
        if cmd == "start pomodoro" {
            self.plugin_host.start_pomodoro();
        } else if cmd.starts_with("countdown ") {
            let rest = &cmd["countdown ".len()..];
            let mut parts = rest.splitn(2, ' ');
            let days_part = parts.next().unwrap_or("");
            let label = parts.next().unwrap_or("").to_string();
            if let Some(num) = days_part.strip_prefix('+').and_then(|s| s.strip_suffix('d')) {
                if let Ok(days) = num.parse::<u64>() {
                    self.plugin_host.add_countdown(label, days);
                }
            }
        } else if cmd.starts_with("dock_layout=") {
            let layout = &cmd["dock_layout=".len()..];
            if layout.eq_ignore_ascii_case("horizontal") {
                self.favorite_dock_layout = DockLayout::Horizontal;
            } else if layout.eq_ignore_ascii_case("vertical") {
                self.favorite_dock_layout = DockLayout::Vertical;
            }
            self.dock_focus_index = None;
            self.status_message.clear();
            self.status_message_last_updated = None;
        } else if cmd.starts_with("dock_limit=") {
            let value = &cmd["dock_limit=".len()..];
            if let Ok(num) = value.parse::<usize>() {
                self.favorite_dock_limit = num.min(5);
            }
        } else if cmd.starts_with("dock_enabled=") {
            let value = &cmd["dock_enabled=".len()..];
            if let Ok(flag) = value.parse::<bool>() {
                self.favorite_dock_enabled = flag;
            }
            if !self.favorite_dock_enabled {
                self.dock_focus_index = None;
                self.status_message.clear();
                self.status_message_last_updated = None;
            }
        } else if cmd.starts_with("simulate") {
            for token in cmd.split_whitespace().skip(1) {
                match token.to_lowercase().as_str() {
                    "enter" => self.simulate_input_queue.push_back(SimInput::Enter),
                    "tab" => self.simulate_input_queue.push_back(SimInput::Tab),
                    "delete" => self.simulate_input_queue.push_back(SimInput::Delete),
                    "drill" => self.simulate_input_queue.push_back(SimInput::Drill),
                    "pop" => self.simulate_input_queue.push_back(SimInput::Pop),
                    "undo" => self.simulate_input_queue.push_back(SimInput::Undo),
                    "redo" => self.simulate_input_queue.push_back(SimInput::Redo),
                    _ => continue,
                }
            }
        } else {
            match cmd {
                "triage" => self.mode = "triage".into(),
                "zen" => self.mode = "zen".into(),
                "settings" => self.mode = "settings".into(),
                "gemx" => self.mode = "gemx".into(),
                "toggle triage" => self.show_triage = !self.show_triage,
                "toggle keymap" => self.show_keymap = !self.show_keymap,
                "toggle spotlight" => self.show_spotlight = !self.show_spotlight,
                "mode zen" => self.mode = "zen".into(),
                "mode gemx" => self.mode = "gemx".into(),
                "arrange" => self.auto_arrange = true,
                "undo" => self.undo(),
                "redo" => self.redo(),
                "toolbar" => {
                    self.zen_toolbar_open = !self.zen_toolbar_open;
                    self.zen_toolbar_index = 0;
                }
                cmd if cmd.starts_with("zen tag") => {
                    let tag = cmd.trim_start_matches("zen tag").trim();
                    if tag.is_empty() {
                        self.set_tag_filter(None);
                    } else {
                        self.set_tag_filter(Some(tag));
                    }
                }
                "zen summary" => {
                    self.toggle_summary_view();
                }
                _ if cmd.starts_with("open ") => {
                    let path = cmd.trim_start_matches("open ").trim();
                    if !path.is_empty() {
                        self.open_zen_file(path);
                        self.mode = "zen".into();
                    }
                }
                "clear" => self.zen_buffer = vec![String::new()],
                _ => {}
            }
        }
        self.exit_spotlight();
    }
}
