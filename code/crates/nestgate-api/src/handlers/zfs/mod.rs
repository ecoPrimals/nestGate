//! **ZFS HANDLERS MODULE**
//!
//! This module provides comprehensive ZFS (ZFS File System) integration
//! including basic operations, universal pools, and zero-cost factory patterns.

/// **ZFS BASIC OPERATIONS**
///
/// Core ZFS operations and API endpoints.
pub mod basic; // Core ZFS operations

/// **ZFS TYPES**
///
/// Common types and structures used across ZFS operations.
pub mod types;

/// **UNIVERSAL POOLS**
///
/// New storage-agnostic pools handler for unified storage management.
pub mod universal_pools; // New storage-agnostic pools handler

/// **ZERO-COST FACTORY**
///
/// Zero-cost ZFS service factory for high-performance operations.
pub mod zero_cost_factory; // Zero-cost ZFS service factory

// Re-export from basic module (avoiding ambiguous types)
pub use basic::{
    create_dataset, create_pool, create_snapshot, delete_dataset, delete_pool, delete_snapshot,
    get_dataset, get_dataset_properties, get_performance_analytics, get_pool, get_pool_status,
    get_zfs_health, list_datasets, list_pools, list_snapshots, predict_tier,
    set_dataset_properties, trigger_optimization,
};

// Re-export types from types module
pub use types::*;

pub use universal_pools::*;
pub use zero_cost_factory::*;
