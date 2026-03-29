// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Modern error Module
//!
//! This module provides core functionality using modern Rust patterns
//! and zero-cost abstractions.

use nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;
use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

// ==================== MODULE CONSTANTS ====================

/// Module version for compatibility tracking
pub use nestgate_config::constants::shared::MODULE_VERSION;

/// Default configuration values
/// Default configuration values from canonical constants
pub use nestgate_config::constants::network::{
    DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS, DEFAULT_TIMEOUT_MS,
};

// ==================== CORE TYPES ====================

/// Configuration for this module
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::EventsErrorConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::EventsErrorConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `EventsError`
pub struct EventsErrorConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
    /// Max Connections
    pub max_connections: usize,
    /// Size of buffer
    pub buffer_size: usize,
}

#[expect(
    deprecated,
    reason = "Default impl for retained deprecated EventsErrorConfig (public migration shim)"
)]
impl Default for EventsErrorConfig {
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
pub type Config = CanonicalNetworkConfig;

/// Service interface re-exported from canonical source
/// See: `crate::traits::Service` for the unified implementation
pub use crate::traits::Service;

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

// ==================== IMPLEMENTATION ====================

/// Default implementation of the service
#[derive(Debug)]
/// Service implementation for Default
pub struct DefaultService {
    #[allow(dead_code)] // Stored for future use
    config: CanonicalNetworkConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
}

impl DefaultService {
    /// Create a new service instance
    #[must_use]
    pub fn new(config: CanonicalNetworkConfig) -> Self {
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
    /// Name
    fn name(&self) -> &'static str {
        "error"
    }

    /// Initialize
    async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing error service");
        Ok(())
    }

    /// Start
    async fn start(&self) -> Result<()> {
        tracing::info!("Starting error service");
        Ok(())
    }

    /// Stop
    async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping error service");
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }

    /// Shutdown
    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        self.stop()
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a default service instance
#[must_use]
pub fn create_service() -> DefaultService {
    DefaultService::new(CanonicalNetworkConfig::default())
}

/// Validate configuration.
///
/// # Errors
///
/// Returns when [`CanonicalNetworkConfig::validate`] fails.
pub fn validate_config(config: &CanonicalNetworkConfig) -> nestgate_types::Result<()> {
    config.validate()
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.api.enabled);
        assert!(config.api.max_connections > 0);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(validate_config(&config).is_ok());

        config.api.max_connections = 0;
        assert!(validate_config(&config).is_err());
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
