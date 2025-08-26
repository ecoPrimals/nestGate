//
// Universal ZFS API handlers with fail-safe mechanisms and backend abstraction

mod basic;
pub mod types;
pub mod universal_pools; // New storage-agnostic pools handler
pub mod universal_zfs;
pub mod zero_cost_factory; // Zero-cost ZFS service factory

pub use basic::*;
pub use universal_pools::*; // Export universal storage functions
pub use universal_zfs::*;
pub use zero_cost_factory::*; // Export zero-cost factory patterns
