// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

use super::types::{CacheConfig, StoragePool, StorageQuota, StorageServiceStats};
use crate::Result;
use crate::error::NestGateError;
use crate::services::storage::config::{StorageServiceConfig, ZfsConfig};
use std::collections::HashMap;
///
/// This module contains the core `StorageManagerService` implementation
/// extracted from the original monolithic storage.rs file.
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

// Type aliases for complex storage types to satisfy clippy
type StoragePoolMap = Arc<RwLock<HashMap<String, StoragePool>>>;
/// Type alias for StorageQuotaMap
type StorageQuotaMap = Arc<RwLock<HashMap<String, StorageQuota>>>;
/// Type alias for CacheConfigMap
type CacheConfigMap = Arc<RwLock<HashMap<String, CacheConfig>>>;

/// Storage Manager Service - Complete implementation with real ZFS integration
pub struct StorageManagerService {
    /// Service ID
    service_id: Uuid,
    /// Canonical storage manager for unified storage operations (temporarily disabled)
    // storage_manager: Arc<CanonicalStorageManager<FilesystemBackend>>,
    /// ZFS-specific configuration
    zfs_config: ZfsConfig,
    /// Storage pools tracking
    pools: StoragePoolMap,
    /// Storage quotas tracking  
    quotas: StorageQuotaMap,
    /// Cache configurations
    cache_configs: CacheConfigMap,
    /// Service statistics
    stats: Arc<RwLock<StorageServiceStats>>,
    /// Service start time
    start_time: SystemTime,
    /// Service configuration
    config: StorageServiceConfig,
}
impl StorageManagerService {
    /// Create a new Storage Manager Service with real implementations
    ///
    /// ✅ CAPABILITY-BASED: Auto-detects available backends (ZFS, filesystem)
    /// ✅ AGNOSTIC: Works on ANY filesystem
    /// ✅ OPTIMIZED: Uses ZFS features when available
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn new() -> Result<Self> {
        // ✅ DEEP DEBT: Capability-based configuration (no hardcoding)
        Self::with_config(StorageServiceConfig::with_auto_detect()).await
    }

    /// Create a new Storage Manager Service with custom configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn with_config(config: StorageServiceConfig) -> Result<Self> {
        info!("Initializing Storage Manager Service with real ZFS integration");

        // Validate configuration
        config.validate().map_err(|e| {
            NestGateError::invalid_input_with_field(
                "config".to_string(),
                format!("Invalid storage configuration: {e}"),
            )
        })?;

        // Canonical unified storage manager wiring deferred; see git history.

        // Initialize adaptive storage if feature is enabled
        let service = Self {
            service_id: Uuid::new_v4(),
            pools: Arc::new(RwLock::new(HashMap::new())),
            quotas: Arc::new(RwLock::new(HashMap::new())),
            cache_configs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(StorageServiceStats::default())),
            start_time: SystemTime::now(),
            zfs_config: config.zfs.clone(),
            config,
        };

        // Initialize service with real implementations
        service.initialize().await?;

        info!("Storage Manager Service initialized successfully with real ZFS support");
        Ok(service)
    }

    /// Initialize the storage service with real ZFS discovery
    async fn initialize(&self) -> Result<()> {
        info!("Initializing storage service components with real ZFS integration");

        // Check ZFS availability
        if self.config.auto_discover_pools {
            self.check_zfs_availability().await?;
            self.discover_zfs_pools().await?;
        }

        // Initialize quota management
        if self.config.enable_quotas {
            self.initialize_quota_management().await?;
        }

        // Initialize cache configurations
        if self.config.enable_caching {
            self.initialize_cache_management()?;
        }

        // Start background monitoring tasks
        if self.config.enable_monitoring {
            self.start_background_tasks()?;
        }

        info!("Storage service initialization complete with real ZFS support");
        Ok(())
    }

    /// Check if ZFS is available on the system
    async fn check_zfs_availability(&self) -> Result<()> {
        info!("Checking ZFS availability");

        // Check if ZFS kernel module is loaded
        match std::fs::read_to_string("/proc/modules") {
            Ok(modules) => {
                if !modules.contains("zfs") {
                    warn!("ZFS kernel module not loaded");
                    return Err(NestGateError::configuration_error_detailed(
                        "zfs_module".to_string(),
                        "ZFS kernel module is not loaded".to_string(),
                        Some("not_loaded".into()),
                        Some("loaded ZFS kernel module".into()),
                        false,
                    ));
                }
            }
            Err(_) => {
                warn!("Cannot read /proc/modules - assuming ZFS is available");
            }
        }

        // Check if zpool command is available
        match tokio::process::Command::new("which")
            .arg("zpool")
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                info!("ZFS tools available");
                Ok(())
            }
            _ => {
                warn!("ZFS tools not found in PATH");
                Err(NestGateError::configuration_error(
                    "zfs_tools",
                    "ZFS tools (zpool) not found in PATH",
                ))
            }
        }
    }

    /// Discover existing ZFS pools
    async fn discover_zfs_pools(&self) -> Result<()> {
        info!("Discovering existing ZFS pools");

        // Execute 'zpool list' to discover existing pools
        match tokio::process::Command::new("zpool")
            .args(["list", "-H", "-o", "name,size,alloc,free,health"])
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let pool_count = stdout
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .count();

                if pool_count > 0 {
                    info!("Discovered {} existing ZFS pools", pool_count);
                    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
                        let parts: Vec<&str> = line.split('\t').collect();
                        if parts.len() >= 5 {
                            info!(
                                "  Pool: {} | Size: {} | Health: {}",
                                parts[0], parts[1], parts[4]
                            );
                        }
                    }
                } else {
                    info!("No existing ZFS pools found");
                }
                Ok(())
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!("ZFS pool discovery failed: {}", stderr);
                // Don't fail initialization if no pools exist
                Ok(())
            }
            Err(e) => {
                warn!("Failed to execute zpool command: {}", e);
                // Don't fail initialization if command fails
                Ok(())
            }
        }
    }

    /// Initialize quota management
    async fn initialize_quota_management(&self) -> Result<()> {
        info!("Initializing quota management");

        // Initialize quota tracking structures
        if self.config.enable_quotas {
            info!("Quota management enabled - initializing tracking");

            // Check if ZFS quotas are supported
            match tokio::process::Command::new("zfs")
                .args(["get", "-H", "-o", "property", "quota"])
                .output()
                .await
            {
                Ok(output) if output.status.success() => {
                    info!("ZFS quota support confirmed");
                }
                _ => {
                    warn!("ZFS quota support check failed - quotas may not work properly");
                }
            }
        } else {
            info!("Quota management disabled in configuration");
        }

        Ok(())
    }

    /// Initialize cache management
    fn initialize_cache_management(&self) -> Result<()> {
        info!("Initializing cache management");

        // Initialize cache management based on configuration
        // Configure ARC (Adaptive Replacement Cache) if available
        info!("Initializing ZFS ARC cache management");

        // Check current ARC statistics
        match std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats") {
            Ok(arcstats) => {
                // Parse ARC size information
                for line in arcstats.lines() {
                    if line.contains("size") && line.contains("4") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            let arc_size = parts[2].parse::<u64>().unwrap_or(0);
                            if arc_size > 0 {
                                #[expect(
                                    clippy::cast_precision_loss,
                                    reason = "ARC byte size formatted as MB for logs only"
                                )]
                                let arc_size_f: f64 = arc_size as f64;
                                let arc_mb = arc_size_f / 1024.0 / 1024.0;
                                info!("Current ARC size: {} bytes ({arc_mb:.2} MB)", arc_size);
                            }
                        }
                        break;
                    }
                }
            }
            Err(_) => {
                info!("ARC statistics not available (ZFS may not be running)");
            }
        }

        // Initialize cache monitoring
        info!("Cache monitoring initialized");

        Ok(())
    }

    /// Start background monitoring tasks
    fn start_background_tasks(&self) -> Result<()> {
        info!("Starting background monitoring tasks");

        // Start health monitoring task
        let service_id = self.service_id;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;

                // Perform health check
                match tokio::process::Command::new("zpool")
                    .args(["status", "-x"])
                    .output()
                    .await
                {
                    Ok(output) if output.status.success() => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        if stdout.contains("all pools are healthy") {
                            debug!("All ZFS pools healthy (service: {})", service_id);
                        } else {
                            warn!("ZFS pool health issues detected (service: {})", service_id);
                        }
                    }
                    _ => {
                        debug!("ZFS health check skipped (service: {})", service_id);
                    }
                }
            }
        });

        // Start performance monitoring task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;

                // Monitor ZFS ARC efficiency
                if let Ok(arcstats) = std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats") {
                    let mut hits = 0u64;
                    let mut misses = 0u64;

                    for line in arcstats.lines() {
                        if line.contains("hits") && line.contains("4") {
                            if let Some(hit_str) = line.split_whitespace().nth(2) {
                                hits = hit_str.parse().unwrap_or(0);
                            }
                        } else if line.contains("misses") && line.contains("4") {
                            if let Some(miss_str) = line.split_whitespace().nth(2) {
                                misses = miss_str.parse().unwrap_or(0);
                            }
                        }
                    }

                    if hits + misses > 0 {
                        let total = hits + misses;
                        // Integer path avoids u64→f64 ratio casts; `total` > 0.
                        let hit_rate =
                            (u128::from(hits) * 10_000 / u128::from(total)) as f64 / 100.0;
                        debug!("ZFS ARC hit rate: {:.2}%", hit_rate);
                    }
                }
            }
        });

        info!("Background monitoring tasks started");
        Ok(())
    }

    /// Get service ID
    #[must_use]
    pub const fn service_id(&self) -> Uuid {
        self.service_id
    }

    /// Get service start time
    #[must_use]
    pub const fn start_time(&self) -> SystemTime {
        self.start_time
    }

    /// Get service configuration
    #[must_use]
    pub const fn config(&self) -> &StorageServiceConfig {
        &self.config
    }

    /// Get service statistics
    pub async fn stats(&self) -> StorageServiceStats {
        self.stats.read().await.clone()
    }

    /// Get all storage pools
    pub async fn get_pools(&self) -> HashMap<String, StoragePool> {
        self.pools.read().await.clone()
    }

    /// Get all storage quotas
    pub async fn get_quotas(&self) -> HashMap<String, StorageQuota> {
        self.quotas.read().await.clone()
    }

    /// Get all cache configurations
    pub async fn get_cache_configs(&self) -> HashMap<String, CacheConfig> {
        self.cache_configs.read().await.clone()
    }

    /// Get storage manager reference
    /// Get ZFS configuration
    #[must_use]
    pub const fn zfs_config(&self) -> &ZfsConfig {
        &self.zfs_config
    }

    /// Check if ZFS is enabled
    #[must_use]
    pub const fn is_zfs_enabled(&self) -> bool {
        // Check if ZFS binary is configured (indicating ZFS is intended to be used)
        !self.zfs_config.zfs_binary.is_empty()
    }

    /// Check if adaptive storage is available.
    ///
    /// Adaptive storage is accessed through the universal storage interface.
    /// Returns `false` until the universal storage backend is configured.
    #[must_use]
    pub const fn is_adaptive_storage_available(&self) -> bool {
        false
    }

    // Adaptive storage methods removed: `service_integration` module was migrated
    // to `universal_storage`. Adaptive storage operations are available through
    // the universal storage interface in `crate::universal_storage`.

    // ==========================================================================
    // Dataset & Object Operations (delegated to operations modules)
    // ==========================================================================

    /// Create a new dataset
    ///
    /// # Errors
    ///
    /// Returns error if dataset already exists or creation fails
    pub async fn create_dataset(
        &self,
        name: &str,
        params: crate::rpc::tarpc_types::DatasetParams,
    ) -> Result<crate::rpc::tarpc_types::DatasetInfo> {
        super::operations::datasets::create_dataset(&self.config, name, params).await
    }

    /// List all datasets
    ///
    /// # Errors
    ///
    /// Returns error if listing fails
    pub async fn list_datasets(&self) -> Result<Vec<crate::rpc::tarpc_types::DatasetInfo>> {
        super::operations::datasets::list_datasets(&self.config).await
    }

    /// Store an object in a dataset
    ///
    /// Accepts `impl AsRef<[u8]>` to avoid forcing `.to_vec()` in hot paths.
    ///
    /// # Errors
    ///
    /// Returns error if storage fails
    pub async fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: impl AsRef<[u8]>,
    ) -> Result<crate::rpc::tarpc_types::ObjectInfo> {
        super::operations::objects::store_object(&self.config, dataset, key, data).await
    }

    /// Retrieve an object from a dataset
    ///
    /// Returns `Bytes` (zero-copy) instead of `Vec<u8>` in hot paths.
    ///
    /// # Errors
    ///
    /// Returns error if object not found or retrieval fails
    pub async fn retrieve_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> Result<(bytes::Bytes, crate::rpc::tarpc_types::ObjectInfo)> {
        super::operations::objects::retrieve_object(&self.config, dataset, key).await
    }

    /// Delete an object from a dataset
    ///
    /// # Errors
    ///
    /// Returns error if object not found or deletion fails
    pub async fn delete_object(&self, dataset: &str, key: &str) -> Result<()> {
        super::operations::objects::delete_object(&self.config, dataset, key).await
    }

    /// Get a single dataset by name.
    ///
    /// # Errors
    ///
    /// Returns error if dataset not found.
    pub async fn get_dataset(&self, name: &str) -> Result<crate::rpc::tarpc_types::DatasetInfo> {
        super::operations::datasets::get_dataset(&self.config, name).await
    }

    /// List objects in a dataset with optional prefix filter and limit.
    ///
    /// # Errors
    ///
    /// Returns error if the dataset directory cannot be read.
    pub async fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<crate::rpc::tarpc_types::ObjectInfo>> {
        super::operations::objects::list_objects(&self.config, dataset, prefix, limit).await
    }

    /// Get object metadata without reading the body.
    ///
    /// # Errors
    ///
    /// Returns error if object not found.
    pub async fn get_object_metadata(
        &self,
        dataset: &str,
        key: &str,
    ) -> Result<crate::rpc::tarpc_types::ObjectInfo> {
        super::operations::objects::get_object_metadata(&self.config, dataset, key).await
    }

    /// Delete a dataset and all its objects
    ///
    /// # Errors
    ///
    /// Returns error if dataset not found or deletion fails
    pub async fn delete_dataset(&self, name: &str) -> Result<()> {
        super::operations::datasets::delete_dataset(&self.config, name).await
    }
}

// Note: Default trait is intentionally not implemented for StorageManagerService
// Use StorageManagerService::new().await instead for proper async initialization

// ---------------------------------------------------------------------------
// StorageBackend implementation — resolves NG-01
// ---------------------------------------------------------------------------

impl nestgate_rpc::rpc::storage_backend::StorageBackend for StorageManagerService {
    fn create_dataset(
        &self,
        name: &str,
        params: crate::rpc::tarpc_types::DatasetParams,
    ) -> impl std::future::Future<Output = Result<crate::rpc::tarpc_types::DatasetInfo>> + Send + '_
    {
        let name = name.to_owned();
        async move { self.create_dataset(&name, params).await }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn list_datasets(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<crate::rpc::tarpc_types::DatasetInfo>>> + Send + '_
    {
        async move { self.list_datasets().await }
    }

    fn get_dataset(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<crate::rpc::tarpc_types::DatasetInfo>> + Send + '_
    {
        let name = name.to_owned();
        async move { Self::get_dataset(self, &name).await }
    }

    fn delete_dataset(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<crate::rpc::tarpc_types::OperationResult>> + Send + '_
    {
        let name = name.to_owned();
        async move {
            Self::delete_dataset(self, &name).await?;
            Ok(crate::rpc::tarpc_types::OperationResult {
                success: true,
                message: format!("Dataset {name} deleted successfully"),
                metadata: std::collections::HashMap::new(),
            })
        }
    }

    fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: bytes::Bytes,
        _metadata: Option<std::collections::HashMap<String, String>>,
    ) -> impl std::future::Future<Output = Result<crate::rpc::tarpc_types::ObjectInfo>> + Send + '_
    {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move { self.store_object(&dataset, &key, data).await }
    }

    fn retrieve_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl std::future::Future<Output = Result<bytes::Bytes>> + Send + '_ {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move {
            let (bytes, _info) = Self::retrieve_object(self, &dataset, &key).await?;
            Ok(bytes)
        }
    }

    fn get_object_metadata(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl std::future::Future<Output = Result<crate::rpc::tarpc_types::ObjectInfo>> + Send + '_
    {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move { Self::get_object_metadata(self, &dataset, &key).await }
    }

    fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Vec<crate::rpc::tarpc_types::ObjectInfo>>> + Send + '_
    {
        let dataset = dataset.to_owned();
        let prefix = prefix.map(str::to_owned);
        async move { Self::list_objects(self, &dataset, prefix.as_deref(), limit).await }
    }

    fn delete_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl std::future::Future<Output = Result<crate::rpc::tarpc_types::OperationResult>> + Send + '_
    {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move {
            Self::delete_object(self, &dataset, &key).await?;
            Ok(crate::rpc::tarpc_types::OperationResult {
                success: true,
                message: format!("Object {dataset}/{key} deleted successfully"),
                metadata: std::collections::HashMap::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_service_creation() {
        // Create a test configuration that doesn't require ZFS system checks
        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false; // Skip ZFS availability checks
        config.enable_quotas = false; // Skip quota initialization
        config.enable_caching = false; // Skip cache initialization
        config.enable_monitoring = false; // Skip monitoring tasks

        let service = StorageManagerService::with_config(config).await;
        if let Err(ref e) = service {
            println!("StorageManagerService creation error: {e:?}");
        }
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_storage_service_with_config() {
        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false; // Skip ZFS availability checks
        config.enable_quotas = false; // Skip quota initialization
        config.enable_caching = false; // Skip cache initialization
        config.enable_monitoring = false; // Skip monitoring tasks

        let service = StorageManagerService::with_config(config).await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn storage_manager_accessors_after_init() {
        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false;
        config.enable_quotas = false;
        config.enable_caching = false;
        config.enable_monitoring = false;

        let svc = StorageManagerService::with_config(config.clone())
            .await
            .expect("init");
        assert_eq!(*svc.config().base_path, config.base_path);
        assert!(svc.is_zfs_enabled());
        assert!(!svc.is_adaptive_storage_available());
        let stats = svc.stats().await;
        assert_eq!(stats.total_operations, 0);
        assert!(svc.get_pools().await.is_empty());
        assert!(svc.get_quotas().await.is_empty());
    }

    #[tokio::test]
    async fn with_config_rejects_invalid_max_concurrent_operations() {
        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false;
        config.enable_quotas = false;
        config.enable_caching = false;
        config.enable_monitoring = false;
        config.max_concurrent_operations = 0;
        let err = StorageManagerService::with_config(config).await;
        assert!(err.is_err(), "expected validation error");
    }

    #[tokio::test]
    async fn storage_backend_trait_methods_are_reachable() {
        use nestgate_rpc::rpc::StorageBackend;

        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false;
        config.enable_quotas = false;
        config.enable_caching = false;
        config.enable_monitoring = false;

        let svc = StorageManagerService::with_config(config)
            .await
            .expect("init");
        let _ = StorageBackend::list_datasets(&svc).await;
        let _ = svc.start_time();
        let _ = svc.service_id();
        let _ = svc.get_cache_configs().await;
    }

    #[tokio::test]
    async fn storage_service_with_cache_enabled_initializes() {
        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false;
        config.enable_quotas = false;
        config.enable_caching = true;
        config.enable_monitoring = false;

        let svc = StorageManagerService::with_config(config).await;
        assert!(svc.is_ok(), "{:?}", svc.as_ref().err());
    }
}
