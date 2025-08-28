//
// Monitoring-specific configuration structures extracted from the monolithic NestGateCanonicalConfig.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Monitoring domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringDomainConfig {
    pub metrics_enabled: bool,
    pub health_checks_enabled: bool,
    pub tracing_enabled: bool,
    pub log_level: String,
    pub metrics_interval: Duration,
    pub health_check_interval: Duration,
}

impl Default for MonitoringDomainConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_checks_enabled: true,
            tracing_enabled: true,
            log_level: "info".to_string(),
            metrics_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
        }
    }
}
