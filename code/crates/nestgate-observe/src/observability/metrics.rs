// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::cast_precision_loss,
    reason = "Telemetry maps u64/usize OS counters to approximate f64 gauges"
)]

use std::collections::HashMap;
//
// Provides comprehensive system metrics collection and analysis.

use nestgate_types::error::Result;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

const DEFAULT_METRICS_HISTORY_SIZE: usize = 100;

/// Metric value types for custom metrics
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Gauge metric: a value that can go up or down
    Gauge(f64),
    /// Counter metric: a value that only increases
    Counter(u64),
    /// Histogram: distribution of values
    Histogram(Vec<f64>),
    /// Summary: aggregated statistics
    Summary {
        /// Total sum of all values
        sum: f64,
        /// Count of values
        count: u64,
    },
    /// String metric value
    String(String),
}

/// Type alias for custom metrics storage
pub type CustomMetricsMap = HashMap<String, MetricValue>;

/// Performance metrics structure
#[derive(Debug, Clone)]
/// Performancemetrics
pub struct PerformanceMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: SystemTime,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Available memory in bytes
    pub memory_available: u64,
    /// Disk I/O operations per second
    pub disk_iops: f64,
    /// Network bytes per second
    pub network_bytes_per_sec: f64,
    /// Custom application metrics
    pub custom_metrics: HashMap<String, f64>,
}
impl Default for PerformanceMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_available: 0,
            disk_iops: 0.0,
            network_bytes_per_sec: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

#[cfg(all(not(feature = "mock-metrics"), feature = "sysinfo"))]
const ESTIMATED_BASELINE_DISK_IOPS: f64 = 100.0;

#[cfg(all(not(feature = "mock-metrics"), feature = "sysinfo"))]
fn gather_performance_metrics_sysinfo(custom: &CustomMetricsMap) -> PerformanceMetrics {
    use sysinfo::{Disks, Networks, System};

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = f64::from(sys.global_cpu_info().cpu_usage());
    let memory_usage = sys.used_memory();
    let memory_available = sys.available_memory();

    let networks = Networks::new_with_refreshed_list();
    let network_bytes_per_sec: f64 = networks
        .values()
        .map(|data| (data.received() + data.transmitted()) as f64)
        .sum();

    let disks = Disks::new_with_refreshed_list();
    let disk_iops: f64 = disks.len() as f64 * ESTIMATED_BASELINE_DISK_IOPS; // Estimated baseline per-disk IOPS

    PerformanceMetrics {
        timestamp: SystemTime::now(),
        cpu_usage,
        memory_usage,
        memory_available,
        disk_iops,
        network_bytes_per_sec,
        custom_metrics: custom
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    match v {
                        MetricValue::Gauge(val) => *val,
                        MetricValue::Counter(val) => *val as f64,
                        MetricValue::Histogram(val) => val.iter().sum::<f64>() / (val.len() as f64),
                        MetricValue::Summary { sum, count: _ } => *sum,
                        MetricValue::String(_) => 0.0,
                    },
                )
            })
            .collect(),
    }
}

/// Metrics registry for collecting and storing performance data
#[derive(Debug)]
/// Metricsregistry
pub struct MetricsRegistry {
    metrics_history: Arc<RwLock<Vec<PerformanceMetrics>>>,
    max_history: usize,
    custom_metrics: Arc<RwLock<CustomMetricsMap>>,
}
impl Default for MetricsRegistry {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsRegistry {
    /// Create a new metrics registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            max_history: DEFAULT_METRICS_HISTORY_SIZE, // Default max history
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Test-only: insert arbitrary [`MetricValue`] entries for coverage of aggregation paths.
    #[cfg(test)]
    pub(crate) async fn insert_custom_metric_for_test(&self, name: &str, value: MetricValue) {
        let mut custom = self.custom_metrics.write().await;
        custom.insert(name.to_string(), value);
    }

    /// Test-only: lower the in-memory history cap to exercise trimming in `collect_system_metrics`.
    #[cfg(test)]
    pub(crate) fn set_max_history_for_test(&mut self, max_history: usize) {
        self.max_history = max_history;
    }

    /// Collect current system metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn collect_system_metrics(&self) -> Result<()> {
        let metrics = self.gather_system_metrics().await?;

        let mut history = self.metrics_history.write().await;
        history.push(metrics);

        // Keep only the last max_history entries
        if history.len() > self.max_history {
            history.remove(0);
        }

        Ok(())
    }

    /// Get current metrics snapshot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_current_metrics(&self) -> Result<PerformanceMetrics> {
        self.gather_system_metrics().await
    }

    /// Record a custom metric
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn record_custom_metric(&self, name: &str, value: f64) -> Result<()> {
        let mut custom = self.custom_metrics.write().await;
        custom.insert(name.to_string(), MetricValue::Gauge(value));

        tracing::debug!("Recorded custom metric: {} = {}", name, value);
        Ok(())
    }

    /// Get metrics history for a specific duration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_metrics_history(&self, duration: Duration) -> Result<Vec<PerformanceMetrics>> {
        let history = self.metrics_history.read().await;
        let cutoff_time = SystemTime::now() - duration;

        let filtered: Vec<PerformanceMetrics> = history
            .iter()
            .filter(|metrics| metrics.timestamp >= cutoff_time)
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Gather system metrics from the OS
    async fn gather_system_metrics(&self) -> Result<PerformanceMetrics> {
        let custom = self.custom_metrics.read().await;

        #[cfg(not(feature = "mock-metrics"))]
        {
            // ecoBin v3.0: Linux uses `/proc` + rustix; `sysinfo` is fallback (non-Linux or parse failure).
            #[cfg(target_os = "linux")]
            if let (
                Some(cpu_usage),
                Some(memory_usage),
                Some(memory_available),
                Some((rx, tx)),
                Some(disk_iops),
            ) = (
                nestgate_platform::linux_proc::globalcpu_usage_percent_from_stat(),
                nestgate_platform::linux_proc::used_memory_bytes(),
                nestgate_platform::linux_proc::available_memory_bytes(),
                nestgate_platform::linux_proc::network_rx_tx_bytes_sum(),
                nestgate_platform::linux_proc::diskstats_entry_count(),
            ) {
                let network_bytes_per_sec: f64 = (rx + tx) as f64;
                return Ok(PerformanceMetrics {
                    timestamp: SystemTime::now(),
                    cpu_usage,
                    memory_usage,
                    memory_available,
                    disk_iops,
                    network_bytes_per_sec,
                    custom_metrics: custom
                        .iter()
                        .map(|(k, v)| {
                            (
                                k.clone(),
                                match v {
                                    MetricValue::Gauge(val) => *val,
                                    MetricValue::Counter(val) => *val as f64,
                                    MetricValue::Histogram(val) => {
                                        val.iter().sum::<f64>() / (val.len() as f64)
                                    }
                                    MetricValue::Summary { sum, count: _ } => *sum,
                                    MetricValue::String(_) => 0.0,
                                },
                            )
                        })
                        .collect(),
                });
            }

            // Non-Linux, or Linux when `/proc` parsing failed: `sysinfo` fallback.
            #[cfg(feature = "sysinfo")]
            {
                Ok(gather_performance_metrics_sysinfo(&custom))
            }
            #[cfg(not(feature = "sysinfo"))]
            {
                Ok(PerformanceMetrics::default())
            }
        }

        #[cfg(feature = "mock-metrics")]
        {
            // MOCK METRICS: For testing only
            tracing::warn!("Using mock metrics - enable 'mock-metrics' feature is ON");
            Ok(PerformanceMetrics {
                timestamp: SystemTime::now(),
                cpu_usage: 25.0,                          // Mock CPU usage
                memory_usage: 1024 * 1024 * 512,          // Mock 512MB used
                memory_available: 1024 * 1024 * 1024 * 2, // Mock 2GB available
                disk_iops: 100.0,
                network_bytes_per_sec: 1024.0 * 1024.0, // Mock 1MB/s
                custom_metrics: custom
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            match v {
                                MetricValue::Gauge(val) => *val,
                                MetricValue::Counter(val) => *val as f64,
                                MetricValue::Histogram(val) => {
                                    val.iter().sum::<f64>() / (val.len() as f64)
                                }
                                MetricValue::Summary { sum, count: _ } => *sum,
                                MetricValue::String(_) => 0.0,
                            },
                        )
                    })
                    .collect(),
            })
        }
    }
}

/// Metrics collector interface
pub trait MetricsCollector {
    /// Collect metrics from this component
    fn collect_metrics(&self) -> HashMap<String, f64>;
    /// Get component name for metrics labeling
    fn component_name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCollector;

    impl MetricsCollector for TestCollector {
        fn collect_metrics(&self) -> HashMap<String, f64> {
            let mut m = HashMap::new();
            m.insert("k".to_string(), 1.0);
            m
        }

        fn component_name(&self) -> &str {
            "test_collector"
        }
    }

    #[test]
    fn metrics_collector_trait_smoke() {
        let c = TestCollector;
        assert_eq!(c.component_name(), "test_collector");
        assert_eq!(c.collect_metrics().get("k"), Some(&1.0));
    }

    #[tokio::test]
    async fn test_metrics_registry() -> nestgate_types::Result<()> {
        let registry = MetricsRegistry::new();

        // Should be able to collect metrics
        assert!(registry.collect_system_metrics().await.is_ok());

        // Should be able to get current metrics
        let metrics = registry.get_current_metrics().await?;
        assert!(metrics.cpu_usage >= 0.0);
        assert!(metrics.memory_usage > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_metrics() -> nestgate_types::Result<()> {
        let registry = MetricsRegistry::new();

        // Record custom metric
        let _tags: std::collections::HashMap<String, String> = HashMap::new();
        assert!(
            registry
                .record_custom_metric("test_metric", 42.0)
                .await
                .is_ok()
        );

        // Should appear in current metrics
        let metrics = registry.get_current_metrics().await?;
        assert_eq!(metrics.custom_metrics.get("test_metric"), Some(&42.0));
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_history() -> nestgate_types::Result<()> {
        let registry = MetricsRegistry::new();

        // Collect a few metrics
        registry.collect_system_metrics().await?;
        registry.collect_system_metrics().await?;

        // Should have history
        let history = registry
            .get_metrics_history(Duration::from_secs(60))
            .await?;
        assert!(!history.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_history_trims_when_exceeding_max() -> nestgate_types::Result<()> {
        let mut registry = MetricsRegistry::new();
        registry.set_max_history_for_test(2);
        for _ in 0..5 {
            registry.collect_system_metrics().await?;
        }
        let history = registry
            .get_metrics_history(Duration::from_secs(3600))
            .await?;
        assert_eq!(history.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_metric_value_variants_map_to_snapshot() -> nestgate_types::Result<()> {
        let registry = MetricsRegistry::new();
        registry
            .insert_custom_metric_for_test("g", MetricValue::Gauge(2.0))
            .await;
        registry
            .insert_custom_metric_for_test("c", MetricValue::Counter(10))
            .await;
        registry
            .insert_custom_metric_for_test("h", MetricValue::Histogram(vec![1.0, 3.0]))
            .await;
        registry
            .insert_custom_metric_for_test(
                "s",
                MetricValue::Summary {
                    sum: 10.0,
                    count: 2,
                },
            )
            .await;
        registry
            .insert_custom_metric_for_test("t", MetricValue::String("x".into()))
            .await;

        let m = registry.get_current_metrics().await?;
        assert_eq!(m.custom_metrics.get("g"), Some(&2.0));
        assert_eq!(m.custom_metrics.get("c"), Some(&10.0));
        assert_eq!(m.custom_metrics.get("h"), Some(&2.0));
        assert_eq!(m.custom_metrics.get("s"), Some(&10.0));
        assert_eq!(m.custom_metrics.get("t"), Some(&0.0));
        Ok(())
    }

    /// `sysinfo` fallback snapshot builder (non-mock); used when Linux `/proc` parsing is unavailable.
    #[cfg(all(not(feature = "mock-metrics"), feature = "sysinfo"))]
    #[test]
    fn gather_performance_metrics_sysinfo_returns_sane_snapshot() {
        let custom = CustomMetricsMap::new();
        let m = super::gather_performance_metrics_sysinfo(&custom);
        assert!(m.cpu_usage >= 0.0);
        assert!(m.cpu_usage <= 100.0);
        assert!(m.memory_usage > 0 || m.memory_available > 0 || m.disk_iops >= 0.0);
        assert!(m.network_bytes_per_sec >= 0.0);
    }

    /// Non-mock `get_current_metrics` must return finite gauges from the OS (Linux `/proc` or sysinfo fallback).
    #[cfg(not(feature = "mock-metrics"))]
    #[tokio::test]
    async fn non_mock_get_current_metrics_uses_real_collection_path() -> nestgate_types::Result<()>
    {
        let registry = MetricsRegistry::new();
        let m = registry.get_current_metrics().await?;
        assert!(m.cpu_usage.is_finite());
        assert!(m.disk_iops.is_finite());
        assert!(m.network_bytes_per_sec.is_finite());
        assert!(m.memory_usage <= u64::MAX);
        Ok(())
    }
}
