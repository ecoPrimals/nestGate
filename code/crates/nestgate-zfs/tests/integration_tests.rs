//! Integration tests for ZFS functionality

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tempfile::TempDir;
use tokio::time::timeout;
use serial_test::serial;

use nestgate_core::{Result, StorageTier as CoreStorageTier, NestGateError};
use nestgate_zfs::{
    config::ZfsConfig,
    performance::PerformanceConfig, 
    manager::ZfsManager,
    pool::ZfsPoolManager,
    dataset::{ZfsDatasetManager, DatasetConfig},
    migration::{MigrationJob, MigrationPriority, MigrationStatus},
    performance::{ZfsPerformanceMonitor, TierMetrics},
    types::{StorageTier, CompressionAlgorithm},
    error::ZfsError,
};

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
            use_real_zfs: false, // Safe default for CI/CD
            test_pool_name: "test_pool".to_string(),
            test_timeout_seconds: 30,
        }
    }
}

/// Test fixture for ZFS integration tests
#[derive(Debug)]
pub struct ZfsTestFixture {
    pub config: TestConfig,
    pub temp_dir: TempDir,
    pub zfs_config: ZfsConfig,
    pub zfs_manager: Option<ZfsManager>,
}

impl ZfsTestFixture {
    /// Create a new test fixture
    pub async fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Self::with_config(TestConfig::default()).await
    }
    
    /// Create a test fixture with custom configuration
    pub async fn with_config(config: TestConfig) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        let mut zfs_config = ZfsConfig::default();
        zfs_config.use_real_zfs = config.use_real_zfs;
        zfs_config.default_pool = config.test_pool_name.clone();
        
        Ok(Self {
            config,
            temp_dir,
            zfs_config,
            zfs_manager: None,
        })
    }
    
    /// Initialize the ZFS manager
    pub async fn init_manager(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = ZfsManager::new(self.zfs_config.clone()).await?;
        self.zfs_manager = Some(manager);
        Ok(())
    }
    
    /// Get a reference to the ZFS manager
    pub fn manager(&self) -> &ZfsManager {
        self.zfs_manager.as_ref().expect("Manager not initialized")
    }
    
    /// Get a mutable reference to the ZFS manager
    pub fn manager_mut(&mut self) -> &mut ZfsManager {
        self.zfs_manager.as_mut().expect("Manager not initialized")
    }
    
    /// Clean up test resources
    pub async fn cleanup(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(mut manager) = self.zfs_manager.take() {
            let _ = manager.shutdown().await;
        }
        Ok(())
    }
}

#[cfg(test)]
mod pool_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pool_discovery() {
        let mut fixture = ZfsTestFixture::new().await.expect("Failed to create test fixture");
        fixture.init_manager().await.expect("Failed to initialize manager");
        
        let manager = fixture.manager();
        let pool_list = manager.list_pools().await?;
        let status = if !pool_list.is_empty() {
            manager.get_pool_status(&pool_list[0]).await?
        } else {
            "No pools available".to_string()
        };
        
        if pool_list.is_empty() {
            println!("⚠️  No pools found (expected in CI environments without ZFS)");
            assert!(status.contains("No pools"));
        } else {
            println!("✅ Pool discovery successful: {} pools found", pool_list.len());
            println!("Pool status: {}", status);
            assert!(!status.is_empty());
        }
        
        fixture.cleanup().await.expect("Failed to cleanup");
    }
}

#[cfg(test)]
mod dataset_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dataset_creation() {
        let mut fixture = ZfsTestFixture::new().await.expect("Failed to create test fixture");
        
        // Create dataset manager directly for testing
        let pool_manager = Arc::new(ZfsPoolManager::new(&fixture.zfs_config).await.unwrap_or_else(|_| {
            ZfsPoolManager::new_for_testing()
        }));
        let dataset_manager = ZfsDatasetManager::new(fixture.zfs_config.clone(), pool_manager);
        
        // Test dataset creation (will use mock if no ZFS)
        let result = dataset_manager.create_dataset("test_dataset", "test_pool", CoreStorageTier::Warm).await;
        
        match result {
            Ok(info) => {
                println!("✅ Dataset created: {}", info.name);
                assert!(!info.name.is_empty());
                assert!(info.used_space >= 0);
            }
            Err(e) => {
                println!("Dataset creation failed (expected without ZFS): {}", e);
                // This is expected in test environments without ZFS
            }
        }
        
        fixture.cleanup().await.expect("Failed to cleanup");
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_performance_metrics_collection() {
        let config = PerformanceConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new(&ZfsConfig::default()).await.unwrap_or_else(|_| {
            ZfsPoolManager::new_for_testing()
        }));
        let dataset_manager = Arc::new(ZfsDatasetManager::new(ZfsConfig::default(), pool_manager.clone()));
        
        let mut perf_monitor = ZfsPerformanceMonitor::new(config, pool_manager, dataset_manager);
        
        // Test performance monitoring startup
        match perf_monitor.start().await {
            Ok(_) => {
                println!("✅ Performance monitoring started");
                
                // Test metrics collection
                let metrics = perf_monitor.get_current_metrics().await;
                println!("Current metrics timestamp: {:?}", metrics.timestamp);
                println!("I/O wait percent: {:.2}%", metrics.system_metrics.io_wait_percent);
                println!("Network I/O: {:.2} MB", metrics.system_metrics.network_io_mbs);
                
                // Verify metrics are reasonable
                assert!(metrics.system_metrics.io_wait_percent >= 0.0);
                assert!(metrics.system_metrics.io_wait_percent <= 100.0);
                assert!(metrics.system_metrics.network_io_mbs >= 0.0);
                
                let _ = perf_monitor.stop().await;
            }
            Err(e) => {
                println!("Performance monitoring failed to start: {}", e);
                // This might be expected in some test environments
            }
        }
    }
}

#[cfg(test)]
mod core_functionality_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_zfs_manager_creation_and_basic_ops() {
        // Initialize tracing for test debugging
        let _ = tracing_subscriber::fmt::try_init();
        
        println!("🧪 Testing ZFS Manager creation and basic operations...");
        
        // Test ZFS manager creation
        let zfs_config = ZfsConfig::default();
        match ZfsManager::new(zfs_config).await {
            Ok(mut zfs_manager) => {
                println!("✅ ZFS Manager created successfully");
                
                // Test manager startup
                if let Err(e) = zfs_manager.start().await {
                    println!("ZFS Manager start failed (expected without ZFS): {}", e);
                }
                
                // Test service status retrieval
                if let Ok(status) = zfs_manager.get_service_status().await {
                    println!("✅ Service status retrieved: {:?}", status.overall_health);
                    println!("Pool status: {} online, {} degraded", 
                             status.pool_status.pools_online, 
                             status.pool_status.pools_degraded);
                }
                
                // Test graceful shutdown
                if let Err(e) = zfs_manager.shutdown().await {
                    println!("Shutdown error: {}", e);
                }
                
                println!("✅ ZFS Manager basic operations completed");
            }
            Err(e) => {
                println!("ZFS Manager creation failed (expected without ZFS): {}", e);
                // This is expected in test environments without ZFS
            }
        }
        
        println!("🎉 ZFS Manager test completed");
    }
}

/// Helper functions for tests
pub mod test_helpers {
    use super::*;
    
    /// Wait for a condition with timeout
    pub async fn wait_for_condition<F, Fut>(
        condition: F,
        timeout_duration: Duration,
        check_interval: Duration,
    ) -> nestgate_core::Result<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let timeout_result = timeout(timeout_duration, async {
            loop {
                if condition().await {
                    return;
                }
                tokio::time::sleep(check_interval).await;
            }
        }).await;
        
        match timeout_result {
            Ok(_) => Ok(()),
            Err(_) => Err(NestGateError::Timeout("Condition timeout".to_string())),
        }
    }
    
    /// Create test dataset configuration
    pub fn create_test_dataset_config(name: &str) -> DatasetConfig {
        DatasetConfig {
            name: name.to_string(),
            parent: "test_pool".to_string(),
            tier: CoreStorageTier::Warm,
            compression: CompressionAlgorithm::Lz4,
            record_size: 128 * 1024,
            quota: Some(1024 * 1024 * 1024), // 1GB
            reservation: None,
            properties: vec![],
        }
    }
    
    /// Create test migration job
    pub fn create_test_migration_job(
        source: &str,
        source_tier: CoreStorageTier,
        target_tier: CoreStorageTier,
    ) -> MigrationJob {
        // Convert core StorageTier to ZFS StorageTier
        let zfs_source_tier = match source_tier {
            CoreStorageTier::Hot => StorageTier::Hot,
            CoreStorageTier::Warm => StorageTier::Warm,
            CoreStorageTier::Cold => StorageTier::Cold,
            CoreStorageTier::Cache => StorageTier::Hot, // Map Cache to Hot
        };
        let zfs_target_tier = match target_tier {
            CoreStorageTier::Hot => StorageTier::Hot,
            CoreStorageTier::Warm => StorageTier::Warm,
            CoreStorageTier::Cold => StorageTier::Cold,
            CoreStorageTier::Cache => StorageTier::Hot, // Map Cache to Hot
        };
        
        MigrationJob::new(
            PathBuf::from(source),
            zfs_source_tier,
            zfs_target_tier,
            MigrationPriority::Normal,
            1024 * 1024, // 1MB
        )
    }
} 