// src/main.rs

mod action;
mod app;
mod input;
mod screen;
mod state;
mod ui;
mod plugin;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
    }
}