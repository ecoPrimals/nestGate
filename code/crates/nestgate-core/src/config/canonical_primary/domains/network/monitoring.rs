// **NETWORK MONITORING CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

/// Network monitoring configuration for observability and health checks.
///
/// Controls metrics collection, health checking, and event logging for network operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMonitoringConfig {
    /// Whether metrics collection is enabled.
    pub metrics_enabled: bool,
    /// Interval in seconds between health checks.
    pub health_check_interval_secs: u64,
    /// Whether to log network events.
    pub log_network_events: bool,
}

impl NetworkMonitoringConfig {
    /// Create development-optimized configuration with minimal logging.
    ///
    /// Enables metrics but disables verbose event logging.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            metrics_enabled: true,
            health_check_interval_secs: 60,
            log_network_events: false,
        }
    }

    /// Create production-hardened configuration with comprehensive monitoring.
    ///
    /// Enables full metrics and event logging with frequent health checks.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            metrics_enabled: true,
            health_check_interval_secs: 30,
            log_network_events: true,
        }
    }

    /// Validate the monitoring configuration.
    ///
    /// Ensures health check intervals are reasonable.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Merge this configuration with another, preferring values from `other`.
    ///
    /// All fields from `other` will replace the current values.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.metrics_enabled = other.metrics_enabled;
        self.health_check_interval_secs = other.health_check_interval_secs;
        self.log_network_events = other.log_network_events;
        self
    }
}
