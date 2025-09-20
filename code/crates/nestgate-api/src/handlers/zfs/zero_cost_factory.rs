//! **ZERO-COST ZFS SERVICE FACTORY**
//!
//! Factory pattern for creating zero-cost ZFS service abstractions
//! with compile-time optimizations and type safety.

use crate::handlers::zfs_stub::ZeroCostZfsOperations;

/// **ZERO-COST NATIVE ZFS SERVICE**
///
/// High-performance ZFS service implementation with zero-cost abstractions.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Inner field used for ZFS operations
pub struct ZeroCostNativeZfsService<const MAX_POOLS: usize> {
    /// ZFS operations interface
    inner: ZeroCostZfsOperations,
}

impl<const MAX_POOLS: usize> ZeroCostNativeZfsService<MAX_POOLS> {
    /// Create a new zero-cost native ZFS service instance
    pub const fn new() -> Self {
        Self {
            inner: ZeroCostZfsOperations::new(),
        }
    }
}
