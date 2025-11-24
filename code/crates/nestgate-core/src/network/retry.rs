//! Modern retry Module
//! 
//! This module provides networking functionality using modern Rust patterns
//! and zero-cost abstractions.

use std::time::Duration;
use crate::error::NestGateUnifiedError;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::{NestGateError, Result};
// ==================== MODULE CONSTANTS ====================
/// Module version for compatibility tracking
pub use crate::constants::shared::MODULE_VERSION;
/// Default configuration values
/// Default configuration values from canonical constants
pub use crate::constants::network::{
    DEFAULT_TIMEOUT_MS, DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS
};
// ==================== CORE TYPES ====================
/// Configuration for network retry module
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkRetryConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkRetryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct NetworkRetryConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_connections: usize,
    pub buffer_size: usize,
impl Default for NetworkRetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            max_connections: DEFAULT_MAX_CONNECTIONS,
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
// ==================== USE CANONICAL TRAIT ====================
// Use canonical Service trait from traits module instead of duplicating
pub use super::traits::{Service, HealthStatus};

/// Performance metrics for monitoring
pub struct Metrics {
    pub requests_processed: u64,
    pub errors_encountered: u64,
    pub average_response_time: Duration,
    pub memory_usage_bytes: u64,
impl Default for Metrics {
            requests_processed: 0,
            errors_encountered: 0,
            average_response_time: Duration::from_millis(0),
            memory_usage_bytes: 0,
// ==================== IMPLEMENTATION STUB ====================
/// Default implementation of the service
#[derive(Debug)]
pub struct DefaultService {
    config: NetworkRetryConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
impl DefaultService {
    /// Create a new service instance
    pub fn new(config: NetworkRetryConfig) -> Self {
            config,
            metrics: Arc::new(tokio::sync::RwLock::new(Metrics::default())),
    /// Get current metrics
    pub async fn get_metrics(&self) -> Metrics {
        self.metrics.read().await.clone()
impl Service for DefaultService {
    fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        // Initialization implementation
        tracing::info!("Initializing {} service with config: {:?}", 
                      stringify!(retry), config);
        Ok(())
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send {
        // Health check implementation
        Ok(HealthStatus::Healthy)
    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        // Shutdown implementation
        tracing::info!("Shutting down {} service", stringify!(retry));
// ==================== UTILITY FUNCTIONS ====================
/// Create a default service instance
pub fn create_service() -> DefaultService {
    DefaultService::new(NetworkRetryConfig::default())
/// Validate configuration
pub async fn validate_config(config: &NetworkRetryConfig) -> crate::Result<()> {
    if config.max_connections == 0 {
        return Err(NestGateError::configuration_error(
            "retry",
            "max_connections must be greater than 0"
        ));
    }
    if config.buffer_size == 0 {
        return Err(NestGateError::configuration_error(
            "retry",
            "buffer_size must be greater than 0"
        ));
    }
    Ok(())
// ==================== TESTS ====================

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type NetworkRetryConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkRetryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CONFIG TESTS ====================

    #[test]
    fn test_config_default() {
        let config = NetworkRetryConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
        assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
        assert_eq!(config.timeout, Duration::from_millis(DEFAULT_TIMEOUT_MS));
    }

    #[test]
    fn test_config_custom() {
        let config = NetworkRetryConfig {
            enabled: false,
            timeout: Duration::from_secs(10),
            max_connections: 50,
            buffer_size: 2048,
        };
        assert!(!config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.max_connections, 50);
        assert_eq!(config.buffer_size, 2048);
    }

    #[test]
    fn test_config_serialization() {
        let config = NetworkRetryConfig::default();
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());
    }

    // ==================== CONFIG VALIDATION TESTS ====================

    #[tokio::test]
    async fn test_config_validation_valid() {
        let config = NetworkRetryConfig::default();
        assert!(validate_config(&config).await.is_ok());
    }

    #[tokio::test]
    async fn test_config_validation_zero_connections() {
        let config = NetworkRetryConfig {
            max_connections: 0,
            ..Default::default()
        };
        let result = validate_config(&config).await;
        assert!(result.is_err());
        assert!(format!("{:?}", result.unwrap_err()).contains("max_connections"));
    }

    #[tokio::test]
    async fn test_config_validation_zero_buffer() {
        let config = NetworkRetryConfig {
            buffer_size: 0,
            ..Default::default()
        };
        let result = validate_config(&config).await;
        assert!(result.is_err());
        assert!(format!("{:?}", result.unwrap_err()).contains("buffer_size"));
    }

    #[tokio::test]
    async fn test_config_validation_edge_case() {
        let config = NetworkRetryConfig {
            max_connections: 1,
            buffer_size: 1,
            ..Default::default()
        };
        assert!(validate_config(&config).await.is_ok());
    }

    // ==================== SERVICE TESTS ====================

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_service();
        // Service created successfully
        assert!(format!("{:?}", service).contains("DefaultService"));
    }

    #[tokio::test]
    async fn test_service_with_custom_config() {
        let config = NetworkRetryConfig {
            enabled: true,
            timeout: Duration::from_secs(5),
            max_connections: 100,
            buffer_size: 4096,
        };
        let service = DefaultService::new(config);
        assert!(format!("{:?}", service).contains("DefaultService"));
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
    async fn test_service_initialize() {
        let service = create_service();
        let result = service.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_shutdown() {
        let service = create_service();
        let result = service.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = create_service();
        
        // Initialize
        assert!(service.initialize().await.is_ok());
        
        // Health check
        let health = service.health_check().await;
        assert!(health.is_ok());
        assert_eq!(
            health.expect("Test: health_check should return Ok"),
            HealthStatus::Healthy
        );
        
        // Shutdown
        assert!(service.shutdown().await.is_ok());
    }

    // ==================== METRICS TESTS ====================

    #[tokio::test]
    async fn test_metrics_default() {
        let service = create_service();
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
        assert_eq!(metrics.average_response_time, Duration::from_millis(0));
        assert_eq!(metrics.memory_usage_bytes, 0);
    }

    #[tokio::test]
    async fn test_metrics_structure() {
        let metrics = Metrics::default();
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
    }

    #[tokio::test]
    async fn test_metrics_read_multiple_times() {
        let service = create_service();
        let metrics1 = service.get_metrics().await;
        let metrics2 = service.get_metrics().await;
        
        // Both reads should succeed
        assert_eq!(metrics1.requests_processed, metrics2.requests_processed);
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_config_extreme_timeout() {
        let config = NetworkRetryConfig {
            timeout: Duration::from_secs(3600), // 1 hour
            ..Default::default()
        };
        assert_eq!(config.timeout, Duration::from_secs(3600));
    }

    #[test]
    fn test_config_zero_timeout() {
        let config = NetworkRetryConfig {
            timeout: Duration::from_millis(0),
            ..Default::default()
        };
        assert_eq!(config.timeout, Duration::from_millis(0));
    }

    #[test]
    fn test_config_max_values() {
        let config = NetworkRetryConfig {
            enabled: true,
            timeout: Duration::from_secs(u64::MAX / 1000), // Near max but avoids overflow
            max_connections: usize::MAX,
            buffer_size: usize::MAX,
        };
        assert_eq!(config.max_connections, usize::MAX);
        assert_eq!(config.buffer_size, usize::MAX);
    }

    // ==================== CONCURRENT TESTS ====================

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        let service = Arc::new(create_service());
        
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let service = Arc::clone(&service);
                tokio::spawn(async move {
                    service.health_check().await
                })
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
                tokio::spawn(async move {
                    service.get_metrics().await
                })
            })
            .collect();
        
        for handle in handles {
            let metrics = handle.await.expect("Task should complete");
            assert_eq!(metrics.requests_processed, 0);
        }
    }

    #[tokio::test]
    async fn test_concurrent_service_operations() {
        let service = Arc::new(create_service());
        
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let service = Arc::clone(&service);
                tokio::spawn(async move {
                    if i % 2 == 0 {
                        service.health_check().await.map(|_| ())
                    } else {
                        service.get_metrics().await;
                        Ok(())
                    }
                })
            })
            .collect();
        
        for handle in handles {
            assert!(handle.await.is_ok());
        }
    }
}
