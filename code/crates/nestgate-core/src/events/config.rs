// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Modern config Module
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
/// use crate::network::config::EventsModuleConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::EventsModuleConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for EventsModule
pub struct EventsModuleConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
    /// Max Connections
    pub max_connections: usize,
    /// Size of buffer
    pub buffer_size: usize,
}

impl Default for EventsModuleConfig {
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
pub type Config = EventsModuleConfig;

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
    config: EventsModuleConfig,
    metrics: Arc<tokio::sync::RwLock<Metrics>>,
}

impl DefaultService {
    /// Create a new service instance
    pub fn new(config: EventsModuleConfig) -> Self {
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
    fn name(&self) -> &str {
        "config"
    }

    /// Initialize
    async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing config service");
        Ok(())
    }

    /// Start
    async fn start(&self) -> Result<()> {
        tracing::info!("Starting config service");
        Ok(())
    }

    /// Stop
    async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping config service");
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
pub fn create_service() -> DefaultService {
    DefaultService::new(EventsModuleConfig::default())
}

/// Validate configuration
pub async fn validate_config(config: &EventsModuleConfig) -> crate::Result<()> {
    if config.max_connections == 0 {
        return Err(NestGateError::configuration_error(
            "events_config",
            "max_connections must be greater than 0",
        ));
    }

    if config.buffer_size == 0 {
        return Err(NestGateError::configuration_error(
            "events_config",
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
/// Type alias for Eventsmoduleconfigcanonical
pub type EventsModuleConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using EventsModuleConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::DEFAULT_MAX_CONNECTIONS;

    #[test]
    fn test_config_default() {
        let config = EventsModuleConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
    }

    #[tokio::test]
    async fn test_config_validation() {
        let mut config = EventsModuleConfig::default();
        assert!(validate_config(&config).await.is_ok());

        config.max_connections = 0;
        assert!(validate_config(&config).await.is_err());
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_service();

        assert!(service.initialize().await.is_ok());
        // health_check() now returns bool, not HealthStatus
        assert!(service.health_check().await.expect("Configuration error"));
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
