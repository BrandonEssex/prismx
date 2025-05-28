#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {};
}

pub mod core;

pub use core::{layout, node};

#[cfg(feature = "std")]
#[macro_use]
pub mod logger;

#[cfg(feature = "std")]
pub mod screen;
#[cfg(feature = "std")]
pub mod bootstrap;
#[cfg(feature = "std")]
pub mod config;
#[cfg(feature = "std")]
pub mod plugin;
#[cfg(feature = "std")]
pub mod plugins;
#[cfg(feature = "std")]
pub mod tui;
#[cfg(feature = "std")]
pub mod trust;
#[cfg(feature = "std")]
pub mod federation;
#[cfg(feature = "std")]
pub mod snapshot;
#[cfg(feature = "std")]
pub mod retire;
#[cfg(feature = "std")]
pub mod keymap;
#[cfg(feature = "std")]
pub mod spotlight;
#[cfg(feature = "std")]
pub mod theme;
#[cfg(feature = "std")]
pub mod clipboard;
#[cfg(feature = "std")]
pub mod input;
#[cfg(feature = "std")]
pub mod dashboard;
#[cfg(feature = "std")]
pub mod zen;
#[cfg(feature = "std")]
pub mod render;
#[cfg(feature = "std")]
pub mod beamx;
#[cfg(feature = "std")]
pub mod canvas;
#[cfg(feature = "std")]
pub mod beam_color;
#[cfg(feature = "std")]
pub mod ui;
#[cfg(feature = "std")]
pub mod gemx;
#[cfg(feature = "std")]
pub mod routineforge;
#[cfg(feature = "std")]
pub mod triage;
#[cfg(feature = "std")]
pub mod settings;
#[cfg(feature = "std")]
pub mod hotkeys;
#[cfg(feature = "std")]
#[path = "state/mod.rs"]
pub mod state;
#[cfg(feature = "std")]
pub mod shortcuts;
