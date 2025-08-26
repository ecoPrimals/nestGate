use std::collections::HashMap;
//
// Monitoring-related configuration including metrics, alerts, logging,
// and health checks.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Monitoring configuration (consolidates 15+ monitoring configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct MonitoringConfig {
    /// Metrics configuration
    pub metrics: MonitoringMetricsConfig,
    /// Alerting configuration
    pub alerts: AlertingConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Health checks configuration
    pub health_checks: MonitoringHealthConfig,
    /// Tracing configuration
    pub tracing: TracingConfig,
}

/// Monitoring metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Collection interval
    pub interval: Duration,
    /// Metrics format
    pub format: String,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Metric name
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator
    pub operator: String,
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel type (email, slack, webhook)
    pub channel_type: String,
    /// Channel configuration
    pub config: std::collections::HashMap<String, String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: String,
    /// Log output
    pub output: String,
    /// Log rotation
    pub rotation: LogRotationConfig,
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Maximum file size
    pub max_size: u64,
    /// Maximum number of files
    pub max_files: u32,
    /// Rotation interval
    pub interval: Duration,
}

/// Monitoring health configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringHealthConfig {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoints
    pub endpoints: Vec<String>,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    /// Tracing endpoint
    pub endpoint: String,
    /// Sample rate
    pub sample_rate: f64,
}


impl Default for MonitoringMetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            interval: Duration::from_secs(15),
            format: "prometheus".to_string(),
        }
    }
}


impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: "stdout".to_string(),
            rotation: LogRotationConfig::default(),
        }
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            max_size: 100 * 1024 * 1024, // 100MB
            max_files: 10,
            interval: Duration::from_secs(86400), // Daily
        }
    }
}

impl Default for MonitoringHealthConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            endpoints: vec!["/health".to_string()],
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:14268/api/traces".to_string(),
            sample_rate: 0.1,
        }
    }
} 