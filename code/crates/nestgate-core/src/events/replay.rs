//! Modern replay Module
//!
//! This module provides core functionality using modern Rust patterns
//! and zero-cost abstractions.

use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

// ==================== MODULE CONSTANTS ====================

/// Module version for compatibility tracking
pub use crate::constants::shared::MODULE_VERSION;

/// Default configuration values
/// Default configuration values from canonical constants
pub use crate::constants::network::{
    DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS, DEFAULT_TIMEOUT_MS,
};

// ==================== CORE TYPES ====================

/// Configuration for this module
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::EventsReplayConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::EventsReplayConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct EventsReplayConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_connections: usize,
    pub buffer_size: usize,
}

impl Default for EventsReplayConfig {
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
pub type Config = EventsReplayConfig;

/// Service interface re-exported from canonical source
/// See: `crate::traits::Service` for the unified implementation
pub use crate::traits::Service;

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

// ==================== IMPLEMENTATION ====================

/// Default implementation of the service
#[derive(Debug)]
pub struct DefaultService {
    #[allow(dead_code)] // Stored for future use
    config: EventsReplayConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
}

impl DefaultService {
    /// Create a new service instance
    pub fn new(config: EventsReplayConfig) -> Self {
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
    fn name(&self) -> &str {
        "replay"
    }

    async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing replay service");
        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }

    async fn start(&self) -> Result<()> {
        tracing::info!("Starting replay service");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping replay service");
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down replay service");
        Ok(())
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a default service instance
pub fn create_service() -> DefaultService {
    DefaultService::new(EventsReplayConfig::default())
}

/// Validate configuration
pub async fn validate_config(config: &EventsReplayConfig) -> crate::Result<()> {
    if config.max_connections == 0 {
        return Err(NestGateError::configuration_error(
            "events_replay",
            "max_connections must be greater than 0",
        ));
    }

    if config.buffer_size == 0 {
        return Err(NestGateError::configuration_error(
            "events_replay",
            "buffer_size must be greater than 0",
        ));
    }

    Ok(())
}

// ==================== TESTS ====================

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type EventsReplayConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using EventsReplayConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::DEFAULT_MAX_CONNECTIONS;

    #[test]
    fn test_config_default() {
        let config = EventsReplayConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
    }

    #[tokio::test]
    async fn test_config_validation() {
        let mut config = Config::default();
        assert!(validate_config(&config).await.is_ok());

        config.max_connections = 0;
        assert!(validate_config(&config).await.is_err());
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_service();

        assert!(service.initialize().await.is_ok());
        // health_check() now returns bool, not HealthStatus
        assert!(service.health_check().await.expect("Operation failed"));
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
