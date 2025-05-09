use crate::config::load_config;
use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::screen::Screen;
use crate::extension_host::ExtensionHost;
use crate::input::InputHandler;
use crate::actions::Action;

use crossterm::terminal;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::stdout;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::storage::mindmap_disk;
use serde_json;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut state = AppState::new(config.clone());
    let spotlight = SpotlightModule::new();
    let mut screen = Screen::new(config, spotlight);

    terminal::enable_raw_mode()?;
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut ext_host = ExtensionHost::new();
    ext_host.load_all()?;

    let input = InputHandler;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(750);

    while state.is_running() {
        terminal.draw(|f| {
            screen.draw::<CrosstermBackend<std::io::Stdout>>(f, &mut state);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if let Some(event) = input.poll_event_timeout(timeout)? {
            if let Some(action) = input.handle_event(event) {
                match action {
                    Action::Quit => state.quit(),
                    _ => screen.handle_action(action),
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            screen.handle_action(Action::Tick);
            last_tick = Instant::now();
        }
    }

    mindmap_disk::save_to_file(&screen.mindmap, Path::new("data/mindmap.json"))?;
    let inbox_json = serde_json::to_string_pretty(&screen.inbox)?;
    std::fs::write("data/inbox.json", inbox_json)?;

    terminal::disable_raw_mode()?;
    Ok(())
}