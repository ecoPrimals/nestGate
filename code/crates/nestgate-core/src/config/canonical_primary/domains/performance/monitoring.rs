// **PERFORMANCE MONITORING CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;
// Result type not needed in this module

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMonitoringConfig {
    pub enabled: bool,
    pub metrics: MetricsConfig,
    pub profiling: ProfilingConfig,
    pub alerting: AlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub collection_interval: Duration,
    pub metrics: Vec<PerformanceMetric>,
    pub retention: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceMetric {
    CpuUsage,
    MemoryUsage,
    IoLatency,
    NetworkLatency,
    Throughput,
    ErrorRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub mode: ProfilingMode,
    pub sample_rate: f64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ProfilingMode {
    #[default]
    Cpu,
    Memory,
    Both,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub latency_threshold: Duration,
    pub error_rate_threshold: f64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            collection_interval: Duration::from_secs(30),
            metrics: vec![PerformanceMetric::CpuUsage, PerformanceMetric::MemoryUsage],
            retention: Duration::from_secs(3600),
        }
    }
}

impl Default for ProfilingConfig {
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
    fn default() -> Self {
        Self {
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            latency_threshold: Duration::from_millis(1000),
            error_rate_threshold: 0.05,
        }
    }
}
