#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
pub mod logger;
pub mod node;
pub mod layout;
#[cfg(std)]
pub mod screen;
pub mod bootstrap;
pub mod config;
pub mod plugin;
pub mod plugins;
#[cfg(std)]
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
pub mod zen;
#[cfg(std)]
pub mod render;
pub mod beamx;
pub mod canvas;
pub mod beam_color;
#[cfg(std)]
pub mod ui;
pub mod gemx;
pub mod routineforge;
pub mod triage;
pub mod settings;
pub mod hotkeys;
#[path = "state/mod.rs"]
pub mod state;
pub mod shortcuts;
