// src/main.rs

use prismx::action;
use prismx::app;
use prismx::input;
use prismx::screen;
use prismx::state;
use prismx::ui;
use prismx::plugin;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
    }
}
