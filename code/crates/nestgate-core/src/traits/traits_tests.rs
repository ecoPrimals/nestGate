//! Fresh tests for the canonical traits system
//!
//! These tests validate the current trait system with its evolved API.

use super::canonical::types::{ProviderCapabilities, ProviderHealth, ServiceCapabilities};
use super::*;
use std::collections::HashMap;
use std::time::SystemTime;

// ==================== TEST FIXTURES ====================

/// Test configuration type
#[derive(Debug, Clone, PartialEq)]
struct TestConfig {
    name: String,
    enabled: bool,
    timeout_ms: u64,
}

impl Default for TestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "test-service".to_string(),
            enabled: true,
            timeout_ms: 5000,
        }
    }
}

/// Test health status type
#[derive(Debug, Clone, PartialEq)]
struct TestHealth {
    status: String,
    uptime_seconds: u64,
}

/// Test metrics type
#[derive(Debug, Clone, PartialEq)]
struct TestMetrics {
    requests: u64,
    errors: u64,
    latency_ms: u64,
}

/// Test error type
#[derive(Debug, Clone, PartialEq)]
struct TestError {
    message: String,
}

impl std::fmt::Display for TestError {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestError: {}", self.message)
    }
}

impl std::error::Error for TestError {}

// ==================== MOCK SERVICE IMPLEMENTATION ====================

/// Mock service implementation for testing CanonicalService trait
#[allow(dead_code)]
struct MockService {
    config: TestConfig,
    started: bool,
    health: TestHealth,
    metrics: TestMetrics,
}

impl MockService {
    /// Creates a new instance
    fn new() -> Self {
        Self {
            config: TestConfig::default(),
            started: false,
            health: TestHealth {
                status: "healthy".to_string(),
                uptime_seconds: 0,
            },
            metrics: TestMetrics {
                requests: 0,
                errors: 0,
                latency_ms: 50,
            },
        }
    }
}

impl CanonicalService for MockService {
    /// Type alias for Config
    type Config = TestConfig;
    /// Type alias for Health
    type Health = TestHealth;
    /// Type alias for Metrics
    type Metrics = TestMetrics;
    /// Type alias for Error
    type Error = TestError;

    /// Start
    async fn start(&self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    /// Stop
    async fn stop(&self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    /// Checks if Healthy
    async fn is_healthy(&self) -> std::result::Result<Self::Health, Self::Error> {
        Ok(self.health.clone())
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> std::result::Result<Self::Metrics, Self::Error> {
        Ok(self.metrics.clone())
    }

    /// Capabilities
    async fn capabilities(&self) -> std::result::Result<ServiceCapabilities, Self::Error> {
        Ok(ServiceCapabilities {
            can_scale: true,
            can_migrate: false,
            can_backup: true,
            supported_protocols: vec!["http".to_string()],
        })
    }

    /// Validates  Config
    async fn validate_config(&self, _config: &Self::Config) -> Result<Vec<String>, Self::Error> {
        Ok(vec![])
    }

    /// Service Id
    fn service_id(&self) -> &str {
        "mock-service"
    }
}

// ==================== CANONICAL SERVICE TESTS ====================

#[tokio::test]
async fn test_service_start() {
    let service = MockService::new();
    let result = service.start().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_stop() {
    let service = MockService::new();
    let result = service.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_health() {
    let service = MockService::new();
    let health = service.is_healthy().await;
    assert!(health.is_ok());
    let health_status = health.expect("Test setup failed");
    assert_eq!(health_status.status, "healthy");
}

#[tokio::test]
async fn test_service_metrics() {
    let service = MockService::new();
    let metrics = service.get_metrics().await;
    assert!(metrics.is_ok());
    let metric_data = metrics.expect("Test setup failed");
    assert_eq!(metric_data.requests, 0);
    assert_eq!(metric_data.latency_ms, 50);
}

#[tokio::test]
async fn test_service_capabilities() {
    let service = MockService::new();
    let caps = service.capabilities().await;
    assert!(caps.is_ok());
    let capabilities = caps.expect("Test setup failed");
    assert!(capabilities.can_scale);
    assert!(!capabilities.can_migrate);
    assert!(capabilities.can_backup);
}

#[tokio::test]
async fn test_service_validate_config() {
    let service = MockService::new();
    let config = TestConfig::default();
    let result = service.validate_config(&config).await;
    assert!(result.is_ok());
    let errors = result.expect("Test setup failed");
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_service_id() {
    let service = MockService::new();
    assert_eq!(service.service_id(), "mock-service");
}

#[tokio::test]
async fn test_service_lifecycle() {
    let service = MockService::new();

    // Start service
    assert!(service.start().await.is_ok());

    // Check health while running
    let health = service.is_healthy().await;
    assert!(health.is_ok());

    // Stop service
    assert!(service.stop().await.is_ok());
}

// ==================== MOCK PROVIDER IMPLEMENTATION ====================

/// Mock provider for testing CanonicalProvider trait
struct MockProvider {
    config: TestConfig,
}

impl MockProvider {
    /// Creates a new instance
    fn new() -> Self {
        Self {
            config: TestConfig::default(),
        }
    }
}

impl CanonicalProvider<String> for MockProvider {
    /// Type alias for Config
    type Config = TestConfig;
    /// Type alias for Error
    type Error = TestError;
    /// Type alias for Metadata
    type Metadata = HashMap<String, String>;

    /// Provide
    async fn provide(&self, _config: Self::Config) -> std::result::Result<String, Self::Error> {
        Ok("provided-value".to_string())
    }

    /// Configure
    async fn configure(&mut self, config: Self::Config) -> std::result::Result<(), Self::Error> {
        self.config = config;
        Ok(())
    }

    /// Metadata
    async fn metadata(&self) -> std::result::Result<Self::Metadata, Self::Error> {
        let mut meta = HashMap::new();
        meta.insert("name".to_string(), self.config.name.clone());
        Ok(meta)
    }

    /// Health Check
    async fn health_check(&self) -> std::result::Result<ProviderHealth, Self::Error> {
        Ok(ProviderHealth {
            is_healthy: true,
            last_check: SystemTime::now(),
            health: "healthy".to_string(),
        })
    }

    /// Capabilities
    async fn capabilities(&self) -> std::result::Result<ProviderCapabilities, Self::Error> {
        use crate::unified_enums::service_types::UnifiedServiceType;
        Ok(ProviderCapabilities {
            supported_types: vec![UnifiedServiceType::Generic],
            max_instances: Some(100),
        })
    }
}

// ==================== CANONICAL PROVIDER TESTS ====================

#[tokio::test]
async fn test_provider_provide() {
    let provider = MockProvider::new();
    let config = TestConfig::default();
    let result = provider.provide(config).await;
    assert!(result.is_ok());
    assert_eq!(result.expect("Test setup failed"), "provided-value");
}

#[tokio::test]
async fn test_provider_configure() {
    let mut provider = MockProvider::new();
    let config = TestConfig {
        name: "new-name".to_string(),
        enabled: false,
        timeout_ms: 3000,
    };
    let result = provider.configure(config.clone()).await;
    assert!(result.is_ok());
    assert_eq!(provider.config.name, "new-name");
}

#[tokio::test]
async fn test_provider_metadata() {
    let provider = MockProvider::new();
    let result = provider.metadata().await;
    assert!(result.is_ok());
    let meta = result.expect("Test setup failed");
    assert!(meta.contains_key("name"));
}

#[tokio::test]
async fn test_provider_health_check() {
    let provider = MockProvider::new();
    let result = provider.health_check().await;
    assert!(result.is_ok());
    let health = result.expect("Test setup failed");
    assert!(health.is_healthy);
    assert_eq!(health.health, "healthy");
}

#[tokio::test]
async fn test_provider_capabilities() {
    let provider = MockProvider::new();
    let result = provider.capabilities().await;
    assert!(result.is_ok());
    let caps = result.expect("Test setup failed");
    assert_eq!(caps.max_instances, Some(100));
    assert_eq!(caps.supported_types.len(), 1);
}

// ==================== TRAIT AVAILABILITY TESTS ====================

#[test]
fn test_canonical_service_trait_available() {
    ///  Test
    fn _test<T: CanonicalService>() {}
    // Trait is accessible
}

#[test]
fn test_canonical_storage_trait_available() {
    ///  Test
    fn _test<T: CanonicalStorage>() {}
    // Trait is accessible
}

#[test]
fn test_canonical_network_trait_available() {
    ///  Test
    fn _test<T: CanonicalNetwork>() {}
    // Trait is accessible
}

#[test]
fn test_canonical_security_trait_available() {
    ///  Test
    fn _test<T: CanonicalSecurity>() {}
    // Trait is accessible
}

#[test]
fn test_canonical_provider_trait_available() {
    ///  Test
    fn _test<T, P: CanonicalProvider<T>>() {}
    // Trait with generic parameter is accessible
}

#[test]
fn test_unified_storage_trait_available() {
    ///  Test
    fn _test<T: UnifiedStorage>() {}
    // Trait is accessible
}

// ==================== RE-EXPORT TESTS ====================

#[test]
fn test_canonical_service_reexported() {
    use super::CanonicalService;
    ///  Test
    fn _test<T: CanonicalService>() {}
    // Re-export works
}

#[test]
fn test_canonical_provider_reexported() {
    use super::CanonicalProvider;
    ///  Test
    fn _test<T, P: CanonicalProvider<T>>() {}
    // Re-export works
}

#[test]
fn test_unified_storage_reexported() {
    use super::UnifiedStorage;
    ///  Test
    fn _test<T: UnifiedStorage>() {}
    // Re-export works
}
