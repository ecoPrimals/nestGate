//! Modern middleware Module
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
/// Configuration for this module
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkMiddlewareConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkMiddlewareConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct NetworkMiddlewareConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_connections: usize,
    pub buffer_size: usize,
impl Default for NetworkMiddlewareConfig {
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
    config: NetworkMiddlewareConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
impl DefaultService {
    /// Create a new service instance
    pub fn new(config: NetworkMiddlewareConfig) -> Self {
            config,
            metrics: Arc::new(tokio::sync::RwLock::new(Metrics::default())),
    /// Get current metrics
    pub async fn get_metrics(&self) -> Metrics {
        self.metrics.read().await.clone()
impl Service for DefaultService {
    fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        // Initialization implementation
        tracing::info!("Initializing {} service with config: {:?}", 
                      stringify!(middleware), config);
        Ok(())
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send {
        // Health check implementation
        Ok(HealthStatus::Healthy)
    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        // Shutdown implementation
        tracing::info!("Shutting down {} service", stringify!(middleware));
// ==================== UTILITY FUNCTIONS ====================
/// Create a default service instance
pub fn create_service() -> DefaultService {
    DefaultService::new(NetworkMiddlewareConfig::default())
/// Validate configuration
pub async fn validate_config(config: &NetworkMiddlewareConfig) -> crate::Result<()> {
    if config.max_connections == 0 {
        return Err(NestGateError::configuration_error(
            "middleware",
            "max_connections must be greater than 0"
        ));
    }
    if config.buffer_size == 0 {
        return Err(NestGateError::configuration_error(
            "middleware",
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
pub type NetworkMiddlewareConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkMiddlewareConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CONFIG TESTS ====================

    #[test]
    fn test_config_default() {
        let config = NetworkMiddlewareConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
        assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
        assert_eq!(config.timeout, Duration::from_millis(DEFAULT_TIMEOUT_MS));
    }

    #[test]
    fn test_config_custom() {
        let config = NetworkMiddlewareConfig {
            enabled: false,
            timeout: Duration::from_secs(15),
            max_connections: 75,
            buffer_size: 3072,
        };
        assert!(!config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(15));
        assert_eq!(config.max_connections, 75);
        assert_eq!(config.buffer_size, 3072);
    }

    #[test]
    fn test_config_serialization() {
        let config = NetworkMiddlewareConfig::default();
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());
    }

    // ==================== CONFIG VALIDATION TESTS ====================

    #[tokio::test]
    async fn test_config_validation_valid() {
        let config = NetworkMiddlewareConfig::default();
        assert!(validate_config(&config).await.is_ok());
    }

    #[tokio::test]
    async fn test_config_validation_zero_connections() {
        let config = NetworkMiddlewareConfig {
            max_connections: 0,
            ..Default::default()
        };
        let result = validate_config(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_config_validation_zero_buffer() {
        let config = NetworkMiddlewareConfig {
            buffer_size: 0,
            ..Default::default()
        };
        let result = validate_config(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_config_validation_minimal_valid() {
        let config = NetworkMiddlewareConfig {
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
        assert!(format!("{:?}", service).contains("DefaultService"));
    }

    #[tokio::test]
    async fn test_service_custom_config() {
        let config = NetworkMiddlewareConfig {
            enabled: true,
            timeout: Duration::from_secs(20),
            max_connections: 150,
            buffer_size: 8192,
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
        assert!(service.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_service_shutdown() {
        let service = create_service();
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_service_full_lifecycle() {
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
    async fn test_metrics_default_values() {
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
    async fn test_metrics_multiple_reads() {
        let service = create_service();
        let m1 = service.get_metrics().await;
        let m2 = service.get_metrics().await;
        assert_eq!(m1.requests_processed, m2.requests_processed);
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_config_long_timeout() {
        let config = NetworkMiddlewareConfig {
            timeout: Duration::from_secs(7200), // 2 hours
            ..Default::default()
        };
        assert_eq!(config.timeout, Duration::from_secs(7200));
    }

    #[test]
    fn test_config_zero_timeout_allowed() {
        let config = NetworkMiddlewareConfig {
            timeout: Duration::from_millis(0),
            ..Default::default()
        };
        assert_eq!(config.timeout, Duration::from_millis(0));
    }

    #[test]
    fn test_config_large_values() {
        let config = NetworkMiddlewareConfig {
            enabled: true,
            timeout: Duration::from_secs(u64::MAX / 2000),
            max_connections: usize::MAX,
            buffer_size: usize::MAX,
        };
        assert_eq!(config.max_connections, usize::MAX);
    }

    // ==================== CONCURRENT TESTS ====================

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        let service = Arc::new(create_service());
        
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let svc = Arc::clone(&service);
                tokio::spawn(async move { svc.health_check().await })
            })
            .collect();
        
        for h in handles {
            let result = h.await.expect("Task failed");
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_concurrent_metrics_reads() {
        let service = Arc::new(create_service());
        
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let svc = Arc::clone(&service);
                tokio::spawn(async move { svc.get_metrics().await })
            })
            .collect();
        
        for h in handles {
            let metrics = h.await.expect("Task failed");
            assert_eq!(metrics.requests_processed, 0);
        }
    }

    #[tokio::test]
    async fn test_mixed_concurrent_operations() {
        let service = Arc::new(create_service());
        
        let handles: Vec<_> = (0..6)
            .map(|i| {
                let svc = Arc::clone(&service);
                tokio::spawn(async move {
                    if i % 3 == 0 {
                        svc.initialize().await
                    } else if i % 3 == 1 {
                        svc.health_check().await.map(|_| ())
                    } else {
                        svc.get_metrics().await;
                        Ok(())
                    }
                })
            })
            .collect();
        
        for h in handles {
            assert!(h.await.is_ok());
        }
    }
}
