// **CANONICAL MONITORING CONFIGURATION**
//! Monitoring functionality and utilities.
// This module provides the unified monitoring and metrics configuration
//! for the `NestGate` system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Canonical monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Metrics retention period
    pub retention_period: Duration,
    /// Enable capability-based monitoring export
    /// MODERNIZED: Uses capability discovery instead of hardcoded Prometheus
    pub monitoring_capability_enabled: bool,
    /// Monitoring capability endpoint discovered at runtime
    /// MODERNIZED: Dynamic discovery replaces hardcoded Prometheus endpoint  
    pub monitoring_capability_endpoint: String,
    /// Enable JSON export
    pub json_enabled: bool,
    /// JSON export path
    pub json_path: String,
    /// Alert configuration
    pub alerts: AlertConfig,
    /// Custom metrics configuration
    pub custom_metrics: HashMap<String, MetricConfig>,
}
/// Metrics export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Enable Prometheus export
    pub prometheus_enabled: bool,
    /// Prometheus endpoint
    pub prometheus_endpoint: String,
    /// Enable JSON export
    pub json_enabled: bool,
    /// JSON export path
    pub json_path: String,
}
/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertConfig {
    /// Enable alerts
    pub enabled: bool,
    /// Alert thresholds
    pub thresholds: HashMap<String, f64>,
    /// Alert notification endpoints
    pub notifications: Vec<String>,
}
/// Individual metric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricConfig {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Collection enabled
    pub enabled: bool,
    /// Custom labels
    pub labels: HashMap<String, String>,
}
/// Metric type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}
impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(24 * 60 * 60), // 24 hours
            monitoring_capability_enabled: true,
            monitoring_capability_endpoint: "/metrics".to_string(),
            // Removed deprecated prometheus_enabled and prometheus_endpoint
            // Use monitoring_capability_enabled and monitoring_capability_endpoint instead
            json_enabled: false,
            json_path: "/tmp/metrics.json".to_string(),
            alerts: AlertConfig::default(),
            custom_metrics: HashMap::new(),
        }
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            prometheus_enabled: true,
            prometheus_endpoint: "/metrics".to_string(),
            json_enabled: false,
            json_path: "/tmp/metrics.json".to_string(),
        }
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for UnifiedMonitoringConfig
pub type UnifiedMonitoringConfig = MonitoringConfig;
