/// **TEST FACTORY ABSTRACTION**
/// Consolidates all scattered test helper functions into a proper factory pattern
/// 
/// This module eliminates the helper function anti-pattern by providing:
/// - Centralized test object creation
/// - Configurable test scenarios
/// - Consistent test data generation
/// - Proper resource management
/// 
/// **REPLACES**: Multiple helper modules across test files
/// **PROBLEM SOLVED**: Helper function proliferation and duplication

use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use crate::{Result, NestGateError};
use crate::traits::{UniversalService, UniversalServiceRequest, UniversalServiceResponse};
use crate::unified_enums::{UnifiedServiceType, UnifiedServiceState};
// **MIGRATED**: Using canonical config system instead of deprecated unified_types
use crate::config::canonical_master::NestGateCanonicalConfig as NestGateCanonicalConfig;
// **MIGRATED**: Using local UnifiedServiceConfig definition
use crate::service_discovery::config::UnifiedServiceConfig;

// ==================== SECTION ====================

/// Universal Test Factory trait for creating test objects
pub trait TestFactory<T> {
    type Config;
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Create a test instance with default configuration
    fn create_default() -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;
    
    /// Create a test instance with custom configuration
    fn create_with_config(config: Self::Config) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;
    
    /// Create a test instance for specific scenario
    fn create_for_scenario(scenario: TestScenario) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;
    
    /// Create multiple test instances
    fn create_batch(count: usize) -> impl std::future::Future<Output = Result<Vec<T>> + Send;
}

/// Test scenarios for different testing contexts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestScenario {
    /// Unit testing scenario
    Unit,
    /// Integration testing scenario
    Integration,
    /// Performance testing scenario
    Performance,
    /// Chaos testing scenario
    Chaos,
    /// Production simulation scenario
    ProductionSim,
    /// Custom scenario with name
    Custom(String),
}

// ==================== SECTION ====================

/// Factory for creating test services
pub struct ServiceTestFactory;

impl ServiceTestFactory {
    /// Create a mock service with specified behavior
    pub async fn create_mock_service(
        service_type: UnifiedServiceType,
        behavior: ServiceBehavior,
    ) -> Result<Arc<dyn UniversalService<Config = UnifiedServiceConfig, Health = bool>>> {
        let service = MockTestService::new(service_type, behavior);
        Ok(Arc::new(service))
    }
    
    /// Create a service registry for testing
    pub fn create_service_registry() -> TestServiceRegistry {
        TestServiceRegistry::new()
    }
    
    /// Create a service with realistic delays and behaviors
    pub async fn create_realistic_service(
        service_type: UnifiedServiceType,
    ) -> Result<Arc<dyn UniversalService<Config = UnifiedServiceConfig, Health = bool>>> {
        let behavior = ServiceBehavior {
            response_delay: Duration::from_millis(50),
            success_rate: 0.95,
            enable_realistic_errors: true,
        };
        Self::create_mock_service(service_type, behavior).await
    }
}

/// Service behavior configuration for testing
#[derive(Debug, Clone)]
pub struct ServiceBehavior {
    pub response_delay: Duration,
    pub success_rate: f64,
    pub enable_realistic_errors: bool,
}

impl Default for ServiceBehavior {
    fn default() -> Self {
        Self {
            response_delay: Duration::from_millis(1), // Fast for tests
            success_rate: 1.0, // Always succeed by default
            enable_realistic_errors: false,
        }
    }
}

// ==================== SECTION ====================

/// Factory for creating test storage objects
pub struct StorageTestFactory;

impl StorageTestFactory {
    /// Create a test storage backend
    pub async fn create_storage_backend(
        storage_type: crate::universal_storage::UniversalStorageType,
    ) -> Result<Arc<dyn crate::universal_storage::UniversalStorageBackend>> {
        match storage_type {
            crate::universal_storage::UniversalStorageType::Memory => {
                Ok(Arc::new(MemoryStorageBackend::new()))
            }
            crate::universal_storage::UniversalStorageType::Local => {
                Ok(Arc::new(LocalStorageBackend::new("/tmp/nestgate-test")))
            }
            _ => Ok(Arc::new(MockStorageBackend::new(storage_type))),
        }
    }
    
    /// Create a storage resource for testing
    pub fn create_storage_resource(
        name: &str,
        storage_type: crate::universal_storage::UniversalStorageType,
    ) -> crate::universal_storage::UniversalStorageResource {
        use crate::universal_storage::*;
        use chrono::Utc;
        
        UniversalStorageResource {
            resource_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            storage_type,
            resource_type: StorageResourceType::Dataset,
            path: format!("/test/{}", name).into(),
            size_bytes: 1024 * 1024, // 1MB
            available_bytes: 1024 * 1024,
            used_bytes: 0,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            accessed_at: Some(Utc::now()),
            tier: crate::unified_enums::UnifiedTierType::Hot,
            capabilities: vec![StorageCapability::ReadWrite],
            performance: StoragePerformanceMetrics::default(),
            metadata: HashMap::new(),
            tags: vec!["test".to_string()],
            permissions: StoragePermissions::default(),
            health_status: StorageHealthStatus::Healthy,
        }
    }
}

// ==================== SECTION ====================

/// Factory for creating test configurations
pub struct ConfigTestFactory;

impl ConfigTestFactory {
    /// Create a test configuration for specific scenario
    pub fn create_for_scenario(scenario: TestScenario) -> NestGateCanonicalConfig {
        match scenario {
            TestScenario::Unit => Self::create_unit_test_config(),
            TestScenario::Integration => Self::create_integration_test_config(),
            TestScenario::Performance => Self::create_performance_test_config(),
            TestScenario::Chaos => Self::create_chaos_test_config(),
            TestScenario::ProductionSim => Self::create_production_sim_config(),
            TestScenario::Custom(_) => Self::create_default_config(),
        }
    }
    
    /// Create configuration optimized for unit tests
    pub fn create_unit_test_config() -> NestGateCanonicalConfig {
        let mut config = NestGateCanonicalConfig::default();
        config.network.port = 0; // Random port
        config.timeouts.connection_timeout = Duration::from_millis(100);
        config.retry.max_attempts = 1;
        config
    }
    
    /// Create configuration optimized for integration tests
    pub fn create_integration_test_config() -> NestGateCanonicalConfig {
        let mut config = NestGateCanonicalConfig::default();
        config.network.port = 0; // Random port
        config.timeouts.connection_timeout = Duration::from_millis(500);
        config.retry.max_attempts = 2;
        config.monitoring.enable_metrics = false; // Reduce overhead
        config
    }
    
    /// Create configuration optimized for performance tests
    pub fn create_performance_test_config() -> NestGateCanonicalConfig {
        let mut config = NestGateCanonicalConfig::default();
        config.network.port = 0;
        config.timeouts.connection_timeout = Duration::from_secs(1);
        config.retry.max_attempts = 3;
        config.cache.enable_cache = true;
        config.cache.cache_size_mb = 100;
        config
    }
    
    /// Create configuration for chaos testing
    pub fn create_chaos_test_config() -> NestGateCanonicalConfig {
        let mut config = NestGateCanonicalConfig::default();
        config.network.port = 0;
        config.timeouts.connection_timeout = Duration::from_millis(50); // Aggressive timeouts
        config.retry.max_attempts = 5; // More retries for chaos
        config
    }
    
    /// Create configuration simulating production
    pub fn create_production_sim_config() -> NestGateCanonicalConfig {
        let mut config = NestGateCanonicalConfig::default();
        config.network.port = 8080;
        config.timeouts.connection_timeout = Duration::from_secs(30);
        config.retry.max_attempts = 3;
        config.monitoring.enable_metrics = true;
        config.cache.enable_cache = true;
        config.cache.cache_size_mb = 256;
        config
    }
    
    /// Create default test configuration
    pub fn create_default_config() -> NestGateCanonicalConfig {
        Self::create_unit_test_config()
    }
}

// ==================== SECTION ====================

/// Factory for creating test data
pub struct TestDataFactory;

impl TestDataFactory {
    /// Generate test data for specific scenario
    pub fn generate_data<T>(scenario: TestScenario, count: usize) -> Vec<T>
    where
        T: TestDataGenerator,
    {
        (0..count).map(|i| T::generate_for_scenario(&scenario, i)).collect()
    }
    
    /// Generate realistic test data
    pub fn generate_realistic_data<T>(count: usize) -> Vec<T>
    where
        T: TestDataGenerator,
    {
        Self::generate_data(TestScenario::ProductionSim, count)
    }
    
    /// Generate test UUIDs
    pub fn generate_uuids(count: usize) -> Vec<String> {
        (0..count).map(|_| Uuid::new_v4().to_string()).collect()
    }
    
    /// Generate test timestamps
    pub fn generate_timestamps(count: usize) -> Vec<SystemTime> {
        let base = SystemTime::now();
        (0..count)
            .map(|i| base + Duration::from_secs(i as u64))
            .collect()
    }
}

/// Trait for generating test data
pub trait TestDataGenerator {
    fn generate_for_scenario(scenario: &TestScenario, index: usize) -> Self;
}

// ==================== SECTION ====================

/// Mock service for testing
pub struct MockTestService {
    service_type: UnifiedServiceType,
    behavior: ServiceBehavior,
    config: UnifiedServiceConfig,
}

impl MockTestService {
    pub fn new(service_type: UnifiedServiceType, behavior: ServiceBehavior) -> Self {
        Self {
            service_type,
            behavior,
            config: UnifiedServiceConfig::default(),
        }
    }
}

impl UniversalService for MockTestService {
    type Config = UnifiedServiceConfig;
    type Health = bool;
    
    async fn initialize(&mut self, config: Self::Config) -> Result<()> {
        tokio::time::sleep(self.behavior.response_delay).await;
        self.config = config;
        Ok(())
    }
    
    async fn start(&mut self) -> Result<()> {
        tokio::time::sleep(self.behavior.response_delay).await;
        if self.should_simulate_error() {
            return Err(NestGateError::service_error("Simulated start error"));
        }
        Ok(())
    }
    
    async fn stop(&mut self) -> Result<()> {
        tokio::time::sleep(self.behavior.response_delay).await;
        Ok(())
    }
    
    async fn health_check(&self) -> Result<bool> {
        tokio::time::sleep(self.behavior.response_delay).await;
        Ok(!self.behavior.should_fail)
    }
    
    async fn handle_request(&self, _request: UniversalServiceRequest) -> Result<UniversalServiceResponse> {
        tokio::time::sleep(self.behavior.response_delay).await;
        Ok(UniversalServiceResponse {
            request_id: Uuid::new_v4().to_string(),
            status: crate::traits::UniversalResponseStatus::Success,
            data: Some(serde_json::json!({"message": "test response"})),
            error: None,
            metadata: HashMap::new(),
        })
    }

    async fn status(&self) -> crate::unified_enums::UnifiedServiceState {
        if self.behavior.should_fail {
            crate::unified_enums::UnifiedServiceState::Failed
        } else {
            crate::unified_enums::UnifiedServiceState::Running
        }
    }

    async fn health(&self) -> Result<Self::Health> {
        self.health_check().await
    }

    fn service_id(&self) -> &str {
        "mock-test-service"
    }

    fn service_type(&self) -> crate::unified_enums::UnifiedServiceType {
        self.service_type.clone()
    }
}

impl MockTestService {
    fn should_simulate_error(&self) -> bool {
        if !self.behavior.enable_realistic_errors {
            return false;
        }
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() > self.behavior.success_rate
    }
}

/// Service health information for testing
#[derive(Debug, Clone, Serialize)]
pub struct ServiceHealth {
    pub is_healthy: bool,
    pub uptime: Duration,
    pub memory_usage: f64,
}

/// Test service registry
pub struct TestServiceRegistry {
    services: HashMap<String, Arc<dyn UniversalService<Config = UnifiedServiceConfig, Health = ServiceHealth>>>,
}

impl TestServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }
    
    pub fn register_service(&mut self, name: String, service: Arc<dyn UniversalService<Config = UnifiedServiceConfig, Health = ServiceHealth>>) {
        self.services.insert(name, service);
    }
    
    pub fn get_service(&self, name: &str) -> Option<&Arc<dyn UniversalService<Config = UnifiedServiceConfig, Health = ServiceHealth>>> {
        self.services.get(name)
    }
    
    pub fn list_services(&self) -> Vec<&String> {
        self.services.keys().collect()
    }
}

// ==================== SECTION ====================

/// Mock memory storage backend
pub struct MemoryStorageBackend {
    data: Arc<tokio::sync::RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryStorageBackend {
    pub fn new() -> Self {
        Self {
            data: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

impl crate::universal_storage::UniversalStorageBackend for MemoryStorageBackend {
    async fn handle_request(
        &self,
        request: crate::universal_storage::UniversalStorageRequest,
    ) -> Result<crate::universal_storage::UniversalStorageResponse> {
        use crate::universal_storage::unified_storage_traits::{UnifiedStorageRequest, UnifiedStorageResponse, UnifiedStorageBackend, UnifiedStorageType, StorageHealthStatus, StoragePerformanceMetrics, StorageResourceConfig, StorageCapability};
        
        match request {
            UniversalStorageRequest::Read { path, .. } => {
                let data = self.data.read().await;
                if let Some(content) = data.get(&path) {
                    Ok(UniversalStorageResponse::ReadResponse {
                        data: content.clone(),
                        metadata: None,
                    })
                } else {
                    Ok(UniversalStorageResponse::Error {
                        error: "File not found".to_string(),
                        error_code: "NOT_FOUND".to_string(),
                    })
                }
            }
            UniversalStorageRequest::Write { path, data, .. } => {
                let mut storage = self.data.write().await;
                storage.insert(path, data.clone());
                Ok(UniversalStorageResponse::WriteResponse {
                    bytes_written: data.len() as u64,
                    checksum: None,
                })
            }
            _ => Ok(UniversalStorageResponse::Error {
                error: "Operation not implemented".to_string(),
                error_code: "NOT_IMPLEMENTED".to_string(),
            }),
        }
    }
    
    fn backend_type(&self) -> crate::universal_storage::UniversalStorageType {
        crate::universal_storage::UniversalStorageType::Memory
    }
    
    fn capabilities(&self) -> Vec<crate::universal_storage::StorageCapability> {
        vec![
            crate::universal_storage::StorageCapability::ReadWrite,
            crate::universal_storage::StorageCapability::Streaming,
        ]
    }
    
    async fn is_available(&self) -> bool {
        true
    }
    
    async fn health_check(&self) -> Result<crate::universal_storage::StorageHealthStatus> {
        Ok(crate::universal_storage::StorageHealthStatus::Healthy)
    }
    
    async fn get_metrics(&self) -> Result<crate::universal_storage::StoragePerformanceMetrics> {
        Ok(crate::universal_storage::StoragePerformanceMetrics::default())
    }
    
    async fn initialize(&mut self, _config: crate::universal_storage::StorageResourceConfig) -> Result<()> {
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        self.data.write().await.clear();
        Ok(())
    }
}

/// Mock local storage backend
pub struct LocalStorageBackend {
    base_path: std::path::PathBuf,
}

impl LocalStorageBackend {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: std::path::PathBuf::from(base_path),
        }
    }
}

impl UnifiedStorageBackend for LocalStorageBackend {
    async fn handle_request(
        &self,
        _request: crate::universal_storage::UniversalStorageRequest,
    ) -> Result<crate::universal_storage::UniversalStorageResponse> {
        // Simplified implementation for testing
        Ok(crate::universal_storage::UniversalStorageResponse::Error {
            error: "Local storage not implemented in test".to_string(),
            error_code: "NOT_IMPLEMENTED".to_string(),
        })
    }
    
    fn backend_type(&self) -> UnifiedStorageType {
        UnifiedStorageType::Local
    }
    
    fn capabilities(&self) -> Vec<StorageCapability> {
        vec![StorageCapability::ReadWrite]
    }
    
    async fn is_available(&self) -> bool {
        self.base_path.exists()
    }
    
    async fn health_check(&self) -> Result<crate::universal_storage::StorageHealthStatus> {
        if self.is_available().await {
            Ok(crate::universal_storage::StorageHealthStatus::Healthy)
        } else {
            Ok(crate::universal_storage::StorageHealthStatus::Offline)
        }
    }
    
    async fn get_metrics(&self) -> Result<crate::universal_storage::StoragePerformanceMetrics> {
        Ok(crate::universal_storage::StoragePerformanceMetrics::default())
    }
    
    async fn initialize(&mut self, _config: crate::universal_storage::StorageResourceConfig) -> Result<()> {
        tokio::fs::create_dir_all(&self.base_path).await?;
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Generic mock storage backend
pub struct MockStorageBackend {
    storage_type: crate::universal_storage::UniversalStorageType,
}

impl MockStorageBackend {
    pub fn new(storage_type: crate::universal_storage::UniversalStorageType) -> Self {
        Self { storage_type }
    }
}

impl crate::universal_storage::UniversalStorageBackend for MockStorageBackend {
    async fn handle_request(
        &self,
        _request: UnifiedStorageRequest,
    ) -> Result<UnifiedStorageResponse> {
        Ok(UnifiedStorageResponse::Error {
            error: "Mock storage backend".to_string(),
            error_code: "MOCK".to_string(),
        })
    }
    
    fn backend_type(&self) -> crate::universal_storage::UniversalStorageType {
        self.storage_type.clone()
    }
    
    fn capabilities(&self) -> Vec<crate::universal_storage::StorageCapability> {
        self.storage_type.default_capabilities()
    }
    
    async fn is_available(&self) -> bool {
        true
    }
    
    async fn health_check(&self) -> Result<crate::universal_storage::StorageHealthStatus> {
        Ok(crate::universal_storage::StorageHealthStatus::Healthy)
    }
    
    async fn get_metrics(&self) -> Result<crate::universal_storage::StoragePerformanceMetrics> {
        Ok(crate::universal_storage::StoragePerformanceMetrics::default())
    }
    
    async fn initialize(&mut self, _config: crate::universal_storage::StorageResourceConfig) -> Result<()> {
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
