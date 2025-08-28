use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module provides comprehensive performance monitoring and metrics
// collection for optimization tracking and analysis.

use crate::idiomatic_evolution::SafeResultExt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

// Type aliases to reduce complexity
type MetricsStorage = Arc<RwLock<HashMap<String, String>>>;
type MetricsReadGuard<'a> = RwLockReadGuard<'a, HashMap<String, String>>;
type MetricsWriteGuard<'a> = RwLockWriteGuard<'a, HashMap<String, String>>;

/// Performance monitoring system for tracking optimizations
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Metrics storage
    metrics: MetricsStorage,
    /// Global operation counter
    operation_counter: Arc<AtomicU64>,
    /// Start time for uptime tracking
    start_time: Instant,
}

impl PerformanceMonitor {
    /// **IDIOMATIC EVOLUTION**: Safe lock acquisition utilities
    /// Provides safe access to performance data with proper error handling
    pub fn safe_read_lock(&self) -> crate::Result<MetricsReadGuard> {
        self.metrics
            .read()
            .map_err(|_| crate::error::NestGateError::System {
                message: "Failed to acquire read lock on performance metrics".to_string(),
                resource: crate::error::core::SystemResource::Memory,
                utilization: Some(100.0),
                recovery: crate::error::core::RecoveryStrategy::Retry,
            })
    }

    /// Get all performance metrics safely
    pub fn get_all_metrics(&self) -> crate::Result<HashMap<String, String>> {
        let metrics = self.safe_read_lock()?;
        Ok(metrics.clone())
    }

    fn safe_write_lock(&self) -> crate::Result<MetricsWriteGuard> {
        self.metrics
            .write()
            .map_err(|e| crate::error::NestGateError::Internal {
                message: format!("Failed to acquire write lock on metrics: {e}"),
                location: Some("performance_monitor.rs".to_string()),
                location: Some("safe_write_lock".to_string()),
                is_bug: false,
            })
    }

    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            operation_counter: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }

    /// Record an operation duration
    pub async fn record_operation(&self, name: &str, duration: Duration) {
        self.operation_counter.fetch_add(1, Ordering::Relaxed);

        if let Ok(mut metrics) = self.metrics.write() {
            let metric_json = metrics.entry(name.to_string()).or_insert_with(|| {
                serde_json::to_string(&PerformanceMetric::new(name)).unwrap_or_default()
            });

            if let Ok(mut metric) = serde_json::from_str::<PerformanceMetric>(metric_json) {
                metric.record_duration(duration);
                if let Ok(updated_json) = serde_json::to_string(&metric) {
                    *metric_json = updated_json;
                }
            }
        }
    }

    /// Increment a counter metric
    pub async fn increment_counter(&self, name: &str) {
        if let Ok(mut metrics) = self.metrics.write() {
            let metric_json = metrics.entry(name.to_string()).or_insert_with(|| {
                serde_json::to_string(&PerformanceMetric::new(name)).unwrap_or_default()
            });

            if let Ok(mut metric) = serde_json::from_str::<PerformanceMetric>(metric_json) {
                metric.increment_count();
                if let Ok(updated_json) = serde_json::to_string(&metric) {
                    *metric_json = updated_json;
                }
            }
        }
    }

    /// Record a performance metric
    pub async fn record_metric(&self, name: &str, duration: Duration) -> crate::Result<()> {
        // ✅ IDIOMATIC EVOLUTION: Safe lock acquisition instead of unwrap()
        let mut metrics = self.safe_write_lock()?;

        // Get existing metric or create new one
        let metric_json = metrics.get(name).cloned().unwrap_or_else(|| {
            serde_json::to_string(&PerformanceMetric::new(name))
                .unwrap_or_default_with_log("metric_creation")
        });

        // Parse and update metric
        if let Ok(mut metric) = serde_json::from_str::<PerformanceMetric>(&metric_json) {
            metric.record_duration(duration);

            // Serialize back
            let updated_json =
                serde_json::to_string(&metric).unwrap_or_default_with_log("metric_serialization");

            metrics.insert(name.to_string(), updated_json);
        }

        // Increment operation counter
        self.operation_counter.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    /// Time an operation and record it
    pub fn time_operation<F, R>(&self, name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let _duration = start.elapsed();
        // Use a blocking approach for sync context
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.record_operation(name, duration))
        });
        result
    }

    /// Get all metrics
    pub async fn get_metrics(&self) -> HashMap<String, PerformanceMetric> {
        let mut result = HashMap::new();
        if let Ok(metrics) = self.metrics.read() {
            for (name, json_str) in metrics.iter() {
                if let Ok(metric) = serde_json::from_str::<PerformanceMetric>(json_str) {
                    result.insert(name.clone(), metric);
                }
            }
        }
        result
    }

    /// Get a specific metric
    pub async fn get_metric(&self, name: &str) -> Option<PerformanceMetric> {
        self.metrics
            .read()
            .ok()?
            .get(name)
            .and_then(|json_str| serde_json::from_str::<PerformanceMetric>(json_str).ok())
    }

    /// Get performance summary
    pub async fn get_summary(&self) -> PerformanceSummary {
        let metrics = self.get_metrics().await;
        let total_operations = self.operation_counter.load(Ordering::Relaxed);
        let uptime = self.start_time.elapsed();

        let mut total_time = Duration::ZERO;
        let mut slowest_operation = None;
        let mut fastest_operation = None;

        for metric in metrics.values() {
            total_time += metric.total_duration;

            if let Some(avg) = metric.average_duration() {
                match slowest_operation {
                    None => slowest_operation = Some((metric.name.clone(), avg)),
                    Some((_, current_slowest)) if avg > current_slowest => {
                        slowest_operation = Some((metric.name.clone(), avg));
                    }
                    _ => {}
                }

                match fastest_operation {
                    None => fastest_operation = Some((metric.name.clone(), avg)),
                    Some((_, current_fastest)) if avg < current_fastest => {
                        fastest_operation = Some((metric.name.clone(), avg));
                    }
                    _ => {}
                }
            }
        }

        PerformanceSummary {
            total_operations,
            uptime,
            total_time,
            metrics_count: metrics.len(),
            slowest_operation,
            fastest_operation,
            operations_per_second: if uptime.as_secs() > 0 {
                total_operations as f64 / uptime.as_secs_f64()
            } else {
                0.0
            },
        }
    }

    /// Reset all metrics
    pub async fn reset(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.clear();
        }
        self.operation_counter.store(0, Ordering::Relaxed);
    }

    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Clone for PerformanceMonitor {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
            operation_counter: Arc::clone(&self.operation_counter),
            start_time: self.start_time,
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    /// Metric name
    pub name: String,
    /// Number of recorded operations
    pub count: u64,
    /// Total duration of all operations
    pub total_duration: Duration,
    /// Minimum recorded duration
    pub min_duration: Option<Duration>,
    /// Maximum recorded duration
    pub max_duration: Option<Duration>,
    /// Last recorded duration
    pub last_duration: Option<Duration>,
}

impl PerformanceMetric {
    /// Create a new performance metric
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            count: 0,
            total_duration: Duration::ZERO,
            min_duration: None,
            max_duration: None,
            last_duration: None,
        }
    }

    /// Record a duration
    pub fn record_duration(&mut self, duration: Duration) {
        self.count += 1;
        self.total_duration += duration;
        self.last_duration = Some(duration);

        match self.min_duration {
            None => self.min_duration = Some(duration),
            Some(min) if duration < min => self.min_duration = Some(duration),
            _ => {}
        }

        match self.max_duration {
            None => self.max_duration = Some(duration),
            Some(max) if duration > max => self.max_duration = Some(duration),
            _ => {}
        }
    }

    /// Increment count without duration
    pub fn increment_count(&mut self) {
        self.count += 1;
    }

    /// Calculate average duration
    pub fn average_duration(&self) -> Option<Duration> {
        if self.count > 0 {
            Some(self.total_duration / self.count as u32)
        } else {
            None
        }
    }

    /// Get operations per second (if timing data available)
    pub fn operations_per_second(&self) -> Option<f64> {
        if self.total_duration.as_secs_f64() > 0.0 {
            Some(self.count as f64 / self.total_duration.as_secs_f64())
        } else {
            None
        }
    }
}

/// Performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Total operations recorded
    pub total_operations: u64,
    /// System uptime
    pub uptime: Duration,
    /// Total time spent in operations
    pub total_time: Duration,
    /// Number of different metrics
    pub metrics_count: usize,
    /// Slowest operation (name, average duration)
    pub slowest_operation: Option<(String, Duration)>,
    /// Fastest operation (name, average duration)
    pub fastest_operation: Option<(String, Duration)>,
    /// Operations per second
    pub operations_per_second: f64,
}

impl PerformanceSummary {
    /// Get efficiency percentage (0-100)
    pub fn efficiency_percentage(&self) -> f64 {
        if self.uptime.as_secs_f64() > 0.0 {
            let efficiency = self.total_time.as_secs_f64() / self.uptime.as_secs_f64();
            (efficiency * 100.0).min(100.0)
        } else {
            0.0
        }
    }

    /// Get performance assessment
    pub fn performance_assessment(&self) -> &'static str {
        match self.operations_per_second {
            ops if ops > 10000.0 => "Excellent",
            ops if ops > 1000.0 => "Very Good",
            ops if ops > 100.0 => "Good",
            ops if ops > 10.0 => "Fair",
            _ => "Needs Optimization",
        }
    }
}

// Global performance monitor instance
lazy_static::lazy_static! {
    /// Global performance monitor accessible throughout the application
    pub static ref GLOBAL_PERFORMANCE_MONITOR: PerformanceMonitor = PerformanceMonitor::new();
}

/// Convenience function for recording operations globally
pub fn record_operation(name: &str, duration: Duration) {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(GLOBAL_PERFORMANCE_MONITOR.record_operation(name, duration))
    });
}

/// Convenience function for incrementing counters globally
pub fn increment_counter(name: &str) {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(GLOBAL_PERFORMANCE_MONITOR.increment_counter(name))
    });
}

/// Convenience function for timing operations globally
pub fn time_operation<F, R>(name: &str, operation: F) -> R
where
    F: FnOnce() -> R,
{
    GLOBAL_PERFORMANCE_MONITOR.time_operation(name, operation)
}

/// Get global performance summary
pub fn global_performance_summary() -> PerformanceSummary {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(GLOBAL_PERFORMANCE_MONITOR.get_summary())
    })
}

/// Macro for easy performance timing
#[macro_export]
macro_rules! time_it {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        $crate::performance_monitor::record_operation($name, duration);
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    // Removed unused imports: anyhow::Result, std::thread

    #[tokio::test]
    async fn test_performance_monitoring() -> crate::Result<()> {
        let monitor = PerformanceMonitor::new();

        // Record a test operation
        let start_time = std::time::Instant::now();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let end_time = std::time::Instant::now();

        monitor
            .record_operation("sleep_test", start_time, end_time)
            .await?;

        // Get metrics
        let metric = monitor.get_metric("sleep_test").await?.ok_or_else(|| {
            crate::error::NestGateError::internal_error(
                "Expected metric not found".to_string(),
                "test_performance_monitoring".to_string(),
            )
        })?;

        assert!(metric.total_operations > 0);
        assert!(metric.average_duration.as_millis() >= 10);

        println!("✅ Performance monitoring working");
        Ok(())
    }

    #[test]
    fn test_performance_metric() {
        let mut metric = PerformanceMetric::new("test");

        metric.record_duration(Duration::from_millis(100));
        metric.record_duration(Duration::from_millis(200));

        assert_eq!(metric.count, 2);
        assert_eq!(metric.average_duration(), Some(Duration::from_millis(150)));
        assert_eq!(metric.min_duration, Some(Duration::from_millis(100)));
        assert_eq!(metric.max_duration, Some(Duration::from_millis(200)));
    }

    #[tokio::test]
    async fn test_time_operation() -> crate::Result<()> {
        let monitor = PerformanceMonitor::new();

        // Test recording operation
        monitor
            .record_operation("sleep_test", std::time::Duration::from_millis(100))
            .await;

        // Test getting metric
        let metric = monitor.get_metric("sleep_test").await?;
        assert!(metric.is_some());

        Ok(())
    }
}
