//! **ZFS HANDLERS MODULE**
//!
//! This module provides comprehensive ZFS (ZFS File System) integration
//! including basic operations, universal pools, and zero-cost factory patterns.
//!
//! **⚠️ DEVELOPMENT STUB HANDLERS ⚠️**
//!
//! These handlers were designed for the stub ZFS API and are **development-only**.
//! For production ZFS operations, use `nestgate_zfs` crate directly.

/// **ZFS BASIC OPERATIONS** (Development Only)
///
/// Core ZFS operations and API endpoints.
/// **Requires `dev-stubs` feature** - uses stub API, not production ZFS.
#[cfg(feature = "dev-stubs")]
pub mod basic;

/// **ZFS TYPES**
///
/// Common types and structures used across ZFS operations.
/// Conditionally uses real or stub types based on features.
pub mod types;

/// **UNIVERSAL POOLS** (Development Only)
///
/// New storage-agnostic pools handler for unified storage management.
/// **Requires `dev-stubs` feature** - uses stub API, not production ZFS.
#[cfg(feature = "dev-stubs")]
pub mod universal_pools;

/// **ZERO-COST FACTORY**
///
/// Zero-cost ZFS service factory for high-performance operations.
/// Uses conditional compilation for dev/prod.
pub mod zero_cost_factory;

/// **PRODUCTION PLACEHOLDERS**
///
/// Placeholder handlers for production builds without dev-stubs.
/// Return helpful error messages directing to real ZFS integration.
#[cfg(not(feature = "dev-stubs"))]
pub mod production_placeholders;

// Re-export from basic module (avoiding ambiguous types)
#[cfg(feature = "dev-stubs")]
pub use basic::{
    create_dataset, create_pool, create_snapshot, delete_dataset, delete_pool, delete_snapshot,
    get_dataset, get_dataset_properties, get_performance_analytics, get_pool, get_pool_status,
    get_zfs_health, list_datasets, list_pools, list_snapshots, predict_tier,
    set_dataset_properties, trigger_optimization,
};

// Re-export types from types module
pub use types::*;

#[cfg(feature = "dev-stubs")]
pub use universal_pools::*;

// Production placeholders - same names, return "not implemented" messages
#[cfg(not(feature = "dev-stubs"))]
pub use production_placeholders::*;

pub use zero_cost_factory::*;

#[cfg(test)]
mod production_handler_tests;
