//! **PERFORMANCE ANALYSIS TYPES**
//!
//! Types for performance trend analysis, I/O analysis, and cache performance.

use super::capacity::CapacityAnalysis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **PERFORMANCE ANALYSIS RESULT**
///
/// Complete result of performance analysis including trends and resource metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceanalysisresult
pub struct PerformanceAnalysisResult {
    /// Pool performance trends
    pub pool_trends: Vec<PoolTrend>,
    /// System resource metrics
    pub system_resources: SystemResourceMetrics,
    /// Capacity analysis
    pub capacity_analysis: CapacityAnalysis,
    /// I/O performance analysis
    pub io_performance: IOPerformanceAnalysis,
    /// Cache performance analysis
    pub cache_performance: CachePerformanceAnalysis,
}

/// **LATENCY PERCENTILES**
///
/// Latency measurements at different percentiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Latencypercentiles
pub struct LatencyPercentiles {
    /// 50th percentile (median) latency in milliseconds
    pub p50_ms: f64,
    /// 50th percentile (alias)
    pub p50: f64,
    /// 90th percentile latency in milliseconds
    pub p90_ms: f64,
    /// 95th percentile latency in milliseconds
    pub p95_ms: f64,
    /// 95th percentile (alias)
    pub p95: f64,
    /// 99th percentile latency in milliseconds
    pub p99_ms: f64,
    /// 99th percentile (alias)
    pub p99: f64,
    /// 99.9th percentile latency in milliseconds
    pub p999_ms: f64,
    /// 99.9th percentile (alias)
    pub p99_9: f64,
}

/// **THROUGHPUT ANALYSIS**
///
/// Analysis of system throughput characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Throughputanalysis
pub struct ThroughputAnalysis {
    /// Average throughput in operations per second
    pub avg_ops_per_sec: f64,
    /// Peak throughput in operations per second
    pub peak_ops_per_sec: f64,
    /// Minimum throughput in operations per second
    pub min_ops_per_sec: f64,
    /// Peak read throughput
    pub peak_read_throughput: f64,
    /// Peak write throughput
    pub peak_write_throughput: f64,
    /// Average read throughput
    pub average_read_throughput: f64,
    /// Average write throughput
    pub average_write_throughput: f64,
    /// Throughput standard deviation
    pub throughput_stddev: f64,
    /// Throughput patterns over time
    pub throughput_patterns: Vec<String>,
    /// Throughput trend over time
    pub trend: TrendDirection,
}

/// **SYSTEM RESOURCE METRICS**
///
/// System-level resource utilization metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemresourcemetrics
pub struct SystemResourceMetrics {
    /// CPU usage history over time
    pub cpu_usage_history: Vec<f64>,
    /// Memory usage history over time
    pub memory_usage_history: Vec<f64>,
    /// Network throughput history
    pub network_throughput_history: Vec<f64>,
    /// Disk usage history
    pub disk_usage_history: Vec<f64>,
    /// Network interface information
    pub network_interfaces: Vec<NetworkInterface>,
    /// System load averages [1min, 5min, 15min]
    pub load_average: [f64; 3],
}

/// **POOL TREND**
///
/// Performance trend data for a storage pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pooltrend
pub struct PoolTrend {
    /// Pool name
    pub pool_name: String,
    /// Performance trend direction
    pub trend: TrendDirection,
    /// Performance metrics over time
    pub metrics: Vec<PoolMetric>,
}

/// **POOL METRIC**
///
/// Individual performance metric for a pool at a specific time.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolmetric
pub struct PoolMetric {
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Read operations per second
    pub read_ops: f64,
    /// Write operations per second
    pub write_ops: f64,
    /// Read throughput in bytes/second
    pub read_throughput: u64,
    /// Write throughput in bytes/second
    pub write_throughput: u64,
}

/// **NETWORK INTERFACE**
///
/// Network interface information and statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkinterface
pub struct NetworkInterface {
    /// Interface name
    pub name: String,
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes transmitted
    pub bytes_transmitted: u64,
    /// Packets received
    pub packets_received: u64,
    /// Packets transmitted
    pub packets_transmitted: u64,
}

/// **PERFORMANCE TREND ANALYSIS**
///
/// Analysis of performance trends over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancetrendanalysis
pub struct PerformanceTrendAnalysis {
    /// CPU usage trend over time
    pub cpu_trend: TrendData,
    /// Memory usage trend over time
    pub memory_trend: TrendData,
    /// Disk I/O trend over time
    pub disk_io_trend: TrendData,
    /// Network I/O trend over time
    pub network_io_trend: TrendData,
    /// Overall trend assessment
    pub overall_trend: TrendDirection,
}

/// **I/O PERFORMANCE ANALYSIS**
///
/// Detailed analysis of I/O performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ioperformanceanalysis
pub struct IOPerformanceAnalysis {
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f64,
    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f64,
    /// Average read latency (alias)
    pub average_read_latency: f64,
    /// Average write latency (alias)
    pub average_write_latency: f64,
    /// Peak read latency
    pub peak_read_latency: f64,
    /// Peak write latency
    pub peak_write_latency: f64,
    /// Peak read IOPS
    pub peak_read_iops: u32,
    /// Peak write IOPS
    pub peak_write_iops: u32,
    /// I/O queue depth
    pub queue_depth: u32,
    /// I/O bottleneck analysis
    pub bottlenecks: Vec<String>,
    /// Average queue depth
    pub queue_depth_average: f64,
    /// I/O size distribution
    pub io_size_distribution: HashMap<String, f64>,
    /// Bottleneck analysis details
    pub bottleneck_analysis: Vec<String>,
    /// Optimization suggestions
    pub optimization_suggestions: Vec<String>,
    /// Latency percentile analysis
    pub latency_percentiles: LatencyPercentiles,
    /// Throughput analysis
    pub throughput_analysis: ThroughputAnalysis,
}

/// **CACHE PERFORMANCE ANALYSIS**
///
/// Analysis of cache performance and hit rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cacheperformanceanalysis
pub struct CachePerformanceAnalysis {
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    /// Cache miss rate (0.0 to 1.0)
    pub miss_rate: f64,
    /// ARC hit ratio
    pub arc_hit_ratio: f64,
    /// L2ARC hit ratio
    pub l2arc_hit_ratio: f64,
    /// Cache utilization percentage
    pub utilization_percent: f64,
    /// Cache size in bytes
    pub cache_size_bytes: u64,
    /// Cache performance recommendations
    pub recommendations: Vec<String>,
}

/// **TREND DATA**
///
/// Time series data for trend analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Trenddata
pub struct TrendData {
    /// Data points over time
    pub data_points: Vec<f64>,
    /// Trend direction
    pub direction: TrendDirection,
    /// Rate of change
    pub change_rate: f64,
}

/// **TREND DIRECTION**
///
/// Direction of performance trends.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Trenddirection
pub enum TrendDirection {
    /// Performance is improving
    Improving,
    /// Performance is stable
    Stable,
    /// Performance is degrading
    Degrading,
    /// Insufficient data to determine trend
    Unknown,
}
