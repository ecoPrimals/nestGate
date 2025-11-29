// **MONITORING HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MonitoringHandler
pub struct MonitoringHandlerConfig {
    /// Metrics
    pub metrics: MetricsHandlerConfig,
    /// Tracing
    pub tracing: TracingHandlerConfig,
    /// Health Check
    pub health_check: HealthCheckHandlerConfig,
    /// Alerting
    pub alerting: AlertingHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MetricsHandler
pub struct MetricsHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TracingHandler
pub struct TracingHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for HealthCheckHandler
pub struct HealthCheckHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for AlertingHandler
pub struct AlertingHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for MonitoringHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            metrics: MetricsHandlerConfig { enabled: true },
            tracing: TracingHandlerConfig { enabled: true },
            health_check: HealthCheckHandlerConfig { enabled: true },
            alerting: AlertingHandlerConfig { enabled: false },
        }
    }
}

impl MonitoringHandlerConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
