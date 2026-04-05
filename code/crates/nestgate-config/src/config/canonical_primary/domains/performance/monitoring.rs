// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **PERFORMANCE MONITORING CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;
// Result type not needed in this module

/// Performance monitoring configuration for observability and metrics.
///
/// Controls metrics collection, profiling, and alerting for performance monitoring.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `PerformanceMonitoring`
pub struct PerformanceMonitoringConfig {
    /// Whether performance monitoring is enabled.
    pub enabled: bool,
    /// Metrics collection configuration.
    pub metrics: MetricsConfig,
    /// Profiling configuration for detailed analysis.
    pub profiling: ProfilingConfig,
    /// Alerting configuration for performance thresholds.
    pub alerting: AlertingConfig,
}

/// Metrics collection configuration.
///
/// Defines which metrics to collect and how frequently.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Metrics
pub struct MetricsConfig {
    /// Interval between metric collections (default: 30 seconds).
    pub collection_interval: Duration,
    /// List of performance metrics to collect.
    pub metrics: Vec<PerformanceMetric>,
    /// How long to retain metrics data (default: 1 hour).
    pub retention: Duration,
}

/// Performance metric types to collect.
///
/// Defines the various performance metrics that can be tracked.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetric
pub enum PerformanceMetric {
    /// CPU usage percentage.
    CpuUsage,
    /// Memory usage in bytes.
    MemoryUsage,
    /// I/O operation latency.
    IoLatency,
    /// Network operation latency.
    NetworkLatency,
    /// Request throughput (requests/second).
    Throughput,
    /// Error rate percentage.
    ErrorRate,
}

/// Profiling configuration for detailed performance analysis.
///
/// Controls CPU and memory profiling for identifying performance bottlenecks.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Profiling
pub struct ProfilingConfig {
    /// Whether profiling is enabled (default: false).
    pub enabled: bool,
    /// Profiling mode to use.
    pub mode: ProfilingMode,
    /// Sample rate for profiling (0.0-1.0, default: 0.1 = 10%).
    pub sample_rate: f64,
    /// Duration of profiling session (default: 60 seconds).
    pub duration: Duration,
}

/// Profiling mode for performance analysis.
///
/// Determines what aspects of performance to profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Profilingmode
pub enum ProfilingMode {
    /// Profile CPU usage only (default).
    #[default]
    /// Cpu
    Cpu,
    /// Profile memory usage only.
    Memory,
    /// Profile both CPU and memory.
    Both,
    /// Custom profiling mode with specified name.
    Custom(String),
}

/// Alerting configuration for performance threshold violations.
///
/// Defines thresholds that trigger alerts when exceeded.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Alerting
pub struct AlertingConfig {
    /// CPU usage threshold for alerts (0.0-1.0, default: 0.8 = 80%).
    pub cpu_threshold: f64,
    /// Memory usage threshold for alerts (0.0-1.0, default: 0.8 = 80%).
    pub memory_threshold: f64,
    /// Latency threshold for alerts (default: 1 second).
    pub latency_threshold: Duration,
    /// Error rate threshold for alerts (0.0-1.0, default: 0.05 = 5%).
    pub error_rate_threshold: f64,
}

impl Default for MetricsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            collection_interval: Duration::from_secs(30),
            metrics: vec![PerformanceMetric::CpuUsage, PerformanceMetric::MemoryUsage],
            retention: Duration::from_secs(3600),
        }
    }
}

impl Default for ProfilingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            mode: ProfilingMode::default(),
            sample_rate: 0.1,
            duration: Duration::from_secs(60),
        }
    }
}

impl Default for AlertingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            latency_threshold: Duration::from_millis(1000),
            error_rate_threshold: 0.05,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn serde_roundtrip<T>(v: &T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let s = serde_json::to_string(v).expect("to_string");
        let _: T = serde_json::from_str(&s).expect("from_str");
    }

    #[test]
    fn performance_monitoring_default_serde() {
        let c = PerformanceMonitoringConfig::default();
        serde_roundtrip(&c);
    }

    #[test]
    fn performance_metric_variants() {
        for m in [
            PerformanceMetric::CpuUsage,
            PerformanceMetric::MemoryUsage,
            PerformanceMetric::IoLatency,
            PerformanceMetric::NetworkLatency,
            PerformanceMetric::Throughput,
            PerformanceMetric::ErrorRate,
        ] {
            serde_roundtrip(&m);
        }
    }

    #[test]
    fn profiling_mode_variants() {
        for m in [
            ProfilingMode::Cpu,
            ProfilingMode::Memory,
            ProfilingMode::Both,
            ProfilingMode::Custom("x".to_string()),
        ] {
            serde_roundtrip(&m);
        }
    }
}
