//! **CANONICAL NETWORK SERVICE TRAITS**
//!
//! This module provides THE canonical networking trait definitions.
//! All other network modules should use these traits.
//!
//! **DO NOT DUPLICATE** - Import from here instead.

use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

// ==================== MODULE CONSTANTS ====================

/// Module version for compatibility tracking
pub use crate::constants::shared::MODULE_VERSION;

/// Default configuration values from canonical constants
pub use crate::constants::network::{
    DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS, DEFAULT_TIMEOUT_MS,
};

// ==================== CORE TYPES ====================

/// Configuration for network services
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for NetworkTraits
pub struct NetworkTraitsConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
    /// Max Connections
    pub max_connections: usize,
    /// Size of buffer
    pub buffer_size: usize,
}

impl Default for NetworkTraitsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            max_connections: DEFAULT_MAX_CONNECTIONS,
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
}

/// Type alias for convenience in tests
pub type Config = NetworkTraitsConfig;

/// **CANONICAL NETWORK SERVICE TRAIT**
///
/// This is THE single source of truth for network service interfaces.
/// All network modules should use this trait instead of defining their own.
///
/// # Native Async
///
/// Uses `impl Future` for zero-cost abstractions (no async_trait overhead).
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::network::traits::Service;
///
/// struct MyNetworkService;
///
/// impl Service for MyNetworkService {
///     fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send {
///         async move { Ok(()) }
///     }
///     // ... other methods
/// }
/// ```
pub trait Service: Send + Sync {
    /// Initialize the service
    fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Check service health
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send;

    /// Shutdown the service gracefully
    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
}

/// Performance metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metrics
pub struct Metrics {
    /// Requests Processed
    pub requests_processed: u64,
    /// Errors Encountered
    pub errors_encountered: u64,
    /// Average Response Time
    pub average_response_time: Duration,
    /// Memory Usage Bytes
    pub memory_usage_bytes: u64,
}

impl Default for Metrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_processed: 0,
            errors_encountered: 0,
            average_response_time: Duration::from_millis(0),
            memory_usage_bytes: 0,
        }
    }
}

// ==================== IMPLEMENTATION STUB ====================

/// Default implementation of the service
#[derive(Debug)]
/// Service implementation for Default
pub struct DefaultService {
    _config: NetworkTraitsConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
}

impl DefaultService {
    /// Create a new service instance
    pub fn new(config: NetworkTraitsConfig) -> Self {
        Self {
            _config: config,
            metrics: Arc::new(tokio::sync::RwLock::new(Metrics::default())),
        }
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> Metrics {
        self.metrics.read().await.clone()
    }
}

impl Service for DefaultService {
    /// Initialize
    async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing network service");
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<HealthStatus> {
        Ok(HealthStatus::Healthy)
    }

    /// Shutdown
    async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down network service");
        Ok(())
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a default service instance
pub fn create_service() -> DefaultService {
    DefaultService::new(NetworkTraitsConfig::default())
}

/// Validate configuration
pub fn validate_config(config: &NetworkTraitsConfig) -> Result<()> {
    if config.max_connections == 0 {
        return Err(NestGateError::configuration_error(
            "network_traits",
            "max_connections must be greater than 0",
        ));
    }

    if config.buffer_size == 0 {
        return Err(NestGateError::configuration_error(
            "network_traits",
            "buffer_size must be greater than 0",
        ));
    }

    Ok(())
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== NETWORK TRAITS CONFIG TESTS ====================

    #[test]
    fn test_config_default() {
        let config = NetworkTraitsConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
        assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
        assert_eq!(config.timeout, Duration::from_millis(DEFAULT_TIMEOUT_MS));
    }

    #[test]
    fn test_config_custom() {
        let config = NetworkTraitsConfig {
            enabled: false,
            timeout: Duration::from_secs(60),
            max_connections: 500,
            buffer_size: 16384,
        };
        assert!(!config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_connections, 500);
        assert_eq!(config.buffer_size, 16384);
    }

    #[test]
    fn test_config_type_alias() {
        let config = Config::default();
        assert!(config.enabled);
    }

    #[test]
    fn test_config_serialization() {
        let config = NetworkTraitsConfig::default();
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());

        let deserialized: std::result::Result<NetworkTraitsConfig, _> =
            serde_json::from_str(&serialized.expect("Test: serialization should succeed"));
        assert!(deserialized.is_ok());
    }

    // ==================== CONFIG VALIDATION TESTS ====================

    #[test]
    fn test_config_validation_valid() {
        let config = NetworkTraitsConfig::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_config_validation_zero_connections() {
        let bad_config = Config {
            max_connections: 0,
            ..Default::default()
        };
        let result = validate_config(&bad_config);
        assert!(result.is_err());
        assert!(format!("{:?}", result.unwrap_err()).contains("max_connections"));
    }

    #[test]
    fn test_config_validation_zero_buffer() {
        let bad_config = Config {
            buffer_size: 0,
            ..Default::default()
        };
        let result = validate_config(&bad_config);
        assert!(result.is_err());
        assert!(format!("{:?}", result.unwrap_err()).contains("buffer_size"));
    }

    #[test]
    fn test_config_validation_valid_edge_cases() {
        let config = Config {
            max_connections: 1,
            buffer_size: 1,
            ..Default::default()
        };
        assert!(validate_config(&config).is_ok());
    }

    // ==================== HEALTH STATUS TESTS ====================

    #[test]
    fn test_health_status_variants() {
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        let unhealthy = HealthStatus::Unhealthy;

        assert_eq!(healthy, HealthStatus::Healthy);
        assert_eq!(degraded, HealthStatus::Degraded);
        assert_eq!(unhealthy, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Degraded, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus::Healthy;
        let serialized = serde_json::to_string(&status);
        assert!(serialized.is_ok());

        let deserialized: std::result::Result<HealthStatus, _> =
            serde_json::from_str(&serialized.expect("Test: serialization should succeed"));
        assert!(deserialized.is_ok());
        assert_eq!(
            deserialized.expect("Test: deserialization should succeed"),
            HealthStatus::Healthy
        );
    }

    // ==================== METRICS TESTS ====================

    #[test]
    fn test_metrics_default() {
        let metrics = Metrics::default();
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
        assert_eq!(metrics.average_response_time, Duration::from_millis(0));
        assert_eq!(metrics.memory_usage_bytes, 0);
    }

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics {
            requests_processed: 1000,
            errors_encountered: 10,
            average_response_time: Duration::from_millis(150),
            memory_usage_bytes: 1024 * 1024 * 100, // 100 MB
        };
        assert_eq!(metrics.requests_processed, 1000);
        assert_eq!(metrics.errors_encountered, 10);
        assert_eq!(metrics.average_response_time, Duration::from_millis(150));
        assert_eq!(metrics.memory_usage_bytes, 1024 * 1024 * 100);
    }

    #[test]
    fn test_metrics_serialization() {
        let metrics = Metrics::default();
        let serialized = serde_json::to_string(&metrics);
        assert!(serialized.is_ok());
    }

    // ==================== SERVICE TESTS ====================

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_service();
        assert!(service.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_service_health_check() {
        let service = create_service();
        let health = service.health_check().await;
        assert!(health.is_ok());
        assert_eq!(
            health.expect("Test: health_check should return Ok"),
            HealthStatus::Healthy
        );
    }

    #[tokio::test]
    async fn test_service_shutdown() {
        let service = create_service();
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = create_service();

        // Initialize
        assert!(service.initialize().await.is_ok());

        // Check health
        let health = service.health_check().await;
        assert!(health.is_ok());
        assert_eq!(
            health.expect("Test: health_check should return Ok"),
            HealthStatus::Healthy
        );

        // Shutdown
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_service_metrics() {
        let service = create_service();
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
    }

    #[tokio::test]
    async fn test_service_custom_config() {
        let config = NetworkTraitsConfig {
            enabled: true,
            timeout: Duration::from_secs(30),
            max_connections: 200,
            buffer_size: 8192,
        };
        let service = DefaultService::new(config);
        assert!(service.initialize().await.is_ok());
    }

    // ==================== DEFAULT SERVICE TESTS ====================

    #[test]
    fn test_default_service_creation() {
        let config = NetworkTraitsConfig::default();
        let service = DefaultService::new(config);
        // Service created successfully
        assert!(format!("{:?}", service).contains("DefaultService"));
    }

    #[tokio::test]
    async fn test_default_service_metrics_read() {
        let service = create_service();
        let metrics1 = service.get_metrics().await;
        let metrics2 = service.get_metrics().await;

        // Both reads should succeed and have same initial values
        assert_eq!(metrics1.requests_processed, metrics2.requests_processed);
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_config_extreme_values() {
        let config = NetworkTraitsConfig {
            enabled: true,
            timeout: Duration::from_secs(3600), // 1 hour
            max_connections: usize::MAX,
            buffer_size: usize::MAX,
        };
        // Should be able to create config with extreme values
        assert_eq!(config.max_connections, usize::MAX);
    }

    #[test]
    fn test_metrics_extreme_values() {
        let metrics = Metrics {
            requests_processed: u64::MAX,
            errors_encountered: u64::MAX,
            average_response_time: Duration::from_secs(u64::MAX),
            memory_usage_bytes: u64::MAX,
        };
        assert_eq!(metrics.requests_processed, u64::MAX);
        assert_eq!(metrics.errors_encountered, u64::MAX);
    }

    #[test]
    fn test_config_zero_timeout() {
        let config = NetworkTraitsConfig {
            timeout: Duration::from_millis(0),
            ..Default::default()
        };
        // Zero timeout is allowed (validation only checks connections and buffer)
        assert_eq!(config.timeout, Duration::from_millis(0));
    }

    // ==================== CONCURRENT TESTS ====================

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        let service = Arc::new(create_service());

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let service = Arc::clone(&service);
                tokio::spawn(async move { service.health_check().await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert!(result.is_ok());
            assert_eq!(
                result.expect("Test: health check result should be Ok"),
                HealthStatus::Healthy
            );
        }
    }

    #[tokio::test]
    async fn test_concurrent_metrics_reads() {
        let service = Arc::new(create_service());

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let service = Arc::clone(&service);
                tokio::spawn(async move { service.get_metrics().await })
            })
            .collect();

        for handle in handles {
            let metrics = handle.await.expect("Task should complete");
            assert_eq!(metrics.requests_processed, 0);
        }
    }
}
