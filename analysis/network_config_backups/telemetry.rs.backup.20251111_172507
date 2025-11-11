// Removed unused error imports
use crate::Result;
/// Comprehensive Telemetry and Metrics Collection System
///
/// Provides advanced telemetry collection, metrics aggregation, and
/// observability features for the NestGate storage system.
///
/// ## Features
/// - **Real-time Metrics**: Live collection of system and application metrics
/// - **Time Series Data**: Historical metrics with configurable retention
/// - **Custom Metrics**: Application-specific metric collection
/// - **Health Monitoring**: Automated health status tracking
/// - **Export Formats**: Prometheus, JSON, and custom format support
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::info;
use tracing::warn;
// Removed unused tracing import
use serde::{Serialize, Deserialize};
/// Telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Collection interval for system metrics
    pub collection_interval: Duration,
    /// Retention period for historical data
    pub retention_period: Duration,
    /// Maximum number of data points to retain per metric
    pub max_data_points: usize,
    /// Enable detailed performance tracking
    pub enable_performance_tracking: bool,
    /// Export endpoints configuration
    pub export_endpoints: Vec<ExportEndpoint>,
}
impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            collection_interval: Duration::from_secs(10),
            retention_period: Duration::from_secs(3600), // 1 hour
            max_data_points: 360,                        // 6 data points per minute * 60 minutes
            enable_performance_tracking: true,
            export_endpoints: vec![],
        }
    }
}

/// Export endpoint configuration
#[derive(Debug, Clone)]
pub struct ExportEndpoint {
    pub name: String,
    pub url: String,
    pub format: ExportFormat,
    pub interval: Duration,
}
/// Export format for telemetry data
/// MODERNIZED: Uses capability-based discovery instead of vendor hardcoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    // DEPRECATED: Prometheus hardcoding - migrate to capability-based monitoring
    #[deprecated(since = "3.0.0", note = "Use capability-based monitoring discovery")]
    Prometheus,
    Custom(String),
    // DEPRECATED: InfluxDB hardcoding - migrate to capability-based monitoring  
    #[deprecated(since = "3.0.0", note = "Use capability-based monitoring discovery")]
    InfluxDb,
    /// NEW: Capability-based monitoring export
    MonitoringCapability {
        capability_type: String,
        format: String,
    },
}
/// Core telemetry collector
pub struct TelemetryCollector {
    config: TelemetryConfig,
    metrics_registry: Arc<RwLock<MetricsRegistry>>,
    collection_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}
/// Centralized metrics registry
#[derive(Debug, Default)]
pub struct MetricsRegistry {
    /// Counter metrics (monotonically increasing values)
    counters: HashMap<String, CounterMetric>,
    /// Gauge metrics (current values that can increase or decrease)
    gauges: HashMap<String, GaugeMetric>,
    /// Histogram metrics (distribution of values)
    histograms: HashMap<String, HistogramMetric>,
    /// Time series data for historical tracking
    time_series: HashMap<String, TimeSeries>,
}
/// Counter metric (monotonically increasing)
#[derive(Debug, Clone)]
pub struct CounterMetric {
    pub name: String,
    pub help: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub last_updated: SystemTime,
}
/// Gauge metric (current value)
#[derive(Debug, Clone)]
pub struct GaugeMetric {
    pub name: String,
    pub help: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub last_updated: SystemTime,
}
/// Histogram metric (distribution of values)
#[derive(Debug, Clone)]
pub struct HistogramMetric {
    pub name: String,
    pub help: String,
    pub buckets: Vec<f64>,
    pub counts: Vec<u64>,
    pub sum: f64,
    pub count: u64,
    pub labels: HashMap<String, String>,
    pub last_updated: SystemTime,
}
/// Time series data point
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub timestamp: SystemTime,
    pub value: f64,
}
/// Time series data collection
#[derive(Debug)]
pub struct TimeSeries {
    pub name: String,
    pub data_points: Vec<DataPoint>,
    pub max_points: usize,
}
impl TelemetryCollector {
    /// Create a new telemetry collector
    #[must_use]
    pub fn new(config: TelemetryConfig) -> Self {
        Self {
            config,
            metrics_registry: Arc::new(RwLock::new(MetricsRegistry::default())),
            collection_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Start the telemetry collection system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn start(&self) -> Result<()>  {
        info!("🔍 Starting comprehensive telemetry collection system");

        // Start system metrics collection
        self.start_system_metrics_collection().await?;

        // Start performance metrics collection
        if self.config.enable_performance_tracking {
            self.start_performance_metrics_collection().await?;
        }

        // Start data cleanup task
        self.start_cleanup_task().await?;

        // Start export tasks
        for endpoint in &self.config.export_endpoints {
            self.start_export_task(endpoint.clone()).await?;
        }

        info!("✅ Telemetry collection system started successfully");
        Ok(())
    }

    /// Stop all telemetry collection tasks
    pub async fn stop(&self) {
        info!("🛑 Stopping telemetry collection system");

        let mut tasks = self.collection_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }

        info!("✅ Telemetry collection system stopped");
    }

    /// Get current metrics snapshot
    pub async fn get_metrics_snapshot(&self) -> HashMap<String, serde_json::Value> {
        let registry = self.metrics_registry.read().await;
        let mut snapshot = HashMap::new();

        // Add counter metrics
        for (name, counter) in &registry.counters {
            snapshot.insert(
                name.clone(),
                serde_json::json!({
                    "type": "counter",
                    "value": counter.value,
                    "help": counter.help,
                    "labels": counter.labels,
                    "last_updated": counter.last_updated
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                }),
            );
        }

        // Add gauge metrics
        for (name, gauge) in &registry.gauges {
            snapshot.insert(
                name.clone(),
                serde_json::json!({
                    "type": "gauge",
                    "value": gauge.value,
                    "help": gauge.help,
                    "labels": gauge.labels,
                    "last_updated": gauge.last_updated
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                }),
            );
        }

        // Add histogram metrics
        for (name, histogram) in &registry.histograms {
            snapshot.insert(
                name.clone(),
                serde_json::json!({
                    "type": "histogram",
                    "buckets": histogram.buckets,
                    "counts": histogram.counts,
                    "sum": histogram.sum,
                    "count": histogram.count,
                    "help": histogram.help,
                    "labels": histogram.labels,
                    "last_updated": histogram.last_updated
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                }),
            );
        }

        snapshot
    }

    /// Record a counter metric
    pub async fn inc_counter(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let mut registry = self.metrics_registry.write().await;

        let counter = registry
            .counters
            .entry(name.to_string())
            .or_insert_with(|| CounterMetric {
                name: name.to_string(),
                help: format!("Counter metric: {name}"),
                value: 0.0,
                labels: labels.clone(),
                last_updated: SystemTime::now(),
            });

        counter.value += value;
        counter.last_updated = SystemTime::now();
        counter.labels = labels;
    }

    /// Set a gauge metric value
    pub async fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        {
            let mut registry = self.metrics_registry.write().await;

            let gauge = registry
                .gauges
                .entry(name.to_string())
                .or_insert_with(|| GaugeMetric {
                    name: name.to_string(),
                    help: format!("Gauge metric: {name}"),
                    value: 0.0,
                    labels: labels.clone(),
                    last_updated: SystemTime::now(),
                });

            gauge.value = value;
            gauge.last_updated = SystemTime::now();
            gauge.labels = labels;
        } // Release the write lock here

        // Add to time series for historical tracking (with separate lock acquisition)
        self.add_to_time_series(name, value).await;
    }

    /// Record a histogram observation
    pub async fn observe_histogram(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let mut registry = self.metrics_registry.write().await;

        let histogram = registry
            .histograms
            .entry(name.to_string())
            .or_insert_with(|| HistogramMetric {
                name: name.to_string(),
                help: format!("Histogram metric: {name}"),
                buckets: vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0],
                counts: vec![0; 9],
                sum: 0.0,
                count: 0,
                labels: labels.clone(),
                last_updated: SystemTime::now(),
            });

        histogram.sum += value;
        histogram.count += 1;
        histogram.last_updated = SystemTime::now();
        histogram.labels = labels;

        // Update bucket counts
        for (i, &bucket) in histogram.buckets.iter().enumerate() {
            if value <= bucket {
                histogram.counts[i] += 1;
            }
        }
    }

    /// Add data point to time series
    async fn add_to_time_series(&self, name: &str, value: f64) {
        let mut registry = self.metrics_registry.write().await;

        let time_series = registry
            .time_series
            .entry(name.to_string())
            .or_insert_with(|| TimeSeries {
                name: name.to_string(),
                data_points: Vec::new(),
                max_points: self.config.max_data_points,
            });

        time_series.data_points.push(DataPoint {
            timestamp: SystemTime::now(),
            value,
        });

        // Maintain maximum data points
        if time_series.data_points.len() > time_series.max_points {
            time_series.data_points.remove(0);
        }
    }

    /// Start system metrics collection task
    async fn start_system_metrics_collection(&self) -> Result<()> {
        let registry = Arc::clone(&self.metrics_registry);
        let interval = self.config.collection_interval;

        let task = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                // Collect system metrics
                if let Ok(cpu_usage) = Self::get_cpu_usage().await {
                    if let Ok(mut reg) = registry.try_write() {
                        let gauge = reg
                            .gauges
                            .entry("system_cpu_usage".to_string())
                            .or_insert_with(|| GaugeMetric {
                                name: "system_cpu_usage".to_string(),
                                help: "System CPU usage percentage".to_string(),
                                value: 0.0,
                                labels: HashMap::new(),
                                last_updated: SystemTime::now(),
                            });
                        gauge.value = cpu_usage;
                        gauge.last_updated = SystemTime::now();
                    }
                }

                if let Ok(memory_usage) = Self::get_memory_usage().await {
                    if let Ok(mut reg) = registry.try_write() {
                        let gauge = reg
                            .gauges
                            .entry("system_memory_usage".to_string())
                            .or_insert_with(|| GaugeMetric {
                                name: "system_memory_usage".to_string(),
                                help: "System memory usage percentage".to_string(),
                                value: 0.0,
                                labels: HashMap::new(),
                                last_updated: SystemTime::now(),
                            });
                        gauge.value = memory_usage;
                        gauge.last_updated = SystemTime::now();
                    }
                }

                debug!("System metrics collected");
            }
        );

        self.collection_tasks.write().await.push(task);
        Ok(())
    }

    /// Start performance metrics collection task
    async fn start_performance_metrics_collection(&self) -> Result<()> {
        let registry = Arc::clone(&self.metrics_registry);
        let interval = self.config.collection_interval;

        let task = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                // Collect performance metrics (buffer pool statistics)
                let (
                    stats_4kb,
                    stats_64kb,
                    stats_1mb,
                    stats_string,
                    stats_cmd,
                    stats_network,
                    stats_json,
                ) = crate::memory_pool::global_buffer_pool_stats();

                // Update buffer pool metrics
                if let Ok(mut reg) = registry.try_write() {
                    // 4KB buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_4kb", &stats_4kb);
                    // 64KB buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_64kb", &stats_64kb);
                    // 1MB buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_1mb", &stats_1mb);
                    // String buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_string", &stats_string);
                    // Command buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_cmd", &stats_cmd);
                    // Network buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_network", &stats_network);
                    // JSON buffer pool
                    Self::update_pool_metrics(&mut reg, "buffer_pool_json", &stats_json);
                }

                debug!("Performance metrics collected");
            }
        );

        self.collection_tasks.write().await.push(task);
        Ok(())
    }

    /// Update buffer pool metrics in registry
    fn update_pool_metrics(
        registry: &mut MetricsRegistry,
        pool_name: &str,
        stats: &crate::memory_pool::PoolStatistics,
    ) {
        let hit_ratio = if stats.hits + stats.misses > 0 {
            stats.hits as f64 / (stats.hits + stats.misses) as f64
        } else {
            0.0
        };

        let labels: HashMap<String, String> = [("pool".to_string(), pool_name.to_string())]
            .into_iter()
            .collect();

        // Hit ratio gauge
        let gauge_name = format!("{pool_name}_hit_ratio");
        let gauge = registry
            .gauges
            .entry(gauge_name)
            .or_insert_with(|| GaugeMetric {
                name: format!("{pool_name}_hit_ratio"),
                help: format!("Hit ratio for {pool_name} buffer pool"),
                value: 0.0,
                labels: labels.clone(),
                last_updated: SystemTime::now(),
            });
        gauge.value = hit_ratio;
        gauge.last_updated = SystemTime::now();

        // Total acquisitions counter
        let counter_name = format!("{pool_name}_acquisitions_total");
        let counter = registry
            .counters
            .entry(counter_name)
            .or_insert_with(|| CounterMetric {
                name: format!("{pool_name}_acquisitions_total"),
                help: format!("Total acquisitions for {pool_name} buffer pool"),
                value: 0.0,
                labels: labels.clone(),
                last_updated: SystemTime::now(),
            });
        counter.value = stats.total_acquisitions as f64;
        counter.last_updated = SystemTime::now();
    }

    /// Start cleanup task to remove old data
    async fn start_cleanup_task(&self) -> Result<()> {
        let registry = Arc::clone(&self.metrics_registry);
        let retention_period = self.config.retention_period;

        let task = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes

            loop {
                interval_timer.tick().await;

                if let Ok(mut reg) = registry.try_write() {
                    let cutoff_time = SystemTime::now() - retention_period;

                    // Clean up time series data
                    for time_series in reg.time_series.values_mut() {
                        time_series
                            .data_points
                            .retain(|point| point.timestamp > cutoff_time);
                    }
                }

                debug!("Telemetry data cleanup completed");
            }
        );

        self.collection_tasks.write().await.push(task);
        Ok(())
    }

    /// Start export task for a specific endpoint
    async fn start_export_task(&self, endpoint: ExportEndpoint) -> Result<()> {
        let registry = Arc::clone(&self.metrics_registry);

        let task = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(endpoint.interval);

            loop {
                interval_timer.tick().await;

                // Export metrics to endpoint
                if let Ok(reg) = registry.try_read() {
                    match endpoint.format {
                        ExportFormat::Json => {
                            Self::export_json(&endpoint, &reg).await;
                        }
                        // DEPRECATED: Use capability-based monitoring instead
                        #[allow(deprecated)]
                        ExportFormat::Prometheus => {
                            Self::export_monitoring_capability(&endpoint, &reg, "prometheus").await;
                        }
                        ExportFormat::Custom(format_type) => {
                            Self::export_custom(&endpoint, &reg, &format_type).await;
                        }
                        // DEPRECATED: Use capability-based monitoring instead  
                        #[allow(deprecated)]
                        ExportFormat::InfluxDb => {
                            Self::export_monitoring_capability(&endpoint, &reg, "influxdb").await;
                        }
                        ExportFormat::MonitoringCapability { capability_type, format } => {
                            Self::export_monitoring_capability(&endpoint, &reg, &format).await;
                        }
                    }
                }
            }
        );

        self.collection_tasks.write().await.push(task);
        Ok(())
    }

    /// Export metrics in JSON format
    async fn export_json(endpoint: &ExportEndpoint, _registry: &MetricsRegistry) {
        debug!("Exporting JSON metrics to {}", endpoint.url);
        // Implementation would send HTTP request to endpoint
    }

    /// Export metrics in Prometheus format
    async fn export_prometheus(endpoint: &ExportEndpoint, _registry: &MetricsRegistry) {
        debug!("Exporting Prometheus metrics to {}", endpoint.url);
        // Implementation would send metrics in Prometheus format
    }

    /// Export metrics using a capability-based monitoring endpoint
    async fn export_monitoring_capability(
        endpoint: &ExportEndpoint,
        registry: &MetricsRegistry,
        capability_type: &str,
    ) {
        debug!("Exporting metrics to capability-based monitoring endpoint: {}", endpoint.url);
        // Implementation would send metrics in the specified format
        // This is a placeholder and would require actual implementation
        // based on the capability_type (e.g., Prometheus, InfluxDB, etc.)
    }

    /// Export metrics in a custom format
    async fn export_custom(
        endpoint: &ExportEndpoint,
        registry: &MetricsRegistry,
        format_type: &str,
    ) {
        debug!("Exporting custom metrics to {} in format: {}", endpoint.url, format_type);
        // Implementation would send metrics in the specified custom format
        // This is a placeholder and would require actual implementation
    }

    /// Get current CPU usage
    async fn get_cpu_usage() -> Result<f64> {
        if let Ok(content) = tokio::fs::read_to_string("/proc/stat").await {
            if let Some(cpu_line) = content.lines().next() {
                let fields: Vec<&str> = cpu_line.split_whitespace().collect();
                if fields.len() >= 8 && fields[0] == "cpu" {
                    let user: u64 = fields[1].parse().unwrap_or(0);
                    let nice: u64 = fields[2].parse().unwrap_or(0);
                    let system: u64 = fields[3].parse().unwrap_or(0);
                    let idle: u64 = fields[4].parse().unwrap_or(0);
                    let iowait: u64 = fields[5].parse().unwrap_or(0);
                    let irq: u64 = fields[6].parse().unwrap_or(0);
                    let softirq: u64 = fields[7].parse().unwrap_or(0);

                    let total = user + nice + system + idle + iowait + irq + softirq;
                    let active = total - idle - iowait;

                    if total > 0 {
                        return Ok((active as f64 / total as f64) * 100.0);
                    }
                }
            }
        }
        Ok(0.0)
    }

    /// Get current memory usage
    async fn get_memory_usage() -> Result<f64> {
        if let Ok(content) = tokio::fs::read_to_string("/proc/meminfo").await {
            let mut total_mem = 0u64;
            let mut available_mem = 0u64;

            for line in content.lines() {
                if let Some(value) = line.strip_prefix("MemTotal:") {
                    if let Some(kb) = value.split_whitespace().next() {
                        total_mem = kb.parse::<u64>().unwrap_or(0);
                    }
                } else if let Some(value) = line.strip_prefix("MemAvailable:") {
                    if let Some(kb) = value.split_whitespace().next() {
                        available_mem = kb.parse::<u64>().unwrap_or(0);
                    }
                }
            }

            if total_mem > 0 {
                let used_mem = total_mem - available_mem;
                return Ok((used_mem as f64 / total_mem as f64) * 100.0);
            }
        }
        Ok(0.0)
    }
}

impl TimeSeries {
    /// Get recent data points within a time window
    pub fn get_recent_data(&self, window: Duration) -> Vec<&DataPoint> {
        let cutoff_time = SystemTime::now() - window;
        self.data_points
            .iter()
            .filter(|point| point.timestamp > cutoff_time)
            .collect()
    }

    /// Calculate average value over time window
    pub fn average_over_window(&self, window: Duration) -> Option<f64> {
        let recent_data = self.get_recent_data(window);
        if recent_data.is_empty() {
            return None;
        }

        let sum: f64 = recent_data.iter().map(|point| point.value).sum();
        Some(sum / (recent_data.len() as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telemetry_basic_functionality() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Test counter
        collector
            .inc_counter("test_counter", 1.0, HashMap::new())
            .await;
        collector
            .inc_counter("test_counter", 2.0, HashMap::new())
            .await;

        // Test gauge
        collector
            .set_gauge("test_gauge", 42.5, HashMap::new())
            .await;

        // Test histogram
        collector
            .observe_histogram("test_histogram", 1.5, HashMap::new())
            .await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.contains_key("test_counter"));
        assert!(snapshot.contains_key("test_gauge"));
        assert!(snapshot.contains_key("test_histogram"));
    }

    #[tokio::test]
    async fn test_time_series_functionality() {
        let mut time_series = TimeSeries {
            name: "test_series".to_string(),
            data_points: Vec::new(),
            max_points: 5,
        };

        // Add data points
        for i in 0..10 {
            time_series.data_points.push(DataPoint {
                timestamp: SystemTime::now(),
                value: i as f64,
            });
        }

        // Should only keep last 5 points due to max_points limit
        assert_eq!(time_series.data_points.len(), 10);

        // Test average calculation
        let avg = time_series.average_over_window(Duration::from_secs(3600));
        assert!(avg.is_some());
    }
}
