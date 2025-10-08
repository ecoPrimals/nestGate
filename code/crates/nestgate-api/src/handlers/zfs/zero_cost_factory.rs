//! **ZERO-COST ZFS SERVICE FACTORY**
//!
//! Factory pattern for creating zero-cost ZFS service abstractions
//! with compile-time optimizations and type safety.
//!
//! **⚠️ DEVELOPMENT ONLY ⚠️**
//!
//! This module is only available with `dev-stubs` feature.

#[cfg(feature = "dev-stubs")]
use crate::handlers::zfs_stub::ZeroCostZfsOperations;

#[cfg(feature = "dev-stubs")]
/// **ZERO-COST NATIVE ZFS SERVICE** (Development Only)
///
/// High-performance ZFS service implementation with zero-cost abstractions.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Inner field used for ZFS operations
pub struct ZeroCostNativeZfsService<const MAX_POOLS: usize> {
    /// ZFS operations interface
    inner: ZeroCostZfsOperations,
}

#[cfg(feature = "dev-stubs")]
impl<const MAX_POOLS: usize> Default for ZeroCostNativeZfsService<MAX_POOLS> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "dev-stubs")]
impl<const MAX_POOLS: usize> ZeroCostNativeZfsService<MAX_POOLS> {
    /// Create a new zero-cost native ZFS service instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: ZeroCostZfsOperations::new(),
        }
    }
}

#[cfg(not(feature = "dev-stubs"))]
/// **ZERO-COST NATIVE ZFS SERVICE** (Production Placeholder)
///
/// Placeholder for production builds.
#[derive(Debug, Clone)]
pub struct ZeroCostNativeZfsService<const MAX_POOLS: usize>;

#[cfg(not(feature = "dev-stubs"))]
impl<const MAX_POOLS: usize> Default for ZeroCostNativeZfsService<MAX_POOLS> {
    fn default() -> Self {
        Self
    }
}

#[cfg(not(feature = "dev-stubs"))]
impl<const MAX_POOLS: usize> ZeroCostNativeZfsService<MAX_POOLS> {
    /// Create a new placeholder service
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
