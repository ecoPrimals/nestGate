//! **ZERO-COST ZFS OPERATIONS MODULE**
//!
//! **CONSOLIDATED**: This module re-exports from `zero_cost_zfs_operations`
//! to eliminate duplication and provide a clean migration path.
//!
//! **Migration**: Use `zero_cost_zfs_operations` directly for new code.
//! This re-export maintains backward compatibility.

// Re-export everything from the canonical location
pub use crate::zero_cost_zfs_operations::*;

