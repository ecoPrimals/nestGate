// Metrics Collection and Export
//! Monitoring and observability functionality.
// Comprehensive metrics system for monitoring NestGate performance,
//! provider health, storage operations, and system resources.

use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// System-wide metrics collector
pub struct MetricsCollector {
    /// System metrics (CPU, memory, etc.)
    system_metrics: Arc<RwLock<SystemMetrics>>,
    /// Provider-specific metrics
    provider_metrics: Arc<RwLock<HashMap<String, ProviderMetrics>>>,
    /// Storage backend metrics
    storage_metrics: Arc<RwLock<HashMap<String, StorageMetrics>>>,
    /// Performance metrics
    /// Metrics collection start time
    start_time: Instant,
    /// Metrics configuration
    config: MetricsConfig,
}
/// Configuration for metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// How often to collect system metrics
    pub collection_interval: Duration,
    /// How long to retain metrics in memory
    pub retention_period: Duration,
    /// Whether to export metrics to external systems
    pub export_enabled: bool,
    /// Export endpoints (Prometheus, etc.)
    pub export_endpoints: Vec<String>,
    /// Custom metric labels
    pub labels: HashMap<String, String>,
}
impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            collection_interval: Duration::from_secs(30),
            retention_period: Duration::from_secs(3600), // 1 hour
            export_enabled: true,
            export_endpoints: vec!["http://localhost:9090/metrics".to_string()],
            labels: HashMap::new(),
        }
    }
}

/// System-level metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage (0.0 - 100.0)
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Available memory in bytes
    pub memory_available: u64,
    /// Disk usage in bytes
    pub disk_usage: u64,
    /// Available disk space in bytes
    pub disk_available: u64,
    /// Network bytes received
    pub network_rx_bytes: u64,
    /// Network bytes transmitted
    pub network_tx_bytes: u64,
    /// Number of active connections
    pub active_connections: usize,
    /// System uptime
    pub uptime: Duration,
    /// Last updated timestamp
    pub timestamp: SystemTime,
}
impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_available: 0,
            disk_usage: 0,
            disk_available: 0,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            active_connections: 0,
            uptime: Duration::from_secs(0),
            timestamp: SystemTime::now(),
        }
    }
}

/// Provider-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    /// Provider name
    pub provider_name: String,
    /// Provider type (genome_data, model_data, etc.)
    pub provider_type: String,
    /// Total requests made to this provider
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Current rate limit status
    pub rate_limit_remaining: Option<u32>,
    /// Provider health status
    pub health_status: String,
    /// Last successful request timestamp
    pub last_success: Option<SystemTime>,
    /// Last failure timestamp
    pub last_failure: Option<SystemTime>,
    /// Error breakdown by type
    pub error_counts: HashMap<String, u64>,
}
impl ProviderMetrics {
    #[must_use]
    pub fn new(provider_name: String, provider_type: String) -> Self {
        Self {
            provider_name,
            provider_type,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            rate_limit_remaining: None,
            health_status: "unknown".to_string(),
            last_success: None,
            last_failure: None,
            error_counts: HashMap::new(),
        }
    }

    /// Record a successful request
    pub fn record_success(&mut self, response_time: Duration) {
        self.total_requests += 1;
        self.successful_requests += 1;
        self.last_success = Some(SystemTime::now());

        // Update average response time
        let response_time_ms = response_time.as_millis() as f64;
        self.avg_response_time_ms = (self.avg_response_time_ms * (self.total_requests - 1) as f64
            + response_time_ms)
            / self.total_requests as f64;
    }

    /// Record a failed request
    pub fn record_failure(&mut self, error_type: String) {
        self.total_requests += 1;
        self.failed_requests += 1;
        self.last_failure = Some(SystemTime::now());

        // Update error counts
        *self.error_counts.entry(error_type).or_insert(0) += 1;
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.successful_requests as f64 / self.total_requests as f64) * 100.0
    }
}

/// Storage backend metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Storage backend name
    pub backend_name: String,
    /// Backend type (filesystem, object_storage, etc.)
    pub backend_type: String,
    /// Total read operations
    pub read_operations: u64,
    /// Total write operations
    pub write_operations: u64,
    /// Total delete operations
    pub delete_operations: u64,
    /// Bytes read
    pub bytes_read: u64,
    /// Bytes written
    pub bytes_written: u64,
    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f64,
    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f64,
    /// Storage errors
    pub error_count: u64,
    /// Available storage space (if applicable)
    pub available_space: Option<u64>,
    /// Used storage space
    pub used_space: Option<u64>,
}
impl StorageMetrics {
    pub fn new(backend_name: String, backend_type: String) -> Self {
        Self {
            backend_name,
            backend_type,
            read_operations: 0,
            write_operations: 0,
            delete_operations: 0,
            bytes_read: 0,
            bytes_written: 0,
            avg_read_latency_ms: 0.0,
            avg_write_latency_ms: 0.0,
            error_count: 0,
            available_space: None,
            used_space: None,
        }
    }

    /// Record a read operation
    pub fn record_read(&mut self, bytes: u64, latency: Duration) {
        self.read_operations += 1;
        self.bytes_read += bytes;

        let latency_ms = latency.as_millis() as f64;
        self.avg_read_latency_ms = (self.avg_read_latency_ms * (self.read_operations - 1) as f64
            + latency_ms)
            / self.read_operations as f64;
    }

    /// Record a write operation
    pub fn record_write(&mut self, bytes: u64, latency: Duration) {
        self.write_operations += 1;
        self.bytes_written += bytes;

        let latency_ms = latency.as_millis() as f64;
        self.avg_write_latency_ms =
            (self.avg_write_latency_ms * (self.write_operations - 1) as f64 + latency_ms)
                / self.write_operations as f64;
    }

    /// Get throughput in MB/s for reads
    pub fn read_throughput_mbps(&self) -> f64 {
        if self.read_operations == 0 || self.avg_read_latency_ms == 0.0 {
            return 0.0;
        }
        let avg_bytes_per_read = self.bytes_read as f64 / self.read_operations as f64;
        let mb_per_read = avg_bytes_per_read / (1024.0 * 1024.0);
        let reads_per_second = 1000.0 / self.avg_read_latency_ms;
        mb_per_read * reads_per_second
    }
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Request latency percentiles
    pub latency_p50: f64,
    pub latency_p95: f64,
    pub latency_p99: f64,
    /// Throughput (requests per second)
    pub throughput_rps: f64,
    /// Concurrent requests
    pub concurrent_requests: u32,
    /// Queue depths
    pub queue_depth: u32,
    /// Cache hit rates
    pub cache_hit_rate: f64,
    /// Error rates
    pub error_rate: f64,
    /// Recent latency samples
    latency_samples: Vec<f64>,
}
impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            latency_p50: 0.0,
            latency_p95: 0.0,
            latency_p99: 0.0,
            throughput_rps: 0.0,
            concurrent_requests: 0,
            queue_depth: 0,
            cache_hit_rate: 0.0,
            error_rate: 0.0,
            latency_samples: Vec::with_capacity(1000),
        }
    }
}

impl PerformanceMetrics {
    /// Add a latency sample
    pub fn add_latency_sample(&mut self, latency_ms: f64) {
        self.latency_samples.push(latency_ms);

        // Keep only recent samples (last 1000)
        if self.latency_samples.len() > 1000 {
            self.latency_samples.remove(0);
        }

        // Recalculate percentiles
        self.calculate_percentiles();
    }

    /// Calculate latency percentiles
    fn calculate_percentiles(&mut self) {
        if self.latency_samples.is_empty() {
            return;
        }

        let mut sorted_samples = self.latency_samples.clone();
        sorted_samples.sort_by(|a, b| {
            // Handle NaN values by treating them as greater than any other value
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater)
        });

        let len = sorted_samples.len();
        self.latency_p50 = sorted_samples[len * 50 / 100];
        self.latency_p95 = sorted_samples[len * 95 / 100];
        self.latency_p99 = sorted_samples[len * 99 / 100];
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    #[must_use]
    pub fn new(config: MetricsConfig) -> Self {
        info!("📊 Initializing metrics collector");

        Self {
            system_metrics: Arc::new(RwLock::new(SystemMetrics::default())),
            provider_metrics: Arc::new(RwLock::new(HashMap::new())),
            storage_metrics: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
            config,
        }
    }

    /// Register a new provider for metrics tracking
    pub async fn register_provider(&self, provider_name: String, provider_type: String) {
        let mut providers = self.provider_metrics.write().await;
        providers.insert(
            provider_name.clone(),
            ProviderMetrics::new(provider_name.clone(), provider_type),
        );
        debug!("📝 Registered provider for metrics: {}", provider_name);
    }

    /// Register a new storage backend for metrics tracking
    pub async fn register_storage_backend(&self, backend_name: String, backend_type: String) {
        let mut backends = self.storage_metrics.write().await;
        backends.insert(
            backend_name.clone(),
            StorageMetrics::new(backend_name.clone(), backend_type),
        );
        debug!(
            "📝 Registered storage backend for metrics: {}",
            backend_name
        );
    }

    /// Record provider request success
    pub async fn record_provider_success(&self, provider_name: &str, response_time: Duration) {
        let mut providers = self.provider_metrics.write().await;
        if let Some(metrics) = providers.get_mut(provider_name) {
            metrics.record_success(response_time);

            // Also update performance metrics
            let mut perf = self.performance_metrics.write().await;
            perf.add_latency_sample(response_time.as_millis() as f64);
        }
    }

    /// Record provider request failure
    pub async fn record_provider_failure(&self, provider_name: &str, error_type: String) {
        let mut providers = self.provider_metrics.write().await;
        if let Some(metrics) = providers.get_mut(provider_name) {
            metrics.record_failure(error_type);
        }
    }

    /// Record storage operation
    pub async fn record_storage_read(&self, backend_name: &str, bytes: u64, latency: Duration) {
        let mut backends = self.storage_metrics.write().await;
        if let Some(metrics) = backends.get_mut(backend_name) {
            metrics.record_read(bytes, latency);
        }
    }

    /// Record storage write operation
    pub async fn record_storage_write(&self, backend_name: &str, bytes: u64, latency: Duration) {
        let mut backends = self.storage_metrics.write().await;
        if let Some(metrics) = backends.get_mut(backend_name) {
            metrics.record_write(bytes, latency);
        }
    }

    /// Update system metrics
    pub async fn update_system_metrics(&self) {
        let mut system = self.system_metrics.write().await;

        // In a real implementation, you'd collect actual system metrics
        // For now, we'll simulate some values
        system.cpu_usage = self.get_cpu_usage().unwrap_or(0.0);
        system.memory_usage = self.get_memory_usage().unwrap_or(0);
        system.memory_available = self.get_memory_available().unwrap_or(0);
        system.uptime = self.start_time.elapsed();
        system.timestamp = SystemTime::now();

        debug!(
            "📊 Updated system metrics - CPU: {:.1}%, Memory: {} MB",
            system.cpu_usage,
            system.memory_usage / 1024 / 1024
        );
    }

    /// Get current system metrics snapshot
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        self.system_metrics.read().await.clone()
    }

    /// Get provider metrics
    pub async fn get_provider_metrics(&self, provider_name: &str) -> Option<ProviderMetrics> {
        self.provider_metrics
            .read()
            .await
            .get(provider_name)
            .cloned()
    }

    /// Get all provider metrics
    pub async fn get_all_provider_metrics(&self) -> HashMap<String, ProviderMetrics> {
        self.provider_metrics.read().await.clone()
    }

    /// Get current metrics for alert evaluation
    pub async fn get_current_metrics(&self) -> crate::Result<HashMap<String, serde_json::Value>> {
        let mut metrics = HashMap::new();

        // Add system metrics (simplified for now)
        let _system_metrics = self.system_metrics.read().await;
        metrics.insert("cpu_usage".to_string(), serde_json::json!(50.0)); // Mock data
        metrics.insert("memory_usage".to_string(), serde_json::json!(60.0)); // Mock data
        metrics.insert("disk_usage".to_string(), serde_json::json!(70.0)); // Mock data

        // Add health metrics for common components
        metrics.insert("zfs_health".to_string(), serde_json::json!(1.0)); // Healthy
        metrics.insert("api_health".to_string(), serde_json::json!(1.0)); // Healthy
        metrics.insert("network_health".to_string(), serde_json::json!(1.0)); // Healthy

        Ok(metrics)
    }

    /// Get storage metrics
    pub async fn get_storage_metrics(&self, backend_name: &str) -> Option<StorageMetrics> {
        self.storage_metrics.read().await.get(backend_name).cloned()
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()
    }

    /// Start metrics collection background task
    pub fn start_collection_task(&self) -> tokio::task::JoinHandle<()> {
        let collector = Arc::new(self.clone());
        let interval = self.config.collection_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                // Update system metrics
                collector.update_system_metrics().await;

                // Clean up old metrics
                collector.cleanup_old_metrics().await;

                // Export metrics if enabled
                if collector.config.export_enabled {
                    if let Err(e) = collector.export_metrics().await {
                        warn!("Failed to export metrics: {}", e);
                    }
                }
            }
        })
    }

    /// Clean up old metrics data
    async fn cleanup_old_metrics(&self) {
        // In a real implementation, you'd remove metrics older than retention_period
        debug!("🧹 Cleaning up old metrics data");
    }

    /// Export metrics to external systems
    async fn export_metrics(&self) -> Result<()> {
        debug!("📤 Exporting metrics to external systems");

        // In a real implementation, you'd export to Prometheus, InfluxDB, etc.
        // For now, just log the metrics
        let system = self.get_system_metrics().await;
        let providers = self.get_all_provider_metrics().await;
        let performance = self.get_performance_metrics().await;

        info!(
            "📊 Metrics Export - System CPU: {:.1}%, Providers: {}, P95 Latency: {:.1}ms",
            system.cpu_usage,
            providers.len(),
            performance.latency_p95
        );

        Ok(())
    }

    // Helper methods for system metrics collection
    fn get_cpu_usage(&self) -> Result<f64> {
        // Simulate CPU usage - in real implementation, use sysinfo or similar
        Ok(rand::random::<f64>() * 100.0)
    }

    fn get_memory_usage(&self) -> Result<u64> {
        // Simulate memory usage
        Ok(1024 * 1024 * 1024) // 1GB
    }

    fn get_memory_available(&self) -> Result<u64> {
        // Simulate available memory
        Ok(4 * 1024 * 1024 * 1024) // 4GB
    }
}

// Make MetricsCollector cloneable for background tasks
impl Clone for MetricsCollector {
    fn clone(&self) -> Self {
        Self {
            system_metrics: Arc::clone(&self.system_metrics),
            provider_metrics: Arc::clone(&self.provider_metrics),
            storage_metrics: Arc::clone(&self.storage_metrics),
            start_time: self.start_time,
            config: self.config.clone(),
        }
    }
}

/// Metrics exporter for external systems
pub struct MetricsExporter {
    collector: Arc<MetricsCollector>,
    export_format: ExportFormat,
}
/// Supported export formats
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Prometheus,
    Json,
    InfluxDB,
}
impl MetricsExporter {
    pub fn new(collector: Arc<MetricsCollector>, format: ExportFormat) -> Self {
        Self {
            collector,
            export_format: format,
        }
    }

    /// Export metrics in the specified format
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn export(&self) -> Result<String>  {
        match self.export_format {
            ExportFormat::Prometheus => self.export_prometheus().await,
            ExportFormat::Json => self.export_json().await,
            ExportFormat::InfluxDB => self.export_influxdb().await,
        }
    }

    async fn export_prometheus(&self) -> Result<String> {
        let system = self.collector.get_system_metrics().await;
        let providers = self.collector.get_all_provider_metrics().await;
        let performance = self.collector.get_performance_metrics().await;

        let mut output = String::new();

        // System metrics
        output.push_str(" HELP nestgate_cpu_usage CPU usage percentage\n");
        output.push_str(" TYPE nestgate_cpu_usage gauge\n");
        output.push_str(&format!("nestgate_cpu_usage {system.cpu_usage}\n"));

        output.push_str(" HELP nestgate_memory_usage Memory usage in bytes\n");
        output.push_str(" TYPE nestgate_memory_usage gauge\n");
        output.push_str(&format!("nestgate_memory_usage {system.memory_usage}\n"));

        // Provider metrics
        for (name, metrics) in providers {
            output.push_str(" HELP nestgate_provider_requests_total Total requests to provider\n");
            output.push_str(" TYPE nestgate_provider_requests_total counter\n");
            output.push_str(&format!(
                "nestgate_provider_requests_total{{provider=\"{}\"} {}\n",
                name, metrics.total_requests
            ));

            output.push_str(&format!(
                "nestgate_provider_success_rate{{provider=\"{}\"} {}\n",
                name,
                metrics.success_rate()
            ));
        }

        // Performance metrics
        output.push_str(&format!(
            "nestgate_latency_p95 {}\n",
            performance.latency_p95
        ));
        output.push_str(&format!(
            "nestgate_throughput_rps {}\n",
            performance.throughput_rps
        ));

        Ok(output)
    }

    async fn export_json(&self) -> Result<String> {
        let system = self.collector.get_system_metrics().await;
        let providers = self.collector.get_all_provider_metrics().await;
        let performance = self.collector.get_performance_metrics().await;

        let export_data = serde_json::json!({
            "system": system,
            "providers": providers,
            "performance": performance,
            "timestamp": SystemTime::now()
        );

        serde_json::to_string_pretty(&export_data).map_err(|e| NestGateError::internal_error(
            location: Some("MetricsExporter::export_json".to_string())})
    }

    async fn export_influxdb(&self) -> Result<String> {
        let system = self.collector.get_system_metrics().await;
        let providers = self.collector.get_all_provider_metrics().await;

        let mut output = String::new();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| {
                // System time is before UNIX epoch (should never happen on modern systems)
                // Fall back to zero
                std::time::Duration::from_secs(0)
            })
            .as_nanos();

        // System metrics in InfluxDB line protocol
        output.push_str(&format!(
            "system_metrics cpu_usage={},memory_usage={} {}\n",
            system.cpu_usage, system.memory_usage, timestamp
        ));

        // Provider metrics
        for (name, metrics) in providers {
            output.push_str(&format!(
                "provider_metrics,provider={} total_requests={},success_rate={} {}\n",
                name,
                metrics.total_requests,
                metrics.success_rate(),
                timestamp
            ));
        }

        Ok(output)
    }
}