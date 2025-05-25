//! Abstractions over collection types that work in both `std` and `no_std`
//! builds. When the `std` feature is enabled we simply re-export the types
//! from `std::collections`.  In `no_std` mode we rely on `hashbrown` for
//! `HashMap`/`HashSet` and `alloc::collections` for `VecDeque`.

#![allow(dead_code)]

#[cfg(feature = "std")]
pub use std::collections::{HashMap, HashSet, VecDeque};
#[cfg(feature = "std")]
pub use std::rc::Rc;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
pub use alloc::collections::VecDeque;
#[cfg(not(feature = "std"))]
pub use hashbrown::{HashMap, HashSet};
#[cfg(not(feature = "std"))]
pub use alloc::rc::Rc;

