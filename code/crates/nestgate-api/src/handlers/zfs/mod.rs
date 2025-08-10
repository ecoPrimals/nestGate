//! ZFS API Handlers
//!
//! Universal ZFS API handlers with fail-safe mechanisms and backend abstraction

mod basic;
pub mod types;
pub mod universal_pools; // New storage-agnostic pools handler
pub mod universal_zfs;

pub use basic::*;
pub use universal_pools::*; // Export universal storage functions
pub use universal_zfs::*;
