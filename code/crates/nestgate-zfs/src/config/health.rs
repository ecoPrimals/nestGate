//
// Configuration for ZFS health monitoring, alerting, and failure thresholds.

use serde::{Deserialize, Serialize};

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    /// Check interval in seconds
    pub check_interval_seconds: u64,
    /// Failure threshold before marking unhealthy
    pub failure_threshold: u32,
    /// Recovery threshold before marking healthy
    pub recovery_threshold: u32,
    /// Enable alerting
    pub alerting_enabled: bool,
    /// Alert endpoints
    pub alert_endpoints: Vec<String>,
}
impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 60,
            failure_threshold: 3,
            recovery_threshold: 2,
            alerting_enabled: false,
            alert_endpoints: vec![],
        }
    }
}

impl HealthMonitoringConfig {
    /// Create production-optimized health monitoring configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 60,
            failure_threshold: 3,
            recovery_threshold: 1,
            alerting_enabled: true,
            alert_endpoints: vec![
                "email:admin@localhost".to_string(),
                "webhook:http://localhost:8080/alerts".to_string(),
            ],
        }
    }
}
