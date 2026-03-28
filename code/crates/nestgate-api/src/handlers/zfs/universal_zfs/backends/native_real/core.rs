// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Contains the main service structure, command execution, and basic utilities.
// Single responsibility: Service lifecycle and command abstraction.

//! Core module

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use std::process::Command;
use tracing::{info, warn};

use crate::handlers::zfs::universal_zfs::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    HealthCheck, HealthStatus, ServiceMetrics, ServiceStatus, UniversalZfsError, UniversalZfsResult,
};
// Core ZFS service implementation

/// Helper struct for ARC statistics
#[derive(Debug, Clone)]
/// Arcstatistics
pub struct ArcStatistics {
    /// Cache hit ratio as a percentage (0.0 to 1.0)
    pub hit_ratio: f64,
    /// Current cache size in bytes
    pub size_bytes: u64,
    /// Target cache size in bytes
    pub target_size_bytes: u64,
}
/// Helper struct for I/O statistics
#[derive(Debug, Clone)]
/// Iostatistics
pub struct IoStatistics {
    /// Read throughput in MB/s
    pub read_throughput: f64,
    /// Write throughput in MB/s
    pub write_throughput: f64,
    /// Read operations per second
    pub read_iops: f64,
    /// Write operations per second
    pub write_iops: f64,
    /// Average latency in milliseconds
    pub avg_latency: f64,
}
/// Circular buffer for tracking recent latencies (for percentile calculation)
///
/// Uses a fixed-size ring buffer to maintain recent latency samples.
/// This enables efficient percentile calculation without unbounded memory growth.
#[derive(Debug)]
struct LatencyTracker {
    samples: Vec<u64>,
    index: usize,
    capacity: usize,
}

impl LatencyTracker {
    /// Creates a new instance
    fn new(capacity: usize) -> Self {
        Self {
            samples: Vec::with_capacity(capacity),
            index: 0,
            capacity,
        }
    }

    /// Record
    fn record(&mut self, latency_ms: u64) {
        if self.samples.len() < self.capacity {
            self.samples.push(latency_ms);
        } else {
            self.samples[self.index] = latency_ms;
            self.index = (self.index + 1) % self.capacity;
        }
    }

    /// Percentile
    fn percentile(&self, p: f64) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let mut sorted = self.samples.clone();
        sorted.sort_unstable();

        let idx = ((sorted.len() as f64) * p).ceil() as usize;
        let idx = idx.min(sorted.len() - 1);
        sorted[idx] as f64
    }
}

/// Native ZFS service implementation using real zfs/zpool commands
#[derive(Debug)]
/// Service implementation for NativeZfs
pub struct NativeZfsService {
    /// Whether ZFS commands are available on this system
    zfs_available: bool,
    /// Service start time for uptime calculation
    #[allow(dead_code)]
    start_time: SystemTime,
    /// Request tracking for metrics
    request_counter: Arc<AtomicU64>,
    success_counter: Arc<AtomicU64>,
    total_response_time: Arc<AtomicU64>,
    active_connections: Arc<AtomicU64>,
    /// Latency tracker for percentile calculations (tracks last 1000 samples)
    latency_tracker: Arc<Mutex<LatencyTracker>>,
}
impl NativeZfsService {
    /// Create a new native ZFS service
    pub fn new() -> Self {
        let zfs_available = Self::check_zfs_availability();
        info!(
            "🔧 Native ZFS service initialized (available: {})",
            zfs_available
        );

        Self {
            zfs_available,
            start_time: SystemTime::now(),
            request_counter: Arc::new(AtomicU64::new(0)),
            success_counter: Arc::new(AtomicU64::new(0)),
            total_response_time: Arc::new(AtomicU64::new(0)),
            active_connections: Arc::new(AtomicU64::new(0)),
            latency_tracker: Arc::new(Mutex::new(LatencyTracker::new(1000))),
        }
    }

    /// Check if ZFS commands are available on the system
    fn check_zfs_availability() -> bool {
        Command::new("zfs").arg("version").output().is_ok()
    }

    /// Track a request for metrics
    pub fn track_request(&self, duration: Duration, success: bool) {
        let latency_ms = duration.as_millis() as u64;

        self.request_counter.fetch_add(1, Ordering::Relaxed);
        if success {
            self.success_counter.fetch_add(1, Ordering::Relaxed);
        }
        self.total_response_time
            .fetch_add(latency_ms, Ordering::Relaxed);

        // Record latency for percentile tracking
        if let Ok(mut tracker) = self.latency_tracker.lock() {
            tracker.record(latency_ms);
        }
    }

    /// Execute a ZFS command with error handling and metrics tracking
    pub async fn execute_zfs_command(
        &self,
        command: &str,
        args: &[&str],
    ) -> UniversalZfsResult<String> {
        let start_time = Instant::now();
        self.active_connections.fetch_add(1, Ordering::Relaxed);

        let result = tokio::process::Command::new(command)
            .args(args)
            .output()
            .await;

        let duration = start_time.elapsed();
        self.active_connections.fetch_sub(1, Ordering::Relaxed);

        match result {
            Ok(output) if output.status.success() => {
                self.track_request(duration, true);
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            Ok(output) => {
                self.track_request(duration, false);
                let error_msg = String::from_utf8_lossy(&output.stderr);
                warn!(
                    "⚠️ ZFS command failed: {} {:?} - {}",
                    command, args, error_msg
                );
                Err(UniversalZfsError::Internal {
                    message: format!("Command failed: zfs {} - {}", args.join(" "), error_msg),
                })
            }
            Err(e) => {
                self.track_request(duration, false);
                warn!(
                    "⚠️ Failed to execute ZFS command: {} {:?} - {}",
                    command, args, e
                );
                Err(UniversalZfsError::Internal {
                    message: format!("Failed to execute {command} {args:?}: {e}"),
                })
            }
        }
    }

    /// Get service metrics for monitoring
    #[must_use]
    pub fn get_service_metrics(&self) -> ServiceMetrics {
        let requests = self.request_counter.load(Ordering::Relaxed);
        let successes = self.success_counter.load(Ordering::Relaxed);
        let total_time = self.total_response_time.load(Ordering::Relaxed);

        // Calculate percentiles from latency tracker
        let (latency_p95, latency_p99) = self
            .latency_tracker
            .lock()
            .map(|tracker| (tracker.percentile(0.95), tracker.percentile(0.99)))
            .unwrap_or((0.0, 0.0));

        ServiceMetrics {
            service_name: "native_zfs".to_string(),
            timestamp: SystemTime::now(),
            requests_total: requests,
            requests_failed: requests - successes,
            error_rate: if requests > 0 {
                ((requests - successes) as f64) / (requests as f64) * 100.0
            } else {
                0.0
            },
            latency_avg: if requests > 0 {
                (total_time / requests) as f64
            } else {
                0.0
            },
            latency_p95,
            latency_p99,
            custom_metrics: HashMap::new(),
        }
    }
}

impl Default for NativeZfsService {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalZfsService for NativeZfsService {
    /// Service Name
    fn service_name(&self) -> &'static str {
        "native-zfs"
    }

    /// Service Version
    fn service_version(&self) -> &'static str {
        "1.0.0"
    }

    /// Checks if Available
    fn is_available(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move {
        self.zfs_available
    })}


    /// Health Check
    fn health_check(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HealthStatus>> + Send + '_>> {
        Box::pin(async move {
        let output = Command::new("modprobe").args(["zfs"]).output();
        let zfs_available = output.is_ok()
            && output
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    std::io::Error::other("Operation failed: self.base_url".to_string())
                })?
                .status
                .success();

        Ok(HealthStatus {
            service_name: self.service_name().to_string(),
            status: if zfs_available {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Degraded
            },
            checks: vec![HealthCheck {
                name: "zfs_availability".to_string(),
                passed: zfs_available,
                message: Some(if zfs_available {
                    "ZFS modules loaded successfully".to_string()
                } else {
                    "ZFS modules not available".to_string()
                }),
            }],
            last_check: SystemTime::now(),
            metadata: HashMap::new(),
        })
    })}


    /// Gets Metrics
    fn get_metrics(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<ServiceMetrics>> + Send + '_>> {
        Box::pin(async move {
        Ok(self.get_service_metrics())
    })}


    /// Shutdown
    fn shutdown(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        info!("🔄 Shutting down Native ZFS service");
        Ok(())
    })}


    // Forward all other methods to their respective modules
    fn list_pools(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<crate::handlers::zfs::universal_zfs_types::PoolInfo>>> + Send + '_>> {
        Box::pin(async move {
        super::pool_operations::list_pools(self).await
    })}


    // get_pool_info is not part of the trait - removed

    /// Creates  Pool
    fn create_pool(
        &self,
        config: &crate::handlers::zfs::universal_zfs_types::PoolConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<crate::handlers::zfs::universal_zfs_types::PoolInfo>> + Send + '_>> {
        Box::pin(async move {
        super::pool_operations::create_pool(self, config).await
    })}


    /// Destroy Pool
    fn destroy_pool(&self, name: &str) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        super::pool_operations::destroy_pool(self, name, false).await
    })}


    /// Gets Pool
    fn get_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<crate::handlers::zfs::universal_zfs_types::PoolInfo>>> + Send + '_>> {
        Box::pin(async move {
        super::pool_operations::get_pool(self, name).await
    })}


    /// Scrub Pool
    fn scrub_pool(&self, name: &str) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        super::pool_operations::scrub_pool(self, name).await
    })}


    /// Gets Pool Status
    fn get_pool_status(&self, name: &str) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async move {
        super::pool_operations::get_pool_status(self, name).await
    })}


    /// List Datasets
    fn list_datasets(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<crate::handlers::zfs::universal_zfs_types::DatasetInfo>>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::list_datasets(self).await
    })}


    // list_datasets_internal is not part of the trait - removed

    /// Creates  Dataset
    fn create_dataset(
        &self,
        config: &crate::handlers::zfs::universal_zfs_types::DatasetConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<crate::handlers::zfs::universal_zfs_types::DatasetInfo>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::create_dataset(self, config).await
    })}


    /// Destroy Dataset
    fn destroy_dataset(&self, name: &str) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::destroy_dataset(self, name, false).await
    })}


    /// Gets Dataset
    fn get_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<crate::handlers::zfs::universal_zfs_types::DatasetInfo>>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::get_dataset(self, name).await
    })}


    /// Gets Dataset Properties
    fn get_dataset_properties(
        &self,
        dataset_name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HashMap<String, String>>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::get_dataset_properties(self, dataset_name).await
    })}


    /// Sets Dataset Properties
    fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::set_dataset_properties(self, name, properties.clone()).await
    })}


    /// List Dataset Snapshots
    fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<crate::handlers::zfs::universal_zfs_types::SnapshotInfo>>> + Send + '_>> {
        Box::pin(async move {
        super::dataset_operations::list_dataset_snapshots(self, dataset_name).await
    })}


    /// List Snapshots
    fn list_snapshots(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<crate::handlers::zfs::universal_zfs_types::SnapshotInfo>>> + Send + '_>> {
        Box::pin(async move {
        super::snapshot_operations::list_snapshots(self, None).await
    })}


    /// Creates  Snapshot
    fn create_snapshot(
        &self,
        config: &crate::handlers::zfs::universal_zfs_types::SnapshotConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<crate::handlers::zfs::universal_zfs_types::SnapshotInfo>> + Send + '_>> {
        Box::pin(async move {
        super::snapshot_operations::create_snapshot(self, config).await
    })}


    /// Destroy Snapshot
    fn destroy_snapshot(&self, name: &str) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        super::snapshot_operations::destroy_snapshot(self, name, false).await
    })}


    /// Optimize
    fn optimize(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async move {
        super::configuration::optimize(self, "general".to_string())
    })}


    /// Gets Optimization Analytics
    fn get_optimization_analytics(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>> {
        Box::pin(async move {
        let analytics = super::configuration::get_optimization_analytics(self)?;
        Ok(serde_json::Value::Object(analytics.into_iter().collect()))
    })}


    /// Predict Tier
    fn predict_tier(&self, dataset_name: &str) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async move {
        Ok(super::configuration::predict_tier(self, dataset_name)?)
    })}


    /// Gets Configuration
    fn get_configuration(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>> {
        Box::pin(async move {
        let config = super::configuration::get_configuration(self)?;
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    })}


    /// Updates  Configuration
    fn update_configuration(&self, config: serde_json::Value) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move {
        // Convert serde_json::Value to HashMap if it's an object
        let config_map = match config {
            serde_json::Value::Object(map) => map
                .into_iter()
                .collect::<HashMap<String, serde_json::Value>>(),
            _ => HashMap::new(),
        };
        super::configuration::update_configuration(self, config_map)
    })}

}
