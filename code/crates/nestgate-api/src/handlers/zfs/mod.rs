//! ZFS API Handlers
//!
//! Universal ZFS API handlers with fail-safe mechanisms and backend abstraction

mod basic;
pub mod types;
mod universal_zfs;

pub use basic::*;
pub use universal_zfs::*;
