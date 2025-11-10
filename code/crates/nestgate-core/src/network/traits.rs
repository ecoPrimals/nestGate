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
pub struct NetworkTraitsConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_connections: usize,
    pub buffer_size: usize,
}

impl Default for NetworkTraitsConfig {
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
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Performance metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub requests_processed: u64,
    pub errors_encountered: u64,
    pub average_response_time: Duration,
    pub memory_usage_bytes: u64,
}

impl Default for Metrics {
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
pub struct DefaultService {
    config: NetworkTraitsConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
}

impl DefaultService {
    /// Create a new service instance
    pub fn new(config: NetworkTraitsConfig) -> Self {
        Self {
            config,
            metrics: Arc::new(tokio::sync::RwLock::new(Metrics::default())),
        }
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> Metrics {
        self.metrics.read().await.clone()
    }
}

impl Service for DefaultService {
    fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            tracing::info!("Initializing network service");
            Ok(())
        }
    }

    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send {
        async move { Ok(HealthStatus::Healthy) }
    }

    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            tracing::info!("Shutting down network service");
            Ok(())
        }
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

    #[test]
    fn test_config_default() {
        let config = NetworkTraitsConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
    }

    #[test]
    fn test_config_validation() {
        let config = NetworkTraitsConfig::default();
        assert!(validate_config(&config).is_ok());

        let mut bad_config = Config::default();
        bad_config.max_connections = 0;
        assert!(validate_config(&bad_config).is_err());
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_service();
        assert!(service.initialize().await.is_ok());
        assert_eq!(service.health_check().await.unwrap(), HealthStatus::Healthy);
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_metrics() {
        let service = create_service();
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
    }
}
