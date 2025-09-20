//! Monitoring configuration types.

use serde::{Deserialize, Serialize};

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub health_check_interval_seconds: u64,
    pub alert_thresholds: AlertThresholds,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_check_interval_seconds: 30,
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_percent: 80.0,
            memory_percent: 85.0,
            disk_percent: 90.0,
        }
    }
} 