#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use]
pub mod logging;
pub mod collections;
pub mod io;
pub mod node;
pub mod layout;
pub mod screen;
pub mod bootstrap;
pub mod config;
pub mod plugin;
pub mod plugins;
pub mod tui;
pub mod trust;
pub mod federation;
pub mod snapshot;
pub mod retire;
pub mod keymap;
pub mod spotlight;
pub mod theme;
pub mod clipboard;
pub mod input;
pub mod dashboard;
pub mod render;
pub mod beamx;
pub mod ui;
pub mod gemx;
pub mod routineforge;
pub mod settings;
#[path = "state/mod.rs"]
pub mod state;
pub mod shortcuts;
