//! Performance metrics collection.

use serde::{Deserialize, Serialize};

/// **LOAD TEST METRICS**
///
/// Comprehensive metrics collected during load testing execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestMetrics {
    /// Overall performance statistics for the test
    pub performance_stats: PerformanceStats,
    /// Response time distribution and statistics
    pub response_time_stats: ResponseTimeStats,
}

/// **PERFORMANCE STATISTICS**
///
/// Key performance indicators collected during load testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    /// Total number of HTTP requests made during the test
    pub total_requests: u64,
    /// Number of requests that completed successfully
    pub successful_requests: u64,
    /// Number of requests that failed or timed out
    pub failed_requests: u64,
    /// Average number of requests processed per second
    pub requests_per_second: f64,
}

/// **RESPONSE TIME STATISTICS**
///
/// Statistical analysis of response times during load testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeStats {
    /// Minimum response time observed in milliseconds
    pub min_ms: f64,
    /// Maximum response time observed in milliseconds
    pub max_ms: f64,
    /// Average response time in milliseconds
    pub avg_ms: f64,
    /// 50th percentile response time in milliseconds
    pub p50_ms: f64,
    /// 95th percentile response time in milliseconds
    pub p95_ms: f64,
    /// 99th percentile response time in milliseconds
    pub p99_ms: f64,
}

impl Default for LoadTestMetrics {
    fn default() -> Self {
        Self {
            performance_stats: PerformanceStats::default(),
            response_time_stats: ResponseTimeStats::default(),
        }
    }
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            requests_per_second: 0.0,
        }
    }
}

impl Default for ResponseTimeStats {
    fn default() -> Self {
        Self {
            min_ms: 0.0,
            max_ms: 0.0,
            avg_ms: 0.0,
            p50_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
        }
    }
}
