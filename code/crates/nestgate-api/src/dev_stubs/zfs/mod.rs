// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZFS STUB IMPLEMENTATION - DEVELOPMENT ONLY**
//!
//! ⚠️ **WARNING: THIS IS NOT PRODUCTION CODE** ⚠️
//!
//! This module provides stub implementations for ZFS operations during development and testing.
//! All data returned is HARDCODED and does not reflect actual system state.
//!
//! **DO NOT USE IN PRODUCTION** - Use real ZFS implementations from `nestgate-zfs` crate instead.
//!
//! **Refactored**: November 19, 2025 - Split from single 1,015-line file into focused modules
//!
//! # Module Structure:
//!
//! - `config` - ZFS configuration and manager (~100 lines)
//! - `pool_ops` - Pool operations mock (~350 lines)
//! - `dataset_ops` - Dataset operations mock (~350 lines)
//! - `snapshot_ops` - Snapshot operations mock (~200 lines)
//! - `types` - Mock data types (~150 lines)
//!
//! # Production Implementations
//!
//! For production use, see:
//! - `nestgate_zfs::operations::production::ProductionZfsOperations` - Real command execution
//! - `nestgate_zfs::RealZfsOperations` - Actual ZFS commands  
//! - `nestgate_zfs::zero_cost::ProductionZfsManager` - Zero-cost production manager
//!
//! # Feature Gates
//!
//! This module is only available with the `dev-stubs` feature flag.
//! Production builds will NOT include this code.

#![cfg(feature = "dev-stubs")]

// Module declarations
mod config;
mod dataset_ops;
mod pool_ops;
mod snapshot_ops;
mod types;

// Re-export public items for backwards compatibility
pub use config::{ProductionZfsManager, ZfsConfig};
pub use types::{
    ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo, ZeroCostZfsOperations, ZfsError,
    ZfsResult,
};

// Internal re-exports for module access (used by integration within dev_stubs)

// Public trait exports for use with ProductionZfsManager
pub use dataset_ops::DatasetOperations;
pub use pool_ops::PoolOperations;
pub use snapshot_ops::SnapshotOperations;
