// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    async_fn_in_trait,
    reason = "Native async fn on UniversalZfsService; use UniversalZfsServiceEnum or other concrete types, not dyn"
)]
// **CANONICAL ZFS SERVICE TRAIT - COMPREHENSIVE UNIFICATION**
//
// This trait provides the complete canonical interface for all ZFS backend implementations.
// It unifies all methods from native, remote, and fail-safe backends into a single consistent API.

//! Traits module

use std::collections::HashMap;

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

use crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService;
use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;

/// **CANONICAL UNIVERSAL ZFS SERVICE TRAIT**
///
/// This trait defines the complete interface that all ZFS backend implementations must provide.
/// It includes all methods from `native_real`, remote, and `fail_safe` implementations.
/// Async methods use native `async fn` (Rust 1.75+); use `UniversalZfsServiceEnum` or concrete
/// types for storage — `dyn UniversalZfsService` is not supported with async fn in traits.
pub trait UniversalZfsService: Send + Sync {
    // ==================== CORE SERVICE METHODS ====================
    /// Get the service name
    fn service_name(&self) -> &str;

    /// Get the service version  
    fn service_version(&self) -> &str;

    /// Perform a health check on the service
    async fn health_check(&self) -> UniversalZfsResult<HealthStatus>;

    /// Get service metrics
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics>;

    /// Check if the service is available
    async fn is_available(&self) -> bool;

    /// Shutdown the service gracefully
    async fn shutdown(&self) -> UniversalZfsResult<()>;

    // ==================== POOL OPERATIONS ====================

    /// List all pools
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>>;

    /// Create a new pool
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo>;

    /// Get information about a specific pool
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>>;

    /// Destroy a pool
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()>;

    /// Scrub a pool (data integrity check)
    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()>;

    /// Get pool status information
    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String>;

    // ==================== DATASET OPERATIONS ====================

    /// List all datasets
    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>>;

    /// Create a new dataset
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo>;

    /// Get information about a specific dataset
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>>;

    /// Destroy a dataset
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()>;

    /// Set dataset properties
    async fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()>;

    /// Get dataset properties
    async fn get_dataset_properties(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>>;

    // ==================== SNAPSHOT OPERATIONS ====================

    /// List all snapshots
    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>>;

    /// Create a snapshot
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo>;

    /// List snapshots for a specific dataset
    async fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>>;

    /// Destroy a snapshot
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()>;

    // ==================== OPTIMIZATION & CONFIGURATION ====================

    /// Optimize ZFS configuration
    async fn optimize(&self) -> UniversalZfsResult<String>;

    /// Get optimization analytics
    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value>;

    /// Predict optimal tier for data
    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String>;

    /// Get current configuration
    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value>;

    /// Update configuration
    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()>;
}

/// **DYN-COMPATIBLE ZFS SERVICE WRAPPER**
/// Wrapper enum for dynamic dispatch of ZFS services
#[derive(Debug)]
/// Dynzfsservice
pub enum DynZfsService {
    /// Native ZFS service implementation
    Native(NativeZfsService),
    /// Fail-safe ZFS service with circuit breaker
    FailSafe(FailSafeZfsService),
}
impl DynZfsService {
    /// Service name
    #[must_use]
    pub fn service_name(&self) -> &str {
        match self {
            Self::Native(service) => service.service_name(),
            Self::FailSafe(service) => service.service_name(),
        }
    }

    /// Service version
    #[must_use]
    pub fn service_version(&self) -> &str {
        match self {
            Self::Native(service) => service.service_version(),
            Self::FailSafe(service) => service.service_version(),
        }
    }

    /// Health check
    pub async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        match self {
            Self::Native(service) => service.health_check().await,
            Self::FailSafe(service) => service.health_check().await,
        }
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        match self {
            Self::Native(service) => service.get_metrics().await,
            Self::FailSafe(service) => service.get_metrics().await,
        }
    }

    /// Check if available
    pub async fn is_available(&self) -> bool {
        match self {
            Self::Native(service) => service.is_available().await,
            Self::FailSafe(service) => service.is_available().await,
        }
    }

    /// Shutdown
    pub async fn shutdown(&self) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.shutdown().await,
            Self::FailSafe(service) => service.shutdown().await,
        }
    }

    /// Create pool
    pub async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        match self {
            Self::Native(service) => service.create_pool(config).await,
            Self::FailSafe(service) => service.create_pool(config).await,
        }
    }

    /// Get pool
    pub async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        match self {
            Self::Native(service) => service.get_pool(name).await,
            Self::FailSafe(service) => service.get_pool(name).await,
        }
    }

    /// Destroy pool
    pub async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_pool(name).await,
            Self::FailSafe(service) => service.destroy_pool(name).await,
        }
    }

    /// Create snapshot
    pub async fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> UniversalZfsResult<SnapshotInfo> {
        match self {
            Self::Native(service) => service.create_snapshot(config).await,
            Self::FailSafe(service) => service.create_snapshot(config).await,
        }
    }

    /// Destroy snapshot
    pub async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_snapshot(name).await,
            Self::FailSafe(service) => service.destroy_snapshot(name).await,
        }
    }

    /// List datasets
    pub async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        match self {
            Self::Native(service) => service.list_datasets().await,
            Self::FailSafe(service) => service.list_datasets().await,
        }
    }

    /// Scrub pool
    pub async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.scrub_pool(name).await,
            Self::FailSafe(service) => service.scrub_pool(name).await,
        }
    }

    /// Get pool status
    pub async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.get_pool_status(name).await,
            Self::FailSafe(service) => service.get_pool_status(name).await,
        }
    }

    /// Create dataset
    pub async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        match self {
            Self::Native(service) => service.create_dataset(config).await,
            Self::FailSafe(service) => service.create_dataset(config).await,
        }
    }

    /// Get dataset
    pub async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        match self {
            Self::Native(service) => service.get_dataset(name).await,
            Self::FailSafe(service) => service.get_dataset(name).await,
        }
    }

    /// Destroy dataset
    pub async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_dataset(name).await,
            Self::FailSafe(service) => service.destroy_dataset(name).await,
        }
    }

    /// Set dataset properties
    pub async fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => {
                service
                    .set_dataset_properties(dataset_name, properties)
                    .await
            }
            Self::FailSafe(service) => {
                service
                    .set_dataset_properties(dataset_name, properties)
                    .await
            }
        }
    }

    /// Get dataset properties
    pub async fn get_dataset_properties(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        match self {
            Self::Native(service) => service.get_dataset_properties(dataset_name).await,
            Self::FailSafe(service) => service.get_dataset_properties(dataset_name).await,
        }
    }

    /// List snapshots
    pub async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        match self {
            Self::Native(service) => service.list_snapshots().await,
            Self::FailSafe(service) => service.list_snapshots().await,
        }
    }

    /// List dataset snapshots
    pub async fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        match self {
            Self::Native(service) => service.list_dataset_snapshots(dataset_name).await,
            Self::FailSafe(service) => service.list_dataset_snapshots(dataset_name).await,
        }
    }

    /// List pools
    pub async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        match self {
            Self::Native(service) => service.list_pools().await,
            Self::FailSafe(service) => service.list_pools().await,
        }
    }

    /// Optimize
    pub async fn optimize(&self) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.optimize().await,
            Self::FailSafe(service) => service.optimize().await,
        }
    }

    /// Get optimization analytics
    pub async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        match self {
            Self::Native(service) => service.get_optimization_analytics().await,
            Self::FailSafe(service) => service.get_optimization_analytics().await,
        }
    }

    /// Predict tier
    pub async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.predict_tier(file_path).await,
            Self::FailSafe(service) => service.predict_tier(file_path).await,
        }
    }

    /// Get configuration
    pub async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        match self {
            Self::Native(service) => service.get_configuration().await,
            Self::FailSafe(service) => service.get_configuration().await,
        }
    }

    /// Update configuration
    pub async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.update_configuration(config).await,
            Self::FailSafe(service) => service.update_configuration(config).await,
        }
    }
}
