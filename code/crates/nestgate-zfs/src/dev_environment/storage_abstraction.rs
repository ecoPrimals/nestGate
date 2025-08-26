//
// This module provides filesystem-based storage operations that simulate
// ZFS functionality for development environments without dedicated hardware.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::error::CanonicalResult as Result;
use nestgate_core::error::conversions::create_zfs_error;
use nestgate_core::error::domain_errors::ZfsOperation;
use nestgate_core::types::StorageTier;

/// Development Environment Storage Service
///
/// Provides storage operations using filesystem calls instead of ZFS commands.
/// This allows full NestGate functionality on development machines without
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
#[allow(dead_code)] // Development environment simulation - fields intentionally unused
struct SimulatedPool {
    name: String,
    path: PathBuf,
    datasets: Vec<SimulatedDataset>,
    tier: StorageTier,
    created_at: std::time::SystemTime,
}

impl SimulatedPool {
    /// Create a new simulated pool
    #[allow(dead_code)] // Development environment simulation
    pub fn new(name: String, path: PathBuf, tier: StorageTier) -> Self {
        Self {
            name,
            path,
            datasets: Vec::new(),
            tier,
            created_at: std::time::SystemTime::now(),
        }
    }
}

/// Simulated dataset using directories
#[derive(Debug, Clone)]
#[allow(dead_code)] // Development environment simulation - fields intentionally unused
struct SimulatedDataset {
    name: String,
    path: PathBuf,
    mount_point: PathBuf,
    size_bytes: u64,
    tier: StorageTier,
    properties: HashMap<String, String>,
}

impl SimulatedDataset {
    /// Create a new simulated dataset
    #[allow(dead_code)]
    pub fn new(name: String, tier: StorageTier) -> Self {
        let path = PathBuf::from(format!("/dev/datasets/{name}"));
        let mount_point = PathBuf::from(format!("/mnt/{name}"));

        Self {
            name,
            path,
            mount_point,
            tier,
            properties: HashMap::new(),
            size_bytes: 0,
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.size_bytes
    }

    #[allow(dead_code)]
    pub fn tier(&self) -> &StorageTier {
        &self.tier
    }

    #[allow(dead_code)]
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }
}

impl DevEnvironmentStorageService {
    /// Create new development storage service
    pub fn new() -> Self {
        let config = StorageAbstractionConfig::default();

        info!("🗄️ Initializing Development Environment Storage Abstraction");
        info!("📁 Base directory: {:?}", config.base_directory);

        Self {
            base_path: config.base_directory.clone(),
            pools: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: StorageAbstractionConfig) -> Self {
        info!("🗄️ Initializing Development Storage with custom config");
        info!("📁 Base directory: {:?}", config.base_directory);

        Self {
            base_path: config.base_directory.clone(),
            pools: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Initialize the storage abstraction layer
    pub async fn initialize(&self) -> Result<()> {
        // Create base directory
        if let Err(e) = tokio::fs::create_dir_all(&self.base_path).await {
            warn!("Failed to create base storage directory: {}", e);
            return Err(create_zfs_error(
                format!("Failed to create storage directory: {e}"),
                ZfsOperation::Configuration
            ));
        }

        // Create pools directory
        let pools_dir = self.base_path.join("pools");
        if let Err(e) = tokio::fs::create_dir_all(&pools_dir).await {
            warn!("Failed to create pools directory: {}", e);
            return Err(create_zfs_error(
                format!("Failed to create pools directory: {e}"),
                ZfsOperation::Configuration
            ));
        }

        // Create datasets directory
        let datasets_dir = self.base_path.join("datasets");
        if let Err(e) = tokio::fs::create_dir_all(&datasets_dir).await {
            warn!("Failed to create datasets directory: {}", e);
            return Err(modern_zfs::storage_error(
                &format!("Failed to create datasets directory: {e}"),
                None,
            ));
        }

        info!("✅ Development storage abstraction initialized");

        // Create a default development pool
        self.create_simulated_pool(
            "dev-pool",
            self.config.max_pool_size_gb * 1024 * 1024 * 1024,
        )
        .await?;

        Ok(())
    }

    /// Create a simulated storage pool
    pub async fn create_simulated_pool(&self, name: &str, size_bytes: u64) -> Result<()> {
        let mut pools = self.pools.write().await;

        let pool_path = self.base_path.join("pools").join(name);

        // Create physical directory
        if let Err(e) = tokio::fs::create_dir_all(&pool_path).await {
            warn!("Failed to create pool directory: {}", e);
            return Err(modern_zfs::storage_error(
                &format!("Failed to create pool directory: {e}"),
                Some(name),
            ));
        }

        let pool = SimulatedPool {
            name: name.to_string(),
            path: pool_path,
            datasets: Vec::new(),
            tier: StorageTier::Hot, // Default to hot tier
            created_at: std::time::SystemTime::now(),
        };

        pools.insert(name.to_string(), pool);

        if self.config.verbose_logging {
            info!(
                "🏊 Created simulated pool: {} ({}GB)",
                name,
                size_bytes / (1024 * 1024 * 1024)
            );
        }

        Ok(())
    }

    /// Create a simulated dataset
    pub async fn create_simulated_dataset(
        &self,
        pool_name: &str,
        dataset_name: &str,
        tier: StorageTier,
    ) -> Result<()> {
        let mut pools = self.pools.write().await;

        let pool = pools.get_mut(pool_name).ok_or_else(|| {
            modern_zfs::pool_error(
                &format!("Pool not found: {pool_name}"),
                nestgate_core::error::domain_errors::ZfsOperation::SystemCheck,
                Some(pool_name),
            )
        })?;

        let dataset_path = pool.path.join(dataset_name);
        let mount_point = self
            .base_path
            .join("datasets")
            .join(format!("{pool_name}_{dataset_name}"));

        // Create physical directories
        if let Err(e) = tokio::fs::create_dir_all(&dataset_path).await {
            warn!("Failed to create dataset directory: {}", e);
            return Err(modern_zfs::storage_error(
                &format!("Failed to create dataset directory: {e}"),
                Some(pool_name),
            ));
        }

        if let Err(e) = tokio::fs::create_dir_all(&mount_point).await {
            warn!("Failed to create mount point: {}", e);
            return Err(modern_zfs::storage_error(
                &format!("Failed to create mount point: {e}"),
                Some(pool_name),
            ));
        }

        let dataset = SimulatedDataset {
            name: dataset_name.to_string(),
            path: dataset_path,
            mount_point,
            size_bytes: 0,
            tier,
            properties: HashMap::new(),
        };

        pool.datasets.push(dataset);

        if self.config.verbose_logging {
            info!(
                "📊 Created simulated dataset: {}/{} (tier: {:?})",
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
             - Base Directory: {:?}\n\
             - Total Pools: {}\n\
             - Total Datasets: {}\n\
             - Total Size: {}GB\n\
             - Used Size: {}GB\n\
             - Available: {}GB\n\
             - ZFS Features Simulated: {}\n\
             - Pool Names: {:?}",
            self.base_path,
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
pub struct StorageStats {
    pub total_pools: usize,
    pub total_datasets: usize,
    pub total_size_bytes: u64,
    pub total_used_bytes: u64,
    pub available_bytes: u64,
    pub abstraction_type: String,
}

impl Default for DevEnvironmentStorageService {
    fn default() -> Self {
        Self::new()
    }
}

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
                format!("Task execution failed: {:?}", e),
                "async_task".to_string(),
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
                format!("Task execution failed: {:?}", e),
                "async_task".to_string(),
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
