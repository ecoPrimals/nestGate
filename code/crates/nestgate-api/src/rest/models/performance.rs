//! Performance and metrics models

use serde::{Deserialize, Serialize};

// Performance monitoring and metrics collection structures

/// Performance metrics for system monitoring and analysis
///
/// This structure provides comprehensive performance data including CPU usage,
/// memory consumption, throughput, and latency measurements for real-time
/// monitoring and historical analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage: f64,
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// Network throughput in megabits per second
    pub throughput_mbps: f64,
    /// Average response latency in milliseconds
    pub avg_latency_ms: f64,
    /// 95th percentile latency in milliseconds
    pub p95_latency_ms: f64,
    /// 99th percentile latency in milliseconds
    pub p99_latency_ms: f64,
    /// Input/Output operations per second
    pub iops: u64,
}
