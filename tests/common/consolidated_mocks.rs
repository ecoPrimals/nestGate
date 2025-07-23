//! Consolidated Mock Infrastructure
//!
//! This module provides a centralized mock system that consolidates all mock implementations
//! across the test infrastructure to eliminate duplication and provide consistent behavior.

use async_trait::async_trait;
use serde::{Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{ SystemTime};
use tokio::sync::RwLock;
// Removed unused tracing import
use uuid::Uuid;

use nestgate_core::{NestGateError, Result, StorageTier, UniversalService};
use nestgate_zfs::{ZfsConfig, ZfsManager};

/// Consolidated mock registry for all test mocks
pub struct MockRegistry {
    services: Arc<RwLock<HashMap<String, Arc<dyn MockService>>>>,
    zfs_managers: Arc<RwLock<HashMap<String, Arc<MockZfsManager>>>>,
    performance_monitors: Arc<RwLock<HashMap<String, Arc<MockPerformanceMonitor>>>>,
    config: MockRegistryConfig,
}

/// Configuration for the mock registry
#[derive(Debug, Clone)]
pub struct MockRegistryConfig {
    pub enable_realistic_delays: bool,
    pub default_error_rate: f64,
    pub max_concurrent_operations: usize,
    pub cleanup_interval: Duration,
}

impl Default for MockRegistryConfig {
    fn default() -> Self {
        Self {
            enable_realistic_delays: true,
            default_error_rate: 0.01, // 1% error rate
            max_concurrent_operations: 100,
            cleanup_interval: Duration::from_secs(60),
        }
    }
}

/// Unified mock service trait for all service types
#[async_trait]
pub trait MockService: Send + Sync {
    /// Service type identifier
    fn service_type(&self) -> &str;
    
    /// Service instance ID
    fn instance_id(&self) -> &str;
    
    /// Start the mock service
    async fn start(&self) -> Result<()>;
    
    /// Stop the mock service
    async fn stop(&self) -> Result<()>;
    
    /// Get current health status
    async fn health_check(&self) -> Result<MockHealth>;
    
    /// Configure error injection
    async fn configure_errors(&self, error_rate: f64, error_types: Vec<String>);
    
    /// Get service metrics
    async fn get_metrics(&self) -> Result<MockMetrics>;
    
    /// Reset service state
    async fn reset(&self) -> Result<()>;
}

/// Unified health status for all mock services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockHealth {
    pub status: String,
    pub uptime_seconds: u64,
    pub requests_handled: u64,
    pub errors_count: u64,
    pub memory_usage_bytes: u64,
    pub last_health_check: SystemTime,
}

/// Unified metrics for all mock services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockMetrics {
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
    pub throughput_bytes_per_second: u64,
    pub concurrent_operations: usize,
}

/// Consolidated ZFS manager mock
pub struct MockZfsManager {
    instance_id: String,
    config: ZfsConfig,
    pools: Arc<RwLock<HashMap<String, MockPool>>>,
    datasets: Arc<RwLock<HashMap<String, MockDataset>>>,
    snapshots: Arc<RwLock<HashMap<String, MockSnapshot>>>,
    health: Arc<RwLock<MockHealth>>,
    metrics: Arc<RwLock<MockMetrics>>,
    error_injection: Arc<RwLock<MockErrorInjection>>,
}

/// Mock ZFS pool representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockPool {
    pub name: String,
    pub status: String,
    pub capacity_bytes: u64,
    pub used_bytes: u64,
    pub health: String,
    pub created_at: SystemTime,
}

/// Mock ZFS dataset representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockDataset {
    pub name: String,
    pub pool: String,
    pub tier: StorageTier,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub compression: String,
    pub created_at: SystemTime,
}

/// Mock ZFS snapshot representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSnapshot {
    pub name: String,
    pub dataset: String,
    pub size_bytes: u64,
    pub created_at: SystemTime,
}

/// Mock error injection configuration
#[derive(Debug, Clone)]
pub struct MockErrorInjection {
    pub error_rate: f64,
    pub enabled_errors: Vec<String>,
    pub delay_range: (Duration, Duration),
}

impl Default for MockErrorInjection {
    fn default() -> Self {
        Self {
            error_rate: 0.0,
            enabled_errors: Vec::new(),
            delay_range: (Duration::from_millis(10), Duration::from_millis(100)),
        }
    }
}

/// Consolidated performance monitor mock
pub struct MockPerformanceMonitor {
    instance_id: String,
    metrics: Arc<RwLock<MockMetrics>>,
    system_metrics: Arc<RwLock<MockSystemMetrics>>,
    started: Arc<RwLock<bool>>,
}

/// Mock system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_io_mbs: f64,
    pub network_io_mbs: f64,
    pub io_wait_percent: f64,
    pub load_average: f64,
}

impl MockRegistry {
    /// Create a new mock registry
    pub fn new(config: MockRegistryConfig) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            zfs_managers: Arc::new(RwLock::new(HashMap::new())),
            performance_monitors: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a generic mock service
    pub async fn register_service(&self, service: Arc<dyn MockService>) -> Result<()> {
        let mut services = self.services.write().await;
        services.insert(service.instance_id().to_string(), service);
        Ok(())
    }

    /// Create and register a mock ZFS manager
    pub async fn create_zfs_manager(&self, instance_id: String, config: ZfsConfig) -> Result<Arc<MockZfsManager>> {
        let manager = Arc::new(MockZfsManager::new(instance_id.clone(), config));
        
        // Pre-populate with some mock data
        manager.populate_mock_data().await?;
        
        let mut managers = self.zfs_managers.write().await;
        managers.insert(instance_id, manager.clone());
        
        Ok(manager)
    }

    /// Create and register a mock performance monitor
    pub async fn create_performance_monitor(&self, instance_id: String) -> Result<Arc<MockPerformanceMonitor>> {
        let monitor = Arc::new(MockPerformanceMonitor::new(instance_id.clone()));
        
        let mut monitors = self.performance_monitors.write().await;
        monitors.insert(instance_id, monitor.clone());
        
        Ok(monitor)
    }

    /// Get all registered services
    pub async fn list_services(&self) -> Vec<String> {
        let services = self.services.read().await;
        services.keys().cloned().collect()
    }

    /// Cleanup all mock resources
    pub async fn cleanup_all(&self) -> Result<()> {
        info!("🧹 Cleaning up all mock resources...");
        
        // Stop all services
        let services = self.services.read().await;
        for service in services.values() {
            if let Err(e) = service.stop().await {
                warn!("Failed to stop service {}: {}", service.instance_id(), e);
            }
        }
        
        // Clear all registries
        drop(services);
        self.services.write().await.clear();
        self.zfs_managers.write().await.clear();
        self.performance_monitors.write().await.clear();
        
        info!("✅ Mock cleanup completed");
        Ok(())
    }

    /// Configure global error injection
    pub async fn configure_global_errors(&self, error_rate: f64, error_types: Vec<String>) -> Result<()> {
        let services = self.services.read().await;
        for service in services.values() {
            service.configure_errors(error_rate, error_types.clone()).await;
        }
        
        let zfs_managers = self.zfs_managers.read().await;
        for manager in zfs_managers.values() {
            manager.configure_error_injection(error_rate, error_types.clone()).await;
        }
        
        Ok(())
    }
}

impl MockZfsManager {
    /// Create a new mock ZFS manager
    pub fn new(instance_id: String, config: ZfsConfig) -> Self {
        Self {
            instance_id,
            config,
            pools: Arc::new(RwLock::new(HashMap::new())),
            datasets: Arc::new(RwLock::new(HashMap::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            health: Arc::new(RwLock::new(MockHealth {
                status: "healthy".to_string(),
                uptime_seconds: 0,
                requests_handled: 0,
                errors_count: 0,
                memory_usage_bytes: 1024 * 1024,
                last_health_check: SystemTime::now(),
            })),
            metrics: Arc::new(RwLock::new(MockMetrics {
                operations_per_second: 100.0,
                average_latency_ms: 50.0,
                error_rate: 0.0,
                throughput_bytes_per_second: 1024 * 1024,
                concurrent_operations: 0,
            })),
            error_injection: Arc::new(RwLock::new(MockErrorInjection::default())),
        }
    }

    /// Populate with realistic mock data
    pub async fn populate_mock_data(&self) -> Result<()> {
        // Create mock pools
        let mut pools = self.pools.write().await;
        pools.insert("nestpool".to_string(), MockPool {
            name: "nestpool".to_string(),
            status: "ONLINE".to_string(),
            capacity_bytes: 1024 * 1024 * 1024 * 1024, // 1TB
            used_bytes: 512 * 1024 * 1024 * 1024, // 512GB
            health: "HEALTHY".to_string(),
            created_at: SystemTime::now(),
        });
        
        // Create mock datasets
        let mut datasets = self.datasets.write().await;
        datasets.insert("nestpool/workspace".to_string(), MockDataset {
            name: "workspace".to_string(),
            pool: "nestpool".to_string(),
            tier: StorageTier::Hot,
            size_bytes: 100 * 1024 * 1024 * 1024, // 100GB
            used_bytes: 50 * 1024 * 1024 * 1024, // 50GB
            compression: "lz4".to_string(),
            created_at: SystemTime::now(),
        });
        
        // Create mock snapshots
        let mut snapshots = self.snapshots.write().await;
        snapshots.insert("nestpool/workspace@backup".to_string(), MockSnapshot {
            name: "backup".to_string(),
            dataset: "nestpool/workspace".to_string(),
            size_bytes: 10 * 1024 * 1024 * 1024, // 10GB
            created_at: SystemTime::now(),
        });
        
        Ok(())
    }

    /// Configure error injection
    pub async fn configure_error_injection(&self, error_rate: f64, error_types: Vec<String>) {
        let mut error_injection = self.error_injection.write().await;
        error_injection.error_rate = error_rate;
        error_injection.enabled_errors = error_types;
    }

    /// Get mock datasets (for compatibility)
    pub async fn get_mock_datasets(&self) -> Result<Vec<MockDataset>> {
        let datasets = self.datasets.read().await;
        Ok(datasets.values().cloned().collect())
    }

    /// List mock pools
    pub async fn list_pools(&self) -> Result<Vec<MockPool>> {
        let pools = self.pools.read().await;
        Ok(pools.values().cloned().collect())
    }

    /// Create mock dataset
    pub async fn create_dataset(&self, pool: &str, name: &str, tier: StorageTier) -> Result<MockDataset> {
        let dataset = MockDataset {
            name: name.to_string(),
            pool: pool.to_string(),
            tier,
            size_bytes: 10 * 1024 * 1024 * 1024, // 10GB default
            used_bytes: 0,
            compression: "lz4".to_string(),
            created_at: SystemTime::now(),
        };
        
                 let mut datasets = self.datasets.write().await;
         datasets.insert(format!("{pool}/{name}"), dataset.clone());
        
        Ok(dataset)
    }

    /// Check if ZFS is available (always returns false for mock)
    pub async fn is_zfs_available(&self) -> bool {
        false
    }

    /// Get status (mock implementation)
    pub async fn get_status(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "status": "healthy",
            "pools": self.pools.read().await.len(),
            "datasets": self.datasets.read().await.len(),
            "snapshots": self.snapshots.read().await.len(),
        }))
    }
}

impl MockPerformanceMonitor {
    /// Create a new mock performance monitor
    pub fn new(instance_id: String) -> Self {
        Self {
            instance_id,
            metrics: Arc::new(RwLock::new(MockMetrics {
                operations_per_second: 150.0,
                average_latency_ms: 25.0,
                error_rate: 0.0,
                throughput_bytes_per_second: 2 * 1024 * 1024,
                concurrent_operations: 0,
            })),
            system_metrics: Arc::new(RwLock::new(MockSystemMetrics {
                cpu_usage_percent: 35.0,
                memory_usage_percent: 45.0,
                disk_io_mbs: 120.0,
                network_io_mbs: 80.0,
                io_wait_percent: 5.0,
                load_average: 1.2,
            })),
            started: Arc::new(RwLock::new(false)),
        }
    }

    /// Start monitoring
    pub async fn start(&self) -> Result<()> {
        *self.started.write().await = true;
        Ok(())
    }

    /// Stop monitoring
    pub async fn stop(&self) -> Result<()> {
        *self.started.write().await = false;
        Ok(())
    }

    /// Get current metrics
    pub async fn get_current_metrics(&self) -> MockMetrics {
        self.metrics.read().await.clone()
    }

    /// Get system metrics
    pub async fn get_system_metrics(&self) -> MockSystemMetrics {
        self.system_metrics.read().await.clone()
    }
}

/// Helper functions for creating common mock configurations
pub mod helpers {
    use super::*;

    /// Create a mock registry with test defaults
    pub fn create_test_registry() -> MockRegistry {
        MockRegistry::new(MockRegistryConfig {
            enable_realistic_delays: false, // Faster tests
            default_error_rate: 0.0,
            max_concurrent_operations: 50,
            cleanup_interval: Duration::from_secs(10),
        })
    }

    /// Create a mock ZFS manager with test data
    pub async fn create_test_zfs_manager() -> Result<Arc<MockZfsManager>> {
        let config = ZfsConfig::default();
        let manager = Arc::new(MockZfsManager::new("test-zfs".to_string(), config));
        manager.populate_mock_data().await?;
        Ok(manager)
    }

    /// Create a mock performance monitor
    pub fn create_test_performance_monitor() -> Arc<MockPerformanceMonitor> {
        Arc::new(MockPerformanceMonitor::new("test-perf".to_string()))
    }

    /// Wait for condition with timeout (test utility)
    pub async fn wait_for_condition<F, Fut>(
        condition: F,
        timeout: Duration,
        check_interval: Duration,
    ) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if condition().await {
                return Ok(());
            }
            tokio::time::sleep(check_interval).await;
        }

        Err(NestGateError::Internal {
            message: "Condition timeout".to_string(),
            location: Some(file!().to_string()),
            debug_info: None,
            is_bug: false,
        })
    }
}

/// Global mock registry instance for tests
static mut GLOBAL_MOCK_REGISTRY: Option<Arc<MockRegistry>> = None;
static REGISTRY_INIT: std::sync::Once = std::sync::Once::new();

/// Get or create the global mock registry
pub fn get_global_mock_registry() -> Arc<MockRegistry> {
    unsafe {
        REGISTRY_INIT.call_once(|| {
            GLOBAL_MOCK_REGISTRY = Some(Arc::new(MockRegistry::new(MockRegistryConfig::default())));
        });
        GLOBAL_MOCK_REGISTRY.as_ref().unwrap().clone()
    }
}

/// Cleanup global mock registry
pub async fn cleanup_global_mocks() -> Result<()> {
    let registry = get_global_mock_registry();
    registry.cleanup_all().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_mock_registry_basic_operations() {
        let registry = MockRegistry::new(MockRegistryConfig::default());
        
        // Test ZFS manager creation
        let zfs_manager = registry.create_zfs_manager("test-zfs".to_string(), ZfsConfig::default()).await.unwrap();
        assert_eq!(zfs_manager.instance_id, "test-zfs");
        
        // Test data population
        let datasets = zfs_manager.get_mock_datasets().await.unwrap();
        assert!(!datasets.is_empty());
        
        // Test performance monitor creation
        let perf_monitor = registry.create_performance_monitor("test-perf".to_string()).await.unwrap();
        assert_eq!(perf_monitor.instance_id, "test-perf");
        
        // Test cleanup
        registry.cleanup_all().await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_zfs_operations() {
        let manager = helpers::create_test_zfs_manager().await.unwrap();
        
        // Test dataset creation
        let dataset = manager.create_dataset("testpool", "testdataset", StorageTier::Hot).await.unwrap();
        assert_eq!(dataset.name, "testdataset");
        assert_eq!(dataset.pool, "testpool");
        assert_eq!(dataset.tier, StorageTier::Hot);
        
        // Test dataset listing
        let datasets = manager.get_mock_datasets().await.unwrap();
        assert!(datasets.len() >= 2); // Original + new dataset
        
        // Test pool listing
        let pools = manager.list_pools().await.unwrap();
        assert!(!pools.is_empty());
    }

    #[tokio::test]
    async fn test_mock_performance_monitor() {
        let monitor = helpers::create_test_performance_monitor();
        
        // Test start/stop
        monitor.start().await.unwrap();
        assert!(*monitor.started.read().await);
        
        // Test metrics
        let metrics = monitor.get_current_metrics().await;
        assert!(metrics.operations_per_second > 0.0);
        
        let system_metrics = monitor.get_system_metrics().await;
        assert!(system_metrics.cpu_usage_percent >= 0.0);
        
        monitor.stop().await.unwrap();
        assert!(!*monitor.started.read().await);
    }
} 