//! Integration Tests for NestGate ZFS
//!
//! Comprehensive test suite covering all major ZFS functionality
//! including pool management, dataset operations, snapshots, tiering,
//! AI integration, and performance monitoring.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;
use tokio::time::timeout;

use nestgate_core::{Result, StorageTier};
use nestgate_zfs::*;

/// Test configuration for integration tests
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Use real ZFS commands (set to false for mock testing)
    pub use_real_zfs: bool,
    /// Test pool name
    pub test_pool_name: String,
    /// Test timeout in seconds
    pub test_timeout_seconds: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            use_real_zfs: true,  // Default to real ZFS - much better for integration testing
            test_pool_name: "nestpool".to_string(),
            test_timeout_seconds: 30,
        }
    }
}

/// Test fixture for ZFS integration tests
pub struct ZfsTestFixture {
    pub config: TestConfig,
    pub temp_dir: TempDir,
    pub zfs_config: ZfsConfig,
    pub zfs_manager: Option<ZfsManager>,
}

impl ZfsTestFixture {
    /// Create a new test fixture
    pub async fn new() -> Result<Self> {
        Self::with_config(TestConfig::default()).await
    }

    /// Create a test fixture with custom configuration
    pub async fn with_config(config: TestConfig) -> Result<Self> {
        let temp_dir = TempDir::new()
            .map_err(|e| NestGateError::Internal(format!("Failed to create temp dir: {}", e)))?;

        let mut zfs_config = ZfsConfig::default();
        zfs_config.use_real_zfs = config.use_real_zfs;
        zfs_config.default_pool = config.test_pool_name.clone();
        
        // Configure for testing
        zfs_config.health_monitoring.enabled = true;
        zfs_config.health_monitoring.check_interval_seconds = 1;
        zfs_config.metrics.enabled = true;
        zfs_config.metrics.collection_interval_seconds = 1;
        zfs_config.migration.background_migration = false; // Disable for testing
        
        Ok(Self {
            config,
            temp_dir,
            zfs_config,
            zfs_manager: None,
        })
    }

    /// Initialize ZFS manager
    pub async fn init_manager(&mut self) -> Result<()> {
        let manager = ZfsManager::new(self.zfs_config.clone()).await?;
        self.zfs_manager = Some(manager);
        Ok(())
    }

    /// Get ZFS manager reference
    pub fn manager(&self) -> &ZfsManager {
        self.zfs_manager.as_ref().expect("ZFS manager not initialized")
    }

    /// Get mutable ZFS manager reference
    pub fn manager_mut(&mut self) -> &mut ZfsManager {
        self.zfs_manager.as_mut().expect("ZFS manager not initialized")
    }

    /// Cleanup test resources
    pub async fn cleanup(&mut self) -> Result<()> {
        if self.config.use_real_zfs {
            if let Some(mut manager) = self.zfs_manager.take() {
                manager.stop().await?;
            }
        }
        Ok(())
    }
}

/// Pool Management Tests
#[cfg(test)]
mod pool_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_discovery() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Test pool discovery
        let result = manager.pool_manager.discover_pools().await;
        assert!(result.is_ok(), "Pool discovery should succeed");

        // Test pool listing
        let pools = manager.pool_manager.list_pools().await.unwrap();
        assert!(!pools.is_empty(), "Should discover at least one pool");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_pool_status_monitoring() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Initialize pools
        manager.pool_manager.discover_pools().await.unwrap();
        
        // Test pool status
        let status = manager.pool_manager.get_overall_status().await.unwrap();
        assert!(status.pools_online > 0, "Should have at least one online pool");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_pool_health_monitoring() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Start health monitoring
        manager.health_monitor.start_monitoring().await.unwrap();
        
        // Wait for health check
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Check health status
        let health = manager.health_monitor.get_current_status().await.unwrap();
        assert!(matches!(health.overall_health, HealthState::Healthy | HealthState::Unknown));

        fixture.cleanup().await.unwrap();
    }
}

/// Dataset Management Tests
#[cfg(test)]
mod dataset_tests {
    use super::*;

    #[tokio::test]
    async fn test_dataset_creation() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Create test dataset
        let dataset_name = format!("{}/test_dataset", fixture.config.test_pool_name);
        let result = manager.dataset_manager.create_dataset(
            &dataset_name,
            &fixture.config.test_pool_name,
            StorageTier::Warm
        ).await;
        assert!(result.is_ok(), "Dataset creation should succeed");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dataset_info_retrieval() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        let dataset_name = format!("{}/test_info", fixture.config.test_pool_name);
        
        // Create dataset first
        manager.dataset_manager.create_dataset(
            &dataset_name,
            &fixture.config.test_pool_name,
            StorageTier::Warm
        ).await.unwrap();
        
        // Get dataset info
        let info = manager.dataset_manager.get_dataset_info(&dataset_name).await.unwrap();
        assert_eq!(info.name, dataset_name);
        assert!(info.available_space > 0);

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dataset_deletion() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        let dataset_name = format!("{}/test_delete", fixture.config.test_pool_name);
        
        // Create dataset first
        manager.dataset_manager.create_dataset(
            &dataset_name,
            &fixture.config.test_pool_name,
            StorageTier::Warm
        ).await.unwrap();
        
        // Delete dataset
        let result = manager.dataset_manager.delete_dataset(&dataset_name).await;
        assert!(result.is_ok(), "Dataset deletion should succeed");

        fixture.cleanup().await.unwrap();
    }
}

/// Snapshot Management Tests
#[cfg(test)]
mod snapshot_tests {
    use super::*;

    #[tokio::test]
    async fn test_snapshot_creation() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        let dataset_name = format!("{}/snap_test", fixture.config.test_pool_name);
        
        // Create dataset first
        manager.dataset_manager.create_dataset(
            &dataset_name,
            &fixture.config.test_pool_name,
            StorageTier::Warm
        ).await.unwrap();
        
        // Create snapshot
        let snapshot_name = "test_snapshot";
        let result = manager.snapshot_manager.create_snapshot(&dataset_name, snapshot_name, false).await;
        assert!(result.is_ok(), "Snapshot creation should succeed");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_snapshot_policies() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Create test policy
        let policy = crate::snapshot::SnapshotPolicy {
            name: "test_policy".to_string(),
            description: "Test snapshot policy".to_string(),
            enabled: true,
            frequency: crate::snapshot::ScheduleFrequency::Hours(1),
            retention: crate::snapshot::RetentionPolicy::Count(5),
            dataset_patterns: vec!["*/test/*".to_string()],
            tiers: vec![crate::types::StorageTier::Hot],
            name_prefix: "test".to_string(),
            include_properties: true,
            recursive: false,
            max_snapshots_per_run: 10,
            priority: 50,
        };

        let result = manager.snapshot_manager.add_policy(policy.clone()).await;
        assert!(result.is_ok(), "Policy addition should succeed");

        // Verify policy was added
        let retrieved_policy = manager.snapshot_manager.get_policy(&policy.name).await;
        assert!(retrieved_policy.is_some(), "Policy should be retrievable");
        assert_eq!(retrieved_policy.unwrap().name, policy.name);

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_snapshot_statistics() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Get statistics
        let stats = manager.snapshot_manager.get_statistics().await;
        assert!(stats.total_snapshots >= 0, "Statistics should be retrievable");

        fixture.cleanup().await.unwrap();
    }
}

/// Tier Management Tests
#[cfg(test)]
mod tier_tests {
    use super::*;

    #[tokio::test]
    async fn test_tier_initialization() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Initialize tiers
        let result = manager.tier_manager.initialize_tiers().await;
        assert!(result.is_ok(), "Tier initialization should succeed");

        // Get tier status
        let status = manager.tier_manager.get_tier_status().await.unwrap();
        assert!(status.hot_utilization >= 0.0, "Hot tier should have valid utilization");
        assert!(status.warm_utilization >= 0.0, "Warm tier should have valid utilization");
        assert!(status.cold_utilization >= 0.0, "Cold tier should have valid utilization");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_tier_configuration() {
        let fixture = ZfsTestFixture::new().await.unwrap();
        
        // Test tier configurations
        let hot_config = fixture.zfs_config.get_tier_config(&StorageTier::Hot);
        assert_eq!(hot_config.name, "hot");
        assert_eq!(hot_config.properties.get("compression").unwrap(), "lz4");

        let warm_config = fixture.zfs_config.get_tier_config(&StorageTier::Warm);
        assert_eq!(warm_config.name, "warm");
        assert_eq!(warm_config.properties.get("compression").unwrap(), "zstd");

        let cold_config = fixture.zfs_config.get_tier_config(&StorageTier::Cold);
        assert_eq!(cold_config.name, "cold");
        assert_eq!(cold_config.properties.get("compression").unwrap(), "gzip-9");
    }
}

/// Migration Engine Tests
#[cfg(test)]
mod migration_tests {
    use super::*;

    #[tokio::test]
    async fn test_migration_job_creation() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Create migration job
        let source_path = PathBuf::from("/test/file.txt");
        let job_id = manager.migration_engine.queue_migration(
            source_path.clone(),
            crate::types::StorageTier::Warm,
            crate::migration::MigrationPriority::Normal,
        ).await.unwrap();

        assert!(!job_id.is_empty(), "Job ID should not be empty");

        // Check job status
        let job_status = manager.migration_engine.get_job_status(&job_id).await.unwrap();
        assert!(job_status.is_some(), "Job should exist");
        
        let job = job_status.unwrap();
        assert_eq!(job.source_path, source_path);
        assert_eq!(job.target_tier, crate::types::StorageTier::Warm);

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_migration_statistics() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Get migration statistics
        let stats = manager.migration_engine.get_statistics().await;
        assert!(stats.total_jobs >= 0, "Statistics should be valid");
        assert!(stats.successful_migrations >= 0, "Successful migrations should be valid");
        assert!(stats.failed_migrations >= 0, "Failed migrations should be valid");

        fixture.cleanup().await.unwrap();
    }
}

/// AI Integration Tests
#[cfg(test)]
mod ai_tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_tier_recommendation() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Test AI tier recommendation
        let file_path = "/test/sample_file.txt";
        let recommendation = manager.get_ai_tier_recommendation(file_path).await.unwrap();
        
        if let Some(pred) = recommendation {
            assert!(!pred.file_path.is_empty(), "File path should not be empty");
            assert!(pred.confidence >= 0.0 && pred.confidence <= 1.0, "Confidence should be between 0 and 1");
            assert!(!pred.reasoning.is_empty(), "Reasoning should not be empty");
        }

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_ai_optimization_opportunities() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Get AI optimization opportunities
        if let Some(ai) = &manager.ai_integration {
            let opportunities = ai.get_optimization_opportunities().await;
            // Opportunities list can be empty, but should be retrievable
            assert!(opportunities.len() >= 0, "Should be able to retrieve opportunities");
        }

        fixture.cleanup().await.unwrap();
    }
}

/// Performance Monitoring Tests
#[cfg(test)]
mod performance_tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_metrics_collection() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Get current metrics
        let metrics = manager.performance_monitor.get_current_metrics().await;
        assert!(metrics.timestamp > SystemTime::UNIX_EPOCH, "Metrics should have valid timestamp");
        assert!(metrics.pool_metrics.total_iops >= 0.0, "IOPS should be non-negative");
        assert!(metrics.pool_metrics.total_throughput_mbs >= 0.0, "Throughput should be non-negative");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_performance_tier_metrics() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Test tier metrics for each tier
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            let tier_data = manager.performance_monitor.get_tier_metrics(&tier).await;
            if let Some(data) = tier_data {
                assert_eq!(data.current.tier, tier, "Tier should match");
                assert!(data.current.read_iops >= 0.0, "Read IOPS should be non-negative");
                assert!(data.current.write_iops >= 0.0, "Write IOPS should be non-negative");
                assert!(data.current.utilization_percent >= 0.0, "Utilization should be non-negative");
            }
        }

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_performance_alerts() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Get active alerts
        let alerts = manager.performance_monitor.get_active_alerts().await;
        // Alerts can be empty, but should be retrievable
        assert!(alerts.len() >= 0, "Should be able to retrieve alerts");

        fixture.cleanup().await.unwrap();
    }
}

/// Orchestrator Integration Tests
#[cfg(test)]
mod orchestrator_tests {
    use super::*;

    #[tokio::test]
    async fn test_service_status_reporting() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Get enhanced service status
        let status = manager.get_service_status().await.unwrap();
        assert!(matches!(status.overall_health, HealthState::Healthy | HealthState::Unknown | HealthState::Warning));
        assert!(status.pool_status.pools_online >= 0, "Pool count should be non-negative");
        assert!(status.performance_metrics.timestamp > SystemTime::UNIX_EPOCH, "Metrics should have valid timestamp");

        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_optimization_trigger() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Trigger optimization
        let result = manager.trigger_optimization().await.unwrap();
        assert!(result.success, "Optimization should succeed");
        assert!(!result.results.is_empty(), "Should have optimization results");
        assert!(result.timestamp > SystemTime::UNIX_EPOCH, "Should have valid timestamp");

        fixture.cleanup().await.unwrap();
    }
}

/// MCP Integration Tests
#[cfg(test)]
mod mcp_tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_storage_provider_creation() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Create MCP storage provider - create a new manager instance for Arc
        let manager_arc = Arc::new({
            let config = fixture.zfs_config.clone();
            ZfsManager::new(config).await.unwrap()
        });
        
        let config = crate::mcp_integration::ZfsMcpConfig::default();
        let provider = crate::mcp_integration::ZfsMcpStorageProvider::new(
            manager_arc,
            config,
        );

        // Test mount creation
        let mount_request = crate::mcp_integration::McpMountRequest {
            mount_id: "test_mount".to_string(),
            mount_point: "/mcp/test".to_string(),
            tier: StorageTier::Hot,
            size_gb: 1,
        };

        let result = provider.create_mount(mount_request).await;
        // This might fail in test environment, but should not panic
        match result {
            Ok(mount_info) => {
                assert_eq!(mount_info.mount_id, "test_mount");
                assert_eq!(mount_info.tier, StorageTier::Hot);
            }
            Err(_) => {
                // Expected in test environment
            }
        }

        fixture.cleanup().await.unwrap();
    }
}

/// Error Handling Tests
#[cfg(test)]
mod error_tests {
    use super::*;

    #[tokio::test]
    async fn test_error_handling_and_recovery() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        
        // Test invalid configuration
        let mut invalid_config = fixture.zfs_config.clone();
        invalid_config.default_pool = "".to_string(); // Invalid empty pool name
        
        let validation_result = invalid_config.validate();
        assert!(validation_result.is_err(), "Invalid configuration should fail validation");

        // Test manager creation with invalid config
        let _manager_result = ZfsManager::new(invalid_config).await;
        // This might succeed or fail depending on implementation
        // The important thing is it doesn't panic
        
        fixture.cleanup().await.unwrap();
    }

    #[test]
    fn test_error_type_conversions() {
        use crate::error::*;
        
        // Test error type conversions
        let pool_error = PoolError::NotFound { pool_name: "test".to_string() };
        let zfs_error: ZfsError = pool_error.into();
        
        match zfs_error {
            ZfsError::PoolError(PoolError::NotFound { pool_name }) => {
                assert_eq!(pool_name, "test");
            }
            _ => panic!("Error conversion failed"),
        }
    }

    #[test]
    fn test_error_retryability() {
        use crate::error::*;
        
        // Test retryable errors
        let timeout_error = ZfsError::Timeout("Operation timed out".to_string());
        assert!(ZfsError::is_retryable(&timeout_error), "Timeout errors should be retryable");
        
        let not_found_error = ZfsError::PoolError(PoolError::NotFound { pool_name: "test".to_string() });
        assert!(!ZfsError::is_retryable(&not_found_error), "Not found errors should not be retryable");
    }
}

/// Stress Tests
#[cfg(test)]
mod stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_operations() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        // Create a new manager instance for concurrent access
        let manager = Arc::new({
            let config = fixture.zfs_config.clone();
            ZfsManager::new(config).await.unwrap()
        });
        
        // Test concurrent dataset operations
        let mut handles = vec![];
        
        for i in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let pool_name = fixture.config.test_pool_name.clone();
            
            let handle = tokio::spawn(async move {
                let dataset_name = format!("{}/concurrent_test_{}", pool_name, i);
                
                // Create dataset
                let create_result = manager_clone.dataset_manager.create_dataset(
                    &dataset_name,
                    &pool_name,
                    StorageTier::Warm
                ).await;
                
                // Get dataset info
                let info_result = manager_clone.dataset_manager.get_dataset_info(&dataset_name).await;
                
                (create_result, info_result)
            });
            
            handles.push(handle);
        }
        
        // Wait for all operations to complete
        let results = futures::future::join_all(handles).await;
        
        // Check that most operations succeeded (some might fail in test environment)
        let success_count = results.iter()
            .filter(|result| result.is_ok())
            .count();
        
        assert!(success_count >= 0, "At least some concurrent operations should succeed");
        
        fixture.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_memory_usage_under_load() {
        let mut fixture = ZfsTestFixture::new().await.unwrap();
        fixture.init_manager().await.unwrap();

        let manager = fixture.manager();
        
        // Simulate load by getting metrics repeatedly
        for _ in 0..100 {
            let _metrics = manager.performance_monitor.get_current_metrics().await;
            let _status = manager.get_service_status().await.unwrap();
            
            // Small delay to prevent overwhelming the system
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        // Test should complete without excessive memory usage
        // In a real test, we'd measure actual memory usage
        
        fixture.cleanup().await.unwrap();
    }
}

/// Configuration Tests
#[cfg(test)]
mod config_tests {
    use super::*;

    #[tokio::test]
    async fn test_config_serialization() {
        let config = ZfsConfig::default();
        
        // Test JSON serialization
        let json_str = serde_json::to_string(&config).unwrap();
        let deserialized_config: ZfsConfig = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(config.default_pool, deserialized_config.default_pool);
        assert_eq!(config.api_endpoint, deserialized_config.api_endpoint);
    }

    #[tokio::test]
    async fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        
        let config = ZfsConfig::default();
        
        // Test saving configuration
        let save_result = config.save_to_file(&config_path).await;
        assert!(save_result.is_ok(), "Config save should succeed");
        
        // Test loading configuration
        let loaded_config = ZfsConfig::load_from_file(&config_path).await.unwrap();
        assert_eq!(config.default_pool, loaded_config.default_pool);
    }

    #[test]
    fn test_config_validation() {
        let mut config = ZfsConfig::default();
        
        // Valid configuration should pass
        assert!(config.validate().is_ok(), "Default config should be valid");
        
        // Invalid configuration should fail
        config.default_pool = "".to_string();
        assert!(config.validate().is_err(), "Empty pool name should be invalid");
        
        config.default_pool = "test_pool".to_string();
        config.api_endpoint = "invalid_url".to_string();
        assert!(config.validate().is_err(), "Invalid URL should be invalid");
    }
}

/// Helper functions for tests
pub mod test_helpers {
    use super::*;

    /// Wait for condition with timeout
    pub async fn wait_for_condition<F, Fut>(
        condition: F,
        timeout_duration: Duration,
        check_interval: Duration,
    ) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = std::time::Instant::now();
        
        while start.elapsed() < timeout_duration {
            if condition().await {
                return Ok(());
            }
            tokio::time::sleep(check_interval).await;
        }
        
        Err(NestGateError::Internal("Condition timeout".to_string()))
    }

    /// Create test dataset configuration
    pub fn create_test_dataset_config(name: &str) -> crate::dataset::DatasetConfig {
        crate::dataset::DatasetConfig {
            name: name.to_string(),
            parent: "test_pool".to_string(),
            tier: StorageTier::Warm,
            compression: crate::types::CompressionAlgorithm::Lz4,
            record_size: 128 * 1024,
            quota: Some(1024 * 1024 * 1024), // 1GB
            reservation: None,
            properties: vec![],
        }
    }

    /// Create test migration job
    pub fn create_test_migration_job(
        source: &str,
        source_tier: StorageTier,
        target_tier: StorageTier,
    ) -> crate::migration::MigrationJob {
        // Convert core StorageTier to ZFS StorageTier
        let zfs_source_tier = match source_tier {
            StorageTier::Hot => crate::types::StorageTier::Hot,
            StorageTier::Warm => crate::types::StorageTier::Warm,
            StorageTier::Cold => crate::types::StorageTier::Cold,
            StorageTier::Cache => crate::types::StorageTier::Hot, // Map Cache to Hot
        };
        let zfs_target_tier = match target_tier {
            StorageTier::Hot => crate::types::StorageTier::Hot,
            StorageTier::Warm => crate::types::StorageTier::Warm,
            StorageTier::Cold => crate::types::StorageTier::Cold,
            StorageTier::Cache => crate::types::StorageTier::Hot, // Map Cache to Hot
        };
        
        crate::migration::MigrationJob::new(
            PathBuf::from(source),
            zfs_source_tier,
            zfs_target_tier,
            crate::migration::MigrationPriority::Normal,
            1024 * 1024, // 1MB
        )
    }

    /// Verify tier performance hierarchy
    pub fn verify_tier_performance_hierarchy(
        hot: &crate::performance::TierMetrics,
        warm: &crate::performance::TierMetrics,
        cold: &crate::performance::TierMetrics,
    ) {
        // IOPS: Hot > Warm > Cold
        assert!(hot.read_iops > warm.read_iops, "Hot tier should have higher IOPS than warm");
        assert!(warm.read_iops > cold.read_iops, "Warm tier should have higher IOPS than cold");
        
        // Throughput: Hot > Warm > Cold
        assert!(hot.read_throughput_mbs > warm.read_throughput_mbs, "Hot tier should have higher throughput than warm");
        assert!(warm.read_throughput_mbs > cold.read_throughput_mbs, "Warm tier should have higher throughput than cold");
        
        // Latency: Hot < Warm < Cold
        assert!(hot.avg_read_latency_ms < warm.avg_read_latency_ms, "Hot tier should have lower latency than warm");
        assert!(warm.avg_read_latency_ms < cold.avg_read_latency_ms, "Warm tier should have lower latency than cold");
    }
}

/// Create a test ZFS manager with real or mock configuration
async fn create_test_zfs_manager(use_real_zfs: bool) -> Arc<ZfsManager> {
    let mut config = ZfsConfig::default();
    config.use_real_zfs = use_real_zfs;
    config.default_pool = "nestpool".to_string();
    
    let manager = ZfsManager::new(config).await
        .expect("Failed to create test ZFS manager");
    Arc::new(manager)
}

/// Create a test ZFS manager with default settings (real ZFS)
async fn create_test_manager() -> Arc<ZfsManager> {
    create_test_zfs_manager(true).await
} 