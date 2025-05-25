//! Minimal wrapper around filesystem and environment access so the core
//! application can build in `no_std` contexts.  When the `std` feature is
//! enabled we simply re-export the standard library modules.  Without `std`
//! these APIs become stubs that return errors.

#[cfg(feature = "std")]
pub mod fs {
    pub use std::fs::*;
}

#[cfg(not(feature = "std"))]
pub mod fs {
    use alloc::string::String;

    pub fn read_to_string<P: ?Sized>(_path: &P) -> Result<String, ()> {
        Err(())
    }

    pub fn write<P: ?Sized>(_path: &P, _data: impl AsRef<[u8]>) -> Result<(), ()> {
        Err(())
    }

    pub fn create_dir_all<P: ?Sized>(_path: &P) -> Result<(), ()> {
        Err(())
    }
}

#[cfg(feature = "std")]
pub mod env {
    pub use std::env::*;
}

#[cfg(not(feature = "std"))]
pub mod env {
    pub fn var(_key: &str) -> Result<String, ()> {
        Err(())
    }
}

