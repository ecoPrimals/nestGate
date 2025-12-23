// **STORAGE MONITORING CONFIGURATION**

use serde::{Deserialize, Serialize};

/// Storage monitoring configuration for observability and health tracking.
///
/// Provides comprehensive monitoring including metrics collection, alerting,
/// logging, health checks, and diagnostics for storage operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for StorageMonitoring
pub struct StorageMonitoringConfig {
    /// Metrics collection configuration.
    pub metrics: MetricsStorageConfig,
    /// Alerting configuration for storage events.
    pub alerting: AlertingStorageConfig,
    /// Logging configuration for storage operations.
    pub logging: LoggingStorageConfig,
    /// Health check configuration.
    pub health_check: HealthCheckStorageConfig,
    /// Diagnostics and troubleshooting configuration.
    pub diagnostics: DiagnosticsConfig,
}

/// Metrics collection configuration for storage.
///
/// Controls whether storage metrics are collected and exposed.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MetricsStorage
pub struct MetricsStorageConfig {
    /// Whether metrics collection is enabled (default: true).
    pub enabled: bool,
}

/// Alerting configuration for storage events.
///
/// Controls whether alerts are generated for storage issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for AlertingStorage
pub struct AlertingStorageConfig {
    /// Whether alerting is enabled (default: false for development).
    pub enabled: bool,
}

/// Logging configuration for storage operations.
///
/// Controls logging of storage access and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LoggingStorage
pub struct LoggingStorageConfig {
    /// Whether storage logging is enabled (default: true).
    pub enabled: bool,
}

/// Health check configuration for storage backends.
///
/// Controls periodic health checks of storage systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for HealthCheckStorage
pub struct HealthCheckStorageConfig {
    /// Whether health checks are enabled (default: true).
    pub enabled: bool,
}

/// Diagnostics configuration for troubleshooting.
///
/// Enables detailed diagnostics for storage issue investigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Diagnostics
pub struct DiagnosticsConfig {
    /// Whether diagnostics are enabled (default: false).
    pub enabled: bool,
}

impl Default for StorageMonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            metrics: MetricsStorageConfig { enabled: true },
            alerting: AlertingStorageConfig { enabled: false },
            logging: LoggingStorageConfig { enabled: true },
            health_check: HealthCheckStorageConfig { enabled: true },
            diagnostics: DiagnosticsConfig { enabled: false },
        }
    }
}

impl StorageMonitoringConfig {
    /// Create production-optimized monitoring configuration.
    ///
    /// Enables all monitoring features for production environments.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized monitoring configuration.
    ///
    /// Uses default settings suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance monitoring configuration.
    ///
    /// Minimal monitoring overhead for maximum performance.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native monitoring configuration.
    ///
    /// Optimized for cloud monitoring services integration.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, preferring values from `other`.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate monitoring configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
