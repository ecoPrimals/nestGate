//! Analytics configuration types and defaults.

use serde::{Deserialize, Serialize};

/// Analytics configuration for enterprise operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Analytics
pub struct AnalyticsConfig {
    /// Data retention period in days
    pub retention_days: u32,
    /// Sampling interval in seconds
    pub sampling_interval_seconds: u64,
    /// Maximum data points to keep in memory
    pub max_memory_data_points: usize,
    /// Performance threshold alerts
    pub performance_thresholds: PerformanceThresholds,
    /// Enable predictive analytics
    pub enable_predictions: bool,
    /// Analytics storage path
    pub storage_path: std::path::PathBuf,
}

/// Performance threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancethresholds
pub struct PerformanceThresholds {
    /// CPU usage threshold (percentage)
    pub cpu_threshold_percent: f64,
    /// Memory usage threshold (percentage)
    pub memory_threshold_percent: f64,
    /// Disk usage threshold (percentage)
    pub disk_threshold_percent: f64,
    /// I/O latency threshold in milliseconds
    pub io_latency_threshold_ms: u64,
    /// Network bandwidth threshold in bytes per second
    pub network_threshold_bps: u64,
}

impl Default for AnalyticsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            retention_days: 30,
            sampling_interval_seconds: 60,
            max_memory_data_points: 10000,
            performance_thresholds: PerformanceThresholds::default(),
            enable_predictions: true,
            storage_path: std::path::PathBuf::from("/var/lib/nestgate/analytics"),
        }
    }
}

impl Default for PerformanceThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_threshold_percent: 80.0,
            memory_threshold_percent: 85.0,
            disk_threshold_percent: 90.0,
            io_latency_threshold_ms: 100,
            network_threshold_bps: 1_000_000_000, // 1 Gbps
        }
    }
} 