// **MONITORING HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringHandlerConfig {
    pub metrics: MetricsHandlerConfig,
    pub tracing: TracingHandlerConfig,
    pub health_check: HealthCheckHandlerConfig,
    pub alerting: AlertingHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingHandlerConfig {
    pub enabled: bool,
}

impl Default for MonitoringHandlerConfig {
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
    pub const fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
