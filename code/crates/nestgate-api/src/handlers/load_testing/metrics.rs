//! Performance metrics collection.

use serde::{Deserialize, Serialize};

/// **LOAD TEST METRICS**
///
/// Comprehensive metrics collected during load testing execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Loadtestmetrics
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
/// Performancestats
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
/// Responsetimestats
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

impl Default for PerformanceStats {
    /// Returns the default instance
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
    /// Returns the default instance
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_test_metrics_default() {
        let metrics = LoadTestMetrics::default();
        assert_eq!(metrics.performance_stats.total_requests, 0);
        assert_eq!(metrics.response_time_stats.avg_ms, 0.0);
    }

    #[test]
    fn test_performance_stats_default() {
        let stats = PerformanceStats::default();
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.requests_per_second, 0.0);
    }

    #[test]
    fn test_response_time_stats_default() {
        let stats = ResponseTimeStats::default();
        assert_eq!(stats.min_ms, 0.0);
        assert_eq!(stats.max_ms, 0.0);
        assert_eq!(stats.avg_ms, 0.0);
        assert_eq!(stats.p50_ms, 0.0);
        assert_eq!(stats.p95_ms, 0.0);
        assert_eq!(stats.p99_ms, 0.0);
    }

    #[test]
    fn test_performance_stats_serialization() {
        let stats = PerformanceStats {
            total_requests: 10000,
            successful_requests: 9500,
            failed_requests: 500,
            requests_per_second: 166.67,
        };

        let json = serde_json::to_string(&stats);
        assert!(json.is_ok());
    }

    #[test]
    fn test_response_time_stats_serialization() {
        let stats = ResponseTimeStats {
            min_ms: 10.0,
            max_ms: 500.0,
            avg_ms: 100.0,
            p50_ms: 95.0,
            p95_ms: 250.0,
            p99_ms: 400.0,
        };

        let json = serde_json::to_string(&stats);
        assert!(json.is_ok());
    }

    #[test]
    fn test_load_test_metrics_complete() {
        let metrics = LoadTestMetrics {
            performance_stats: PerformanceStats {
                total_requests: 5000,
                successful_requests: 4900,
                failed_requests: 100,
                requests_per_second: 83.33,
            },
            response_time_stats: ResponseTimeStats {
                min_ms: 20.0,
                max_ms: 800.0,
                avg_ms: 120.0,
                p50_ms: 110.0,
                p95_ms: 300.0,
                p99_ms: 600.0,
            },
        };

        assert_eq!(metrics.performance_stats.total_requests, 5000);
        assert_eq!(metrics.response_time_stats.avg_ms, 120.0);

        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok());
    }
}
