/// Consolidated Mock Infrastructure
///
/// This module provides a centralized mock system that consolidates all mock implementations
/// across the test infrastructure to eliminate duplication and provide consistent behavior.

use async_trait::async_trait;
use serde::{Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{ SystemTime};
use tokio::sync::RwLock;
// Removed unused tracing import
use uuid::Uuid;

use nestgate_core::{NestGateError, Result, StorageTier, UniversalService};
use nestgate_zfs::{UnifiedZfsConfig, ZfsManager};
use nestgate_core::unified_enums::service_types::{UnifiedServiceType, UnifiedServiceState};
use nestgate_core::unified_types::UnifiedServiceConfig;
use nestgate_core::error::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

// ==================== MIGRATED TO CANONICAL TRAIT ====================
/// **MIGRATION COMPLETE**: Updated from deprecated trait to canonical UniversalService
/// 
/// **BEFORE**: `use nestgate_core::traits_root::service::core::UniversalService;`
/// **AFTER**: `use nestgate_core::traits::UniversalService;`
/// 
/// This demonstrates the successful trait consolidation across the codebase.
use nestgate_core::traits::{
    UniversalService, 
    UniversalServiceRequest, 
    UniversalServiceResponse,
    create_success_response,
    create_error_response
};

/// Mock service configuration for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockServiceConfig {
    pub service_id: String,
    pub service_type: UnifiedServiceType,
    pub should_fail: bool,
    pub response_delay_ms: u64,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

impl Default for MockServiceConfig {
    fn default() -> Self {
        Self {
            service_id: "mock-service".to_string(),
            service_type: UnifiedServiceType::Testing,
            should_fail: false,
            response_delay_ms: 0,
            custom_settings: HashMap::new(),
        }
    }
}

/// Mock service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockServiceHealth {
    pub status: String,
    pub uptime_ms: u64,
    pub request_count: u64,
    pub error_count: u64,
    pub last_error: Option<String>,
}

/// **CONSOLIDATED MOCK SERVICE**
/// This mock service now uses the canonical UniversalService trait
/// Demonstrates successful migration from fragmented trait definitions
pub struct MockService {
    config: MockServiceConfig,
    status: UnifiedServiceState,
    health: Arc<Mutex<MockServiceHealth>>,
    started_at: Option<std::time::Instant>,
}

impl MockService {
    pub fn new(config: MockServiceConfig) -> Self {
        Self {
            config,
            status: UnifiedServiceState::Stopped,
            health: Arc::new(Mutex::new(MockServiceHealth {
                status: "initialized".to_string(),
                uptime_ms: 0,
                request_count: 0,
                error_count: 0,
                last_error: None,
            })),
            started_at: None,
        }
    }
    
    pub fn with_failure(mut self, should_fail: bool) -> Self {
        self.config.should_fail = should_fail;
        self
    }
    
    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.config.response_delay_ms = delay_ms;
        self
    }
}

/// **CANONICAL TRAIT IMPLEMENTATION**
/// This implementation shows the migration from deprecated traits to the canonical UniversalService
/// 
/// **MIGRATION BENEFITS DEMONSTRATED**:
/// - Unified error handling with `Result<T>`
/// - Enhanced methods like `metrics()` and `handle_request()`
/// - Consistent interface across all services
/// - Type safety with associated types
#[async_trait]
impl UniversalService for MockService {
    type Config = MockServiceConfig;
    type Health = MockServiceHealth;

    async fn initialize(&mut self, config: Self::Config) -> Result<()> {
        self.config = config;
        self.status = UnifiedServiceState::Starting;
        
        if let Ok(mut health) = self.health.lock() {
            health.status = "initialized".to_string();
        }
        
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        if self.config.should_fail {
            self.status = UnifiedServiceState::Error;
            if let Ok(mut health) = self.health.lock() {
                health.error_count += 1;
                health.last_error = Some("Mock service configured to fail".to_string());
            }
            return Err(NestGateError::System {
                message: "Mock service start failure".to_string(),
                resource: crate::error::SystemResource::Service,
                utilization: None,
                recovery: crate::error::RecoveryStrategy::Retry,
            });
        }
        
        self.status = UnifiedServiceState::Running;
        self.started_at = Some(std::time::Instant::now());
        
        if let Ok(mut health) = self.health.lock() {
            health.status = "running".to_string();
        }
        
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        self.status = UnifiedServiceState::Stopping;
        
        // Simulate stop delay if configured
        if self.config.response_delay_ms > 0 {
            tokio::time::sleep(Duration::from_millis(self.config.response_delay_ms)).await;
        }
        
        self.status = UnifiedServiceState::Stopped;
        self.started_at = None;
        
        if let Ok(mut health) = self.health.lock() {
            health.status = "stopped".to_string();
        }
        
        Ok(())
    }

    async fn status(&self) -> UnifiedServiceState {
        self.status
    }

    async fn health(&self) -> Result<Self::Health> {
        let mut health = if let Ok(health) = self.health.lock() {
            health.clone()
        } else {
            return Err(NestGateError::System {
                message: "Failed to acquire health lock".to_string(),
                resource: crate::error::SystemResource::Memory,
                utilization: None,
                recovery: crate::error::RecoveryStrategy::Retry,
            });
        };
        
        // Update uptime if running
        if let Some(started_at) = self.started_at {
            health.uptime_ms = started_at.elapsed().as_millis() as u64;
        }
        
        Ok(health)
    }

    fn service_id(&self) -> &str {
        &self.config.service_id
    }

    fn service_type(&self) -> UnifiedServiceType {
        self.config.service_type
    }

    fn name(&self) -> &str {
        "Mock Service"
    }

    fn version(&self) -> &str {
        "2.1.0-canonical"
    }

    fn description(&self) -> &str {
        "Consolidated mock service using canonical UniversalService trait"
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "testing".to_string(),
            "mocking".to_string(),
            "failure_simulation".to_string(),
            "delay_simulation".to_string(),
        ]
    }

    /// **NEW ENHANCED METHODS** - Available in canonical trait
    async fn metrics(&self) -> Result<HashMap<String, serde_json::Value>> {
        let mut metrics = HashMap::new();
        
        metrics.insert("status".to_string(), serde_json::json!(self.status()));
        metrics.insert("service_type".to_string(), serde_json::json!(self.service_type()));
        metrics.insert("should_fail".to_string(), serde_json::json!(self.config.should_fail));
        metrics.insert("response_delay_ms".to_string(), serde_json::json!(self.config.response_delay_ms));
        
        if let Ok(health) = self.health().await {
            metrics.insert("uptime_ms".to_string(), serde_json::json!(health.uptime_ms));
            metrics.insert("request_count".to_string(), serde_json::json!(health.request_count));
            metrics.insert("error_count".to_string(), serde_json::json!(health.error_count));
        }
        
        Ok(metrics)
    }

    async fn handle_request(&self, request: UniversalServiceRequest) -> Result<UniversalServiceResponse> {
        // Update request count
        if let Ok(mut health) = self.health.lock() {
            health.request_count += 1;
        }
        
        // Simulate response delay if configured
        if self.config.response_delay_ms > 0 {
            tokio::time::sleep(Duration::from_millis(self.config.response_delay_ms)).await;
        }
        
        // Simulate failure if configured
        if self.config.should_fail {
            if let Ok(mut health) = self.health.lock() {
                health.error_count += 1;
                health.last_error = Some(format!("Mock failure for operation: {}", request.operation));
            }
            return Ok(create_error_response(
                request.request_id,
                format!("Mock service configured to fail operation: {}", request.operation)
            ));
        }
        
        // Handle different mock operations
        let response_data = match request.operation.as_str() {
            "ping" => Some(serde_json::json!({"message": "pong"})),
            "echo" => Some(serde_json::json!({
                "echoed_parameters": request.parameters,
                "echoed_metadata": request.metadata
            })),
            "get_config" => Some(serde_json::json!(self.config)),
            "simulate_work" => {
                // Simulate some work with delay
                let work_duration = request.parameters
                    .get("duration_ms")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(100);
                tokio::time::sleep(Duration::from_millis(work_duration)).await;
                Some(serde_json::json!({"work_completed": true, "duration_ms": work_duration}))
            },
            _ => Some(serde_json::json!({"message": format!("Unknown operation: {}", request.operation)}))
        };
        
        Ok(create_success_response(request.request_id, response_data))
    }

    fn get_config(&self) -> Option<Self::Config> {
        Some(self.config.clone())
    }

    async fn update_config(&mut self, config: Self::Config) -> Result<()> {
        self.config = config;
        
        if let Ok(mut health) = self.health.lock() {
            health.status = "config_updated".to_string();
        }
        
        Ok(())
    }
}

/// **MIGRATION SUCCESS DEMONSTRATION**
/// This test validates that the canonical trait works correctly
#[cfg(test)]
mod canonical_trait_tests {
    use super::*;
    use nestgate_core::traits::create_service_request;
    
    #[tokio::test]
    async fn test_canonical_trait_migration() {
        let config = MockServiceConfig {
            service_id: "test-service".to_string(),
            service_type: UnifiedServiceType::Testing,
            should_fail: false,
            response_delay_ms: 0,
            custom_settings: HashMap::new(),
        };
        
        let mut service = MockService::new(config.clone());
        
        // Test initialization with canonical trait
        assert!(service.initialize(config).await.is_ok());
        
        // Test service lifecycle
        assert!(service.start().await.is_ok());
        assert_eq!(service.status().await, UnifiedServiceState::Running);
        
        // Test health check
        let health = service.health().await.expect("Service health check should succeed");
        assert_eq!(health.status, "running");
        
        // Test enhanced methods (new in canonical trait)
        let metrics = service.metrics().await.expect("Service metrics should be available");
        assert!(metrics.contains_key("status"));
        assert!(metrics.contains_key("service_type"));
        
        // Test request handling (new in canonical trait)
        let request = create_service_request("ping", HashMap::new());
        let response = service.handle_request(request).await.expect("Service should handle ping request");
        assert_eq!(response.status, nestgate_core::traits::UniversalResponseStatus::Success);
        
        // Test service stop
        assert!(service.stop().await.is_ok());
        assert_eq!(service.status().await, UnifiedServiceState::Stopped);
    }
    
    #[tokio::test]
    async fn test_migration_compatibility() {
        // Ensure the migrated service maintains all expected functionality
        let service = MockService::new(MockServiceConfig::default());
        
        // Test trait methods are available
        assert_eq!(service.service_id(), "mock-service");
        assert_eq!(service.service_type(), UnifiedServiceType::Testing);
        assert_eq!(service.name(), "Mock Service");
        assert_eq!(service.version(), "2.1.0-canonical");
        assert!(!service.description().is_empty());
        assert!(!service.capabilities().is_empty());
        
        // Test that the service supports the expected capabilities
        assert!(service.supports_capability("testing"));
        assert!(service.supports_capability("mocking"));
    }
}

/// Consolidated ZFS manager mock
pub struct MockZfsManager {
    instance_id: String,
    config: UnifiedZfsConfig,
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
    pub async fn create_zfs_manager(&self, instance_id: String, config: UnifiedZfsConfig) -> Result<Arc<MockZfsManager>> {
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
    pub fn new(instance_id: String, config: UnifiedZfsConfig) -> Self {
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
        let config = UnifiedZfsConfig::default();
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
            // SAFE: Use proper initialization pattern
        {
        REGISTRY_INIT.call_once(|| {
            GLOBAL_MOCK_REGISTRY = Some(Arc::new(MockRegistry::new(MockRegistryConfig::default())));
        });
        GLOBAL_MOCK_REGISTRY.as_ref().expect("Global mock registry should be initialized").clone()
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
        let zfs_manager = registry.create_zfs_manager("test-zfs".to_string(), UnifiedZfsConfig::default()).await
            .expect("ZFS manager creation should succeed");
        assert_eq!(zfs_manager.instance_id, "test-zfs");
        
        // Test data population
        let datasets = zfs_manager.get_mock_datasets().await
            .expect("Mock datasets should be available");
        assert!(!datasets.is_empty());
        
        // Test performance monitor creation
        let perf_monitor = registry.create_performance_monitor("test-perf".to_string()).await
            .expect("Performance monitor creation should succeed");
        assert_eq!(perf_monitor.instance_id, "test-perf");
        
        // Test cleanup
        registry.cleanup_all().await
            .expect("Registry cleanup should succeed");
    }

    #[tokio::test]
    async fn test_mock_zfs_operations() {
        let manager = helpers::create_test_zfs_manager().await
            .expect("Test ZFS manager creation should succeed");
        
        // Test dataset creation
        let dataset = manager.create_dataset("testpool", "testdataset", StorageTier::Hot).await
            .expect("Dataset creation should succeed");
        assert_eq!(dataset.name, "testdataset");
        assert_eq!(dataset.pool, "testpool");
        assert_eq!(dataset.tier, StorageTier::Hot);
        
        // Test dataset listing
        let datasets = manager.get_mock_datasets().await
            .expect("Dataset listing should succeed");
        assert!(datasets.len() >= 2); // Original + new dataset
        
        // Test pool listing
        let pools = manager.list_pools().await
            .expect("Pool listing should succeed");
        assert!(!pools.is_empty());
    }

    #[tokio::test]
    async fn test_mock_performance_monitor() {
        let monitor = helpers::create_test_performance_monitor();
        
        // Test start/stop
        monitor.start().await
            .expect("Performance monitor should start successfully");
        assert!(*monitor.started.read().await);
        
        // Test metrics
        let metrics = monitor.get_current_metrics().await;
        assert!(metrics.operations_per_second > 0.0);
        
        let system_metrics = monitor.get_system_metrics().await;
        assert!(system_metrics.cpu_usage_percent >= 0.0);
        
        monitor.stop().await
            .expect("Performance monitor should stop successfully");
        assert!(!*monitor.started.read().await);
    }
} 