// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides filesystem-based storage operations that simulate
// ZFS functionality for development environments without dedicated hardware.

//! Storage Abstraction module

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::canonical_types::StorageTier;
use nestgate_core::error::CanonicalResult as Result;

/// Development Environment Storage Service
///
/// Provides storage operations using filesystem calls instead of ZFS commands.
/// This allows full `NestGate` functionality on development machines without
/// requiring dedicated ZFS pools.
pub struct DevEnvironmentStorageService {
    /// Base directory for all storage operations
    base_path: PathBuf,
    /// Simulated storage pools
    pools: Arc<RwLock<HashMap<String, SimulatedPool>>>,
    /// Configuration
    config: StorageAbstractionConfig,
}
/// Configuration for storage abstraction
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageAbstractionConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageAbstractionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for `StorageAbstraction`
pub struct StorageAbstractionConfig {
    /// Base directory for all operations
    pub base_directory: PathBuf,
    /// Maximum simulated pool size
    pub max_pool_size_gb: u64,
    /// Enable verbose logging
    pub verbose_logging: bool,
    /// Simulate ZFS features (compression, deduplication, etc.)
    pub simulate_zfs_features: bool,
}
impl Default for StorageAbstractionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            base_directory: std::env::temp_dir().join("nestgate-dev-storage"),
            max_pool_size_gb: 100, // 100GB max simulated
            verbose_logging: false,
            simulate_zfs_features: true,
        }
    }
}

/// Simulated storage pool using filesystem operations
#[derive(Debug, Clone)]
struct SimulatedPool {
    path: PathBuf,
    datasets: Vec<SimulatedDataset>,
}

/// Simulated dataset using directories
#[derive(Debug, Clone)]
struct SimulatedDataset {
    name: String,
    path: PathBuf,
    mount_point: PathBuf,
    size_bytes: u64,
    tier: StorageTier,
    properties: HashMap<String, String>,
}

impl DevEnvironmentStorageService {
    /// Create new development storage service
    pub fn new() -> Self {
        let config = StorageAbstractionConfig::default();

        info!("Initializing Development Environment Storage Abstraction");
        info!("Base directory: {:?}", config.base_directory);

        Self {
            base_path: config.base_directory.clone(),
            pools: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: StorageAbstractionConfig) -> Self {
        info!("Initializing Development Storage with custom config");
        info!("Base directory: {:?}", config.base_directory);

        Self {
            base_path: config.base_directory.clone(),
            pools: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Initialize the storage abstraction layer
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn initialize(&self) -> Result<()> {
        // Create base directory
        if let Err(e) = tokio::fs::create_dir_all(&self.base_path).await {
            warn!("Failed to create base storage directory: {}", e);
            return Err(create_zfs_error(
                format!(
                    "Failed to create storage directory: {}",
                    "actual_error_details"
                ),
                ZfsOperation::Configuration,
            ));
        }

        // Create pools directory
        let pools_dir = self.base_path.join("pools");
        if let Err(e) = tokio::fs::create_dir_all(&pools_dir).await {
            warn!("Failed to create pools directory: {}", e);
            return Err(create_zfs_error(
                format!(
                    "Failed to create pools directory: {}",
                    "actual_error_details"
                ),
                ZfsOperation::Configuration,
            ));
        }

        // Create datasets directory
        let datasets_dir = self.base_path.join("datasets");
        if let Err(e) = tokio::fs::create_dir_all(&datasets_dir).await {
            warn!("Failed to create datasets directory: {}", e);
            return Err(create_zfs_error(
                format!("Failed to create datasets directory: {e}"),
                ZfsOperation::Configuration,
            ));
        }

        info!("Development storage abstraction initialized");

        // Create a default development pool
        self.create_simulated_pool(
            "dev-pool",
            self.config.max_pool_size_gb * 1024 * 1024 * 1024,
        )
        .await?;

        Ok(())
    }

    /// Create a simulated storage pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_simulated_pool(&self, name: &str, size_bytes: u64) -> Result<()> {
        let mut pools = self.pools.write().await;

        let pool_path = self.base_path.join("pools").join(name);

        // Create physical directory
        if let Err(e) = tokio::fs::create_dir_all(&pool_path).await {
            warn!("Failed to create pool directory: {}", e);
            return Err(create_zfs_error(
                format!("Failed to create pool directory: {e}"),
                ZfsOperation::PoolCreate,
            ));
        }

        let pool = SimulatedPool {
            path: pool_path,
            datasets: Vec::new(),
        };

        pools.insert(name.to_string(), pool);

        if self.config.verbose_logging {
            info!(
                "Created simulated pool: {} ({}GB)",
                name,
                size_bytes / (1024 * 1024 * 1024)
            );
        }

        Ok(())
    }

    /// Create a simulated dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_simulated_dataset(
        &self,
        pool_name: &str,
        dataset_name: &str,
        tier: StorageTier,
    ) -> Result<()> {
        let mut pools = self.pools.write().await;

        let pool = pools.get_mut(pool_name).ok_or_else(|| {
            create_zfs_error(
                format!("Pool not found: {pool_name}"),
                ZfsOperation::DatasetCreate,
            )
        })?;

        let dataset_path = pool.path.join(dataset_name);
        let mount_point = self
            .base_path
            .join("datasets")
            .join(format!("{pool_name}_error details"));

        // Create physical directories
        if let Err(e) = tokio::fs::create_dir_all(&dataset_path).await {
            warn!("Failed to create dataset directory: {}", e);
            return Err(create_zfs_error(
                format!("Failed to create dataset directory: {e}"),
                ZfsOperation::DatasetCreate,
            ));
        }

        if let Err(e) = tokio::fs::create_dir_all(&mount_point).await {
            warn!("Failed to create mount point: {}", e);
            return Err(create_zfs_error(
                format!("Failed to create mount point: {e}"),
                ZfsOperation::DatasetCreate,
            ));
        }

        let dataset = SimulatedDataset {
            name: dataset_name.to_string(),
            path: dataset_path,
            mount_point,
            size_bytes: 0,
            tier: tier.clone(),
            properties: HashMap::new(),
        };

        pool.datasets.push(dataset);

        if self.config.verbose_logging {
            info!(
                "Created simulated dataset: {}/{} (tier: {:?})",
                pool_name, dataset_name, tier
            );
        }

        Ok(())
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> StorageStats {
        let pools = self.pools.read().await;

        let total_pools = pools.len();
        let total_size_bytes: u64 = pools
            .values()
            .map(|p| p.path.metadata().map(|m| m.len()).unwrap_or(0))
            .sum();
        let total_used_bytes: u64 = pools
            .values()
            .map(|p| p.datasets.iter().map(|d| d.size_bytes).sum::<u64>())
            .sum();
        let total_datasets: usize = pools.values().map(|p| p.datasets.len()).sum();

        StorageStats {
            total_pools,
            total_datasets,
            total_size_bytes,
            total_used_bytes,
            available_bytes: total_size_bytes - total_used_bytes,
            abstraction_type: "Development Environment".to_string(),
        }
    }

    /// Get detailed environment report
    pub async fn get_environment_report(&self) -> String {
        let pools = self.pools.read().await;
        let stats = self.get_storage_stats().await;

        format!(
            "Development Storage Environment Report:\n\
             - Base Directory: {}\n\
             - Total Pools: {}\n\
             - Total Datasets: {}\n\
             - Total Size: {}GB\n\
             - Used Size: {}GB\n\
             - Available: {}GB\n\
             - ZFS Features Simulated: {}\n\
             - Pool Names: {:?}",
            self.base_path.display(),
            stats.total_pools,
            stats.total_datasets,
            stats.total_size_bytes / (1024 * 1024 * 1024),
            stats.total_used_bytes / (1024 * 1024 * 1024),
            stats.available_bytes / (1024 * 1024 * 1024),
            self.config.simulate_zfs_features,
            pools.keys().collect::<Vec<_>>()
        )
    }
}

/// Storage statistics for development environment
#[derive(Debug, Clone)]
/// Storagestats
pub struct StorageStats {
    /// Total Pools
    pub total_pools: usize,
    /// Total Datasets
    pub total_datasets: usize,
    /// Total Size Bytes
    pub total_size_bytes: u64,
    /// Total Used Bytes
    pub total_used_bytes: u64,
    /// Available Bytes
    pub available_bytes: u64,
    /// Abstraction Type
    pub abstraction_type: String,
}
impl Default for DevEnvironmentStorageService {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Storageabstractionconfigcanonical
pub type StorageAbstractionConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageAbstractionConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_initialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let service = DevEnvironmentStorageService::new();
        let result = service.initialize().await;
        assert!(result.is_ok());

        let stats = service.get_storage_stats().await;
        assert_eq!(stats.total_pools, 1); // Should have default dev-pool
        Ok(())
    }
    #[tokio::test]
    async fn test_pool_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let service = DevEnvironmentStorageService::new();
        service.initialize().await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            nestgate_core::NestGateError::internal_error(
                format!("Task execution failed: {e}"),
                "storage-abstraction",
            )
        })?;

        let result = service
            .create_simulated_pool("test-pool", 1024 * 1024 * 1024)
            .await;
        assert!(result.is_ok());

        let stats = service.get_storage_stats().await;
        assert_eq!(stats.total_pools, 2); // dev-pool + test-pool
        Ok(())
    }

    #[tokio::test]
    async fn test_dataset_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let service = DevEnvironmentStorageService::new();
        service.initialize().await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            nestgate_core::NestGateError::internal_error(
                format!("Task execution failed: {e}"),
                "storage-abstraction",
            )
        })?;

        let result = service
            .create_simulated_dataset("dev-pool", "test-dataset", StorageTier::Hot)
            .await;
        assert!(result.is_ok());

        let report = service.get_environment_report().await;
        assert!(report.contains("Total Datasets: 1"));
        Ok(())
    }
}
