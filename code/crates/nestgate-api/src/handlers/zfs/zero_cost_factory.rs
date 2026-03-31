// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZERO-COST ZFS SERVICE FACTORY**
//!
//! Factory pattern for creating zero-cost ZFS service abstractions
//! with compile-time optimizations and type safety.
//!
//! **⚠️ DEVELOPMENT ONLY ⚠️**
//!
//! This module is only available with `dev-stubs` feature.

#[cfg(feature = "dev-stubs")]
use crate::dev_stubs::zfs::ZeroCostZfsOperations;

#[cfg(feature = "dev-stubs")]
/// **ZERO-COST NATIVE ZFS SERVICE** (Development Only)
///
/// High-performance ZFS service implementation with zero-cost abstractions.
#[derive(Debug, Clone)]
/// Service implementation for `ZeroCostNativeZfs`
pub struct ZeroCostNativeZfsService<const MAX_POOLS: usize> {
    /// ZFS operations interface
    _inner: ZeroCostZfsOperations,
}

#[cfg(feature = "dev-stubs")]
impl<const MAX_POOLS: usize> Default for ZeroCostNativeZfsService<MAX_POOLS> {
    /// Returns the default instance
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
            _inner: ZeroCostZfsOperations::new(),
        }
    }
}

#[cfg(not(feature = "dev-stubs"))]
/// **ZERO-COST NATIVE ZFS SERVICE** (Production Placeholder)
///
/// Placeholder for production builds.
#[derive(Debug, Clone)]
/// Service implementation for `ZeroCostNativeZfs`
pub struct ZeroCostNativeZfsService<const MAX_POOLS: usize>;

#[cfg(not(feature = "dev-stubs"))]
impl<const MAX_POOLS: usize> Default for ZeroCostNativeZfsService<MAX_POOLS> {
    /// Returns the default instance
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

#[cfg(all(test, not(feature = "dev-stubs")))]
mod tests {
    use super::ZeroCostNativeZfsService;

    #[test]
    fn production_placeholder_new_and_default() {
        let _a = ZeroCostNativeZfsService::<4>::new();
        let _b = ZeroCostNativeZfsService::<4>;
    }
}

#[cfg(all(test, feature = "dev-stubs"))]
mod dev_stubs_tests {
    use super::ZeroCostNativeZfsService;

    #[test]
    fn dev_stubs_new_and_default() {
        let _a = ZeroCostNativeZfsService::<8>::new();
        let _b = ZeroCostNativeZfsService::<8>::default();
    }
}
