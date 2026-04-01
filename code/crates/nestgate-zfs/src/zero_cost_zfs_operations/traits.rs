// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZERO-COST ZFS OPERATIONS TRAITS**
//! This module defines the core trait for zero-cost ZFS operations
//! Replaces `Arc<dyn>` patterns with compile-time dispatch for maximum performance

use nestgate_core::Result;
use nestgate_core::canonical_types::StorageTier;

/// **ZERO-COST ZFS OPERATIONS TRAIT**
/// Replaces `Arc<dyn ZfsOperations>` with native async methods
pub trait ZeroCostZfsOperations<
    const MAX_POOLS: usize = 100,
    const MAX_DATASETS: usize = 10_000,
    const MAX_SNAPSHOTS: usize = 100_000,
>
{
    /// Type alias for Pool
    type Pool: Clone + Send + Sync + 'static;
    /// Type alias for Dataset
    type Dataset: Clone + Send + Sync + 'static;
    /// Type alias for Snapshot
    type Snapshot: Clone + Send + Sync + 'static;
    /// Type alias for Properties
    type Properties: Clone + Send + Sync + 'static;
    /// Type alias for Error
    type Error: Send + Sync + 'static;

    /// Create ZFS pool - native async, no boxing
    fn create_pool(
        &self,
        name: &str,
        devices: &[&str],
    ) -> impl std::future::Future<Output = Result<Self::Pool>> + Send;

    /// Create dataset - compile-time specialization
    fn create_dataset(
        &self,
        pool: &Self::Pool,
        name: &str,
        tier: StorageTier,
    ) -> impl std::future::Future<Output = Result<Self::Dataset>> + Send;

    /// Create snapshot - zero-cost abstraction
    fn create_snapshot(
        &self,
        dataset: &Self::Dataset,
        name: &str,
    ) -> impl std::future::Future<Output = Result<Self::Snapshot>> + Send;

    /// Get pool properties - direct access
    fn get_pool_properties(
        &self,
        pool: &Self::Pool,
    ) -> impl std::future::Future<Output = Result<Self::Properties>> + Send;

    /// List pools with compile-time limits
    fn list_pools(&self) -> impl std::future::Future<Output = Result<Vec<Self::Pool>>> + Send;

    /// List datasets with compile-time limits
    fn list_datasets(
        &self,
        pool: &Self::Pool,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Dataset>>> + Send;

    /// List snapshots with compile-time limits
    fn list_snapshots(
        &self,
        dataset: &Self::Dataset,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Snapshot>>> + Send;

    /// Check pool capacity at compile-time
    fn can_create_pool(&self) -> bool {
        MAX_POOLS > 0
    }

    /// Check dataset capacity at compile-time
    fn can_create_dataset(&self) -> bool {
        MAX_DATASETS > 0
    }

    /// Check snapshot capacity at compile-time
    fn can_create_snapshot(&self) -> bool {
        MAX_SNAPSHOTS > 0
    }

    /// Get max pools at compile-time
    #[must_use]
    fn max_pools() -> usize {
        MAX_POOLS
    }

    /// Get max datasets at compile-time
    #[must_use]
    fn max_datasets() -> usize {
        MAX_DATASETS
    }

    /// Get max snapshots at compile-time
    #[must_use]
    fn max_snapshots() -> usize {
        MAX_SNAPSHOTS
    }
}
