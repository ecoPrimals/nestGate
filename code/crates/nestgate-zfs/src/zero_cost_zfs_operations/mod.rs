// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZERO-COST ZFS OPERATIONS MODULE**
//! This module provides zero-cost abstractions for ZFS operations
//! Replaces `Arc<dyn>` patterns with compile-time dispatch for maximum performance

mod manager;
mod traits;
mod types;
mod utilities;

// Re-export main types and traits
pub use manager::ZeroCostZfsManager;
pub use traits::ZeroCostZfsOperations;
pub use types::{
    DatasetInfoMap, PoolInfoMap, SnapshotInfoMap, ZeroCostDatasetInfo, ZeroCostPoolInfo,
    ZeroCostSnapshotInfo,
};
pub use utilities::{ZfsBenchmark, ZfsMigrationGuide};

// Re-export type aliases for different deployment sizes
pub use manager::{
    DevelopmentZfsManager, EnterpriseZfsManager, HighPerformanceZfsManager, ProductionZfsManager,
    TestingZfsManager,
};

// Test modules
#[cfg(test)]
mod manager_tests;

#[cfg(test)]
mod manager_tests_expanded;

#[cfg(test)]
mod manager_tests_additional;
