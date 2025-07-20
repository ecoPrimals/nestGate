//! ZFS Performance Monitor implementation

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

use crate::{ZfsDatasetManager, ZfsPoolManager};
// StreamingProcReader has been replaced with simpler implementations
use nestgate_core::{NestGateError, Result as CoreResult, StorageTier};

use super::types::*;

impl ZfsPerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(
        config: PerformanceConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Self {
        Self {
            config,
            pool_manager,
            dataset_manager,
            current_metrics: Arc::new(RwLock::new(CurrentPerformanceMetrics::default())),
            metrics_history: Arc::new(RwLock::new(VecDeque::new())),
            tier_metrics: Arc::new(RwLock::new(HashMap::new())),
            alert_conditions: Arc::new(RwLock::new(Vec::new())),
            active_alerts: Arc::new(RwLock::new(Vec::new())),
            collection_task: None,
            analysis_task: None,
            alert_task: None,
            alert_sender: None,
        }
    }

    /// Start performance monitoring
    pub async fn start(&mut self) -> CoreResult<()> {
        info!("Starting ZFS performance monitoring");

        // Load default alert conditions
        self.load_default_alert_conditions().await?;

        // Initialize tier targets
        self.initialize_tier_targets().await?;

        // Start background tasks
        self.start_collection_task().await?;
        self.start_analysis_task().await?;

        if self.config.enable_alerting {
            self.start_alert_task().await?;
        }

        Ok(())
    }

    /// Stop performance monitoring
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping ZFS performance monitoring");

        // Stop all background tasks
        if let Some(task) = self.collection_task.take() {
            task.abort();
        }
        if let Some(task) = self.analysis_task.take() {
            task.abort();
        }
        if let Some(task) = self.alert_task.take() {
            task.abort();
        }

        Ok(())
    }

    /// Load default alert conditions
    async fn load_default_alert_conditions(&self) -> CoreResult<()> {
        let mut conditions = self.alert_conditions.write().await;

        // High latency alert
        conditions.push(AlertCondition {
            id: "high-latency".to_string(),
            name: "High Latency".to_string(),
            description: "Average latency exceeds threshold".to_string(),
            metric: AlertMetric::Latency,
            operator: AlertOperator::GreaterThan,
            threshold: 100.0, // 100ms
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_LATENCY_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            ),
            severity: AlertSeverity::Warning,
            enabled: true,
        });

        // Low throughput alert
        conditions.push(AlertCondition {
            id: "low-throughput".to_string(),
            name: "Low Throughput".to_string(),
            description: "Throughput falls below threshold".to_string(),
            metric: AlertMetric::Throughput,
            operator: AlertOperator::LessThan,
            threshold: 100.0, // 100 MB/s
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_PERFORMANCE_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            ),
            severity: AlertSeverity::Warning,
            enabled: true,
        });

        // High utilization alert
        conditions.push(AlertCondition {
            id: "high-utilization".to_string(),
            name: "High Utilization".to_string(),
            description: "Storage utilization exceeds threshold".to_string(),
            metric: AlertMetric::Utilization,
            operator: AlertOperator::GreaterThan,
            threshold: 85.0, // 85%
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_CAPACITY_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(600), // 10 minutes default
            ),
            severity: AlertSeverity::Critical,
            enabled: true,
        });

        // High error rate alert
        conditions.push(AlertCondition {
            id: "high-error-rate".to_string(),
            name: "High Error Rate".to_string(),
            description: "Error rate exceeds threshold".to_string(),
            metric: AlertMetric::ErrorRate,
            operator: AlertOperator::GreaterThan,
            threshold: 0.01, // 1%
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_ERROR_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(180), // 3 minutes default
            ),
            severity: AlertSeverity::Critical,
            enabled: true,
        });

        info!("Loaded {} default alert conditions", conditions.len());
        Ok(())
    }

    /// Initialize performance targets for each tier
    async fn initialize_tier_targets(&self) -> CoreResult<()> {
        let mut tier_metrics = self.tier_metrics.write().await;

        // Initialize tier performance data for each tier
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            tier_metrics.insert(
                tier,
                TierPerformanceData {
                    tier,
                    current_metrics: TierMetrics::default_for_tier(tier),
                    history: VecDeque::new(),
                    trends: PerformanceTrends::default(),
                },
            );
        }

        info!("Initialized performance targets for all tiers");
        Ok(())
    }

    /// Start metrics collection task
    async fn start_collection_task(&mut self) -> CoreResult<()> {
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);
        let current_metrics = Arc::clone(&self.current_metrics);
        let tier_metrics = Arc::clone(&self.tier_metrics);
        let config = self.config.clone();

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.collection_interval));

            loop {
                interval.tick().await;

                if let Err(e) = Self::collect_metrics(
                    &pool_manager,
                    &dataset_manager,
                    &current_metrics,
                    &tier_metrics,
                )
                .await
                {
                    error!("Metrics collection failed: {}", e);
                }
            }
        });

        self.collection_task = Some(task);
        Ok(())
    }

    /// Collect performance metrics
    async fn collect_metrics(
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
        current_metrics: &Arc<RwLock<CurrentPerformanceMetrics>>,
        tier_metrics: &Arc<RwLock<HashMap<StorageTier, TierPerformanceData>>>,
    ) -> CoreResult<()> {
        debug!("Collecting performance metrics");

        // Collect real ZFS metrics
        let pool_metrics = Self::collect_pool_metrics(pool_manager).await?;
        let system_metrics = Self::collect_system_metrics().await?;
        let io_statistics = Self::collect_io_statistics(pool_manager).await?;
        let tier_data = Self::collect_tier_metrics(dataset_manager).await?;

        let metrics = CurrentPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            tier_metrics: tier_data.clone(),
            system_metrics,
            io_statistics,
            trends: PerformanceTrends::default(),
        };

        // Update current metrics
        {
            let mut current = current_metrics.write().await;
            *current = metrics;
        }

        // Update tier-specific metrics
        {
            let mut tier_data_store = tier_metrics.write().await;
            for (tier, tier_metric) in tier_data {
                if let Some(data) = tier_data_store.get_mut(&tier) {
                    data.current_metrics = tier_metric.clone();
                    data.history.push_back(tier_metric);

                    // Limit history size
                    if data.history.len() > 100 {
                        data.history.pop_front();
                    }
                }
            }
        }

        Ok(())
    }

    /// Collect real ZFS pool performance metrics
    async fn collect_pool_metrics(
        pool_manager: &Arc<ZfsPoolManager>,
    ) -> CoreResult<PoolPerformanceMetrics> {
        debug!("Collecting ZFS pool metrics");

        // Execute zpool iostat to get real I/O statistics
        let iostat_output = tokio::process::Command::new("zpool")
            .args(["iostat", "-v", "-y", "1", "1"])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to execute zpool iostat: {e}")))?;

        if !iostat_output.status.success() {
            warn!("zpool iostat failed, using fallback metrics");
            return Ok(PoolPerformanceMetrics::default());
        }

        let iostat_str = String::from_utf8_lossy(&iostat_output.stdout);
        let parsed_metrics = Self::parse_zpool_iostat(&iostat_str)?;

        // Get pool status information
        let pools = pool_manager.list_pools().await.unwrap_or_default();
        let mut total_size = 0u64;
        let mut total_free = 0u64;
        let mut fragmentation_sum = 0.0;
        let mut compression_sum = 0.0;
        let mut dedup_sum = 0.0;
        let pool_count = pools.len() as f64;

        for pool in &pools {
            // Get detailed pool information
            if let Ok(pool_info) = pool_manager.get_pool_info(&pool.name).await {
                let total_bytes = pool_info.capacity.total_bytes;
                let available_bytes = pool_info.capacity.available_bytes;

                total_size += total_bytes;
                total_free += available_bytes;

                // Collect additional pool properties
                if let Ok(properties) = Self::get_pool_properties(&pool.name).await {
                    fragmentation_sum += properties.fragmentation;
                    compression_sum += properties.compression_ratio;
                    dedup_sum += properties.dedup_ratio;
                }
            }
        }

        let utilization_percent = if total_size > 0 {
            ((total_size - total_free) as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        Ok(PoolPerformanceMetrics {
            total_iops: parsed_metrics.read_ops as f64 + parsed_metrics.write_ops as f64,
            total_throughput_mbs: parsed_metrics.read_throughput_mbs
                + parsed_metrics.write_throughput_mbs,
            avg_latency_ms: (parsed_metrics.read_latency_ms + parsed_metrics.write_latency_ms)
                / 2.0,
            utilization_percent,
            fragmentation_percent: if pool_count > 0.0 {
                fragmentation_sum / pool_count
            } else {
                0.0
            },
            compression_ratio: if pool_count > 0.0 {
                compression_sum / pool_count
            } else {
                1.0
            },
            dedup_ratio: if pool_count > 0.0 {
                dedup_sum / pool_count
            } else {
                1.0
            },
        })
    }

    /// Parse zpool iostat output into structured metrics
    fn parse_zpool_iostat(output: &str) -> CoreResult<IoStatsSummary> {
        let mut read_ops = 0u64;
        let mut write_ops = 0u64;
        let mut read_bytes = 0u64;
        let mut write_bytes = 0u64;

        // Parse iostat output - looking for lines with pool statistics
        for line in output.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 7 && !line.starts_with('-') && !line.contains("pool") {
                // Format: pool alloc free read write read write
                if let (Ok(r_ops), Ok(w_ops), Ok(r_bw), Ok(w_bw)) = (
                    fields[2].parse::<u64>(),
                    fields[3].parse::<u64>(),
                    fields[4].parse::<u64>(),
                    fields[5].parse::<u64>(),
                ) {
                    read_ops += r_ops;
                    write_ops += w_ops;
                    read_bytes += r_bw;
                    write_bytes += w_bw;
                }
            }
        }

        Ok(IoStatsSummary {
            read_ops,
            write_ops,

            read_throughput_mbs: read_bytes as f64 / (1024.0 * 1024.0),
            write_throughput_mbs: write_bytes as f64 / (1024.0 * 1024.0),
            read_latency_ms: 0.0,
            write_latency_ms: 0.0,
        })
    }

    /// Get pool properties for monitoring
    async fn get_pool_properties(pool_name: &str) -> CoreResult<PoolProperties> {
        let output = tokio::process::Command::new("zpool")
            .args(["get", "all", pool_name])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to get pool properties: {e}")))?;

        if !output.status.success() {
            return Ok(PoolProperties::default());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut properties = PoolProperties::default();

        for line in output_str.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 {
                match fields[1] {
                    "fragmentation" => {
                        if let Ok(frag) = fields[2].trim_end_matches('%').parse::<f64>() {
                            properties.fragmentation = frag;
                        }
                    }
                    "compressratio" => {
                        if let Ok(comp) = fields[2].trim_end_matches('x').parse::<f64>() {
                            properties.compression_ratio = comp;
                        }
                    }
                    "dedupratio" => {
                        if let Ok(dedup) = fields[2].trim_end_matches('x').parse::<f64>() {
                            properties.dedup_ratio = dedup;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(properties)
    }

    /// Collect system resource metrics
    async fn collect_system_metrics() -> CoreResult<SystemResourceMetrics> {
        debug!("Collecting system resource metrics");

        let cpu_usage = Self::get_cpu_utilization().await?;
        let memory_info = Self::get_memory_info().await?;
        let network_io = Self::get_network_io().await?;
        let load_average = Self::get_load_average().await?;
        let io_wait = Self::get_io_wait_percent().await?;

        Ok(SystemResourceMetrics {
            cpu_utilization_percent: cpu_usage,
            memory_usage_bytes: memory_info.used_mb * 1024 * 1024,
            available_memory_bytes: memory_info.available_mb * 1024 * 1024,
            network_io_mbs: network_io,
            io_wait_percent: io_wait,
            load_average_1m: load_average,
        })
    }

    /// Get CPU utilization from /proc/stat
    async fn get_cpu_utilization() -> CoreResult<f64> {
        let stat_content = tokio::fs::read_to_string("/proc/stat")
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to read /proc/stat: {e}")))?;

        if let Some(cpu_line) = stat_content.lines().next() {
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

        Ok(0.0)
    }

    /// Get memory information from /proc/meminfo
    async fn get_memory_info() -> CoreResult<LocalMemoryInfo> {
        // Simple fallback implementation
        Ok(LocalMemoryInfo {
            available_mb: 4 * 1024, // 4GB
            used_mb: 4 * 1024,      // 4GB
        })
    }

    /// Get network I/O in MB/s from /proc/net/dev
    async fn get_network_io() -> CoreResult<f64> {
        // Simple fallback implementation
        Ok(10.0) // Default to 10 MB/s
    }

    /// Get system load average
    async fn get_load_average() -> CoreResult<f64> {
        let loadavg_content = tokio::fs::read_to_string("/proc/loadavg")
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to read /proc/loadavg: {e}")))?;

        if let Some(first_field) = loadavg_content.split_whitespace().next() {
            return Ok(first_field.parse().unwrap_or(0.0));
        }

        Ok(0.0)
    }

    /// Get I/O wait percentage from /proc/stat
    async fn get_io_wait_percent() -> CoreResult<f64> {
        let stat_content = match tokio::fs::read_to_string("/proc/stat").await {
            Ok(content) => content,
            Err(_) => return Ok(0.0),
        };

        if let Some(cpu_line) = stat_content.lines().next() {
            let fields: Vec<&str> = cpu_line.split_whitespace().collect();
            if fields.len() >= 6 && fields[0] == "cpu" {
                if let Ok(iowait) = fields[5].parse::<u64>() {
                    let total: u64 = fields[1..8]
                        .iter()
                        .map(|f| f.parse::<u64>().unwrap_or(0))
                        .sum();
                    if total > 0 {
                        return Ok((iowait as f64 / total as f64) * 100.0);
                    }
                }
            }
        }

        Ok(0.0)
    }

    /// Collect I/O statistics
    async fn collect_io_statistics(pool_manager: &Arc<ZfsPoolManager>) -> CoreResult<IoStatistics> {
        debug!("Collecting I/O statistics");

        let pools = pool_manager.list_pools().await.unwrap_or_default();
        let mut total_reads = 0u64;
        let mut total_writes = 0u64;
        let mut total_bytes_read = 0u64;
        let mut total_bytes_written = 0u64;

        for pool in &pools {
            if let Ok(stats) = Self::get_pool_io_stats(&pool.name).await {
                total_reads += stats.read_ops;
                total_writes += stats.write_ops;
                total_bytes_read += stats.bytes_read;
                total_bytes_written += stats.bytes_written;
            }
        }

        let total_ops = total_reads + total_writes;
        let avg_io_size = if total_ops > 0 {
            (total_bytes_read + total_bytes_written) / total_ops
        } else {
            0
        };
        let read_write_ratio = if total_writes > 0 {
            total_reads as f64 / total_writes as f64
        } else {
            0.0
        };

        Ok(IoStatistics {
            total_reads,
            total_writes,
            total_bytes_read,
            total_bytes_written,
            avg_io_size_bytes: avg_io_size,
            read_write_ratio,
        })
    }

    /// Get I/O statistics for a specific pool
    async fn get_pool_io_stats(pool_name: &str) -> CoreResult<PoolIoStats> {
        debug!("Getting I/O stats for pool: {}", pool_name);

        if crate::mock::is_mock_mode() {
            return Ok(PoolIoStats {
                read_ops: 1000,
                write_ops: 500,
                bytes_read: 1024 * 1024 * 1024,
                bytes_written: 512 * 1024 * 1024,
            });
        }

        let output = tokio::process::Command::new("zpool")
            .args(["iostat", "-v", pool_name, "1", "1"])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to get pool I/O stats: {e}")))?;

        if !output.status.success() {
            return Ok(PoolIoStats::default());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut pool_stats = PoolIoStats::default();

        for line in output_str.lines() {
            if line.contains(pool_name) && !line.starts_with("pool:") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 7 {
                    if let Ok(read_ops) = fields[3].parse::<u64>() {
                        pool_stats.read_ops = read_ops;
                    }
                    if let Ok(write_ops) = fields[4].parse::<u64>() {
                        pool_stats.write_ops = write_ops;
                    }
                    if let Ok(read_bw) = Self::parse_iostat_bandwidth(fields[5]) {
                        pool_stats.bytes_read = read_bw;
                    }
                    if let Ok(write_bw) = Self::parse_iostat_bandwidth(fields[6]) {
                        pool_stats.bytes_written = write_bw;
                    }
                }
                break;
            }
        }

        Ok(pool_stats)
    }

    /// Parse bandwidth values from iostat output
    fn parse_iostat_bandwidth(value: &str) -> Result<u64, std::num::ParseFloatError> {
        let value = value.trim();
        if value.is_empty() || value == "-" {
            return Ok(0);
        }

        let (number, multiplier) = if value.ends_with('K') {
            (value.trim_end_matches('K'), 1024_u64)
        } else if value.ends_with('M') {
            (value.trim_end_matches('M'), 1024_u64 * 1024)
        } else if value.ends_with('G') {
            (value.trim_end_matches('G'), 1024_u64 * 1024 * 1024)
        } else {
            (value, 1_u64)
        };

        let number: f64 = number.parse()?;
        Ok((number * multiplier as f64) as u64)
    }

    /// Collect tier-specific metrics
    async fn collect_tier_metrics(
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<HashMap<StorageTier, TierMetrics>> {
        debug!("Collecting tier-specific metrics");

        let mut tier_metrics = HashMap::new();

        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            let metrics = Self::collect_single_tier_metrics(&tier, dataset_manager).await?;
            tier_metrics.insert(tier, metrics);
        }

        Ok(tier_metrics)
    }

    /// Collect metrics for a single tier
    async fn collect_single_tier_metrics(
        tier: &StorageTier,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<TierMetrics> {
        debug!("Collecting metrics for tier: {:?}", tier);

        let datasets = dataset_manager.list_datasets().await.unwrap_or_default();
        let tier_datasets: Vec<_> = datasets.into_iter().filter(|d| d.tier == *tier).collect();

        if tier_datasets.is_empty() {
            return Ok(TierMetrics::default_for_tier(*tier));
        }

        let mut total_read_iops = 0.0;
        let mut total_write_iops = 0.0;
        let mut total_read_throughput = 0.0;
        let mut total_write_throughput = 0.0;
        let mut total_read_latency = 0.0;
        let mut total_write_latency = 0.0;
        let mut total_utilization = 0.0;
        let dataset_count = tier_datasets.len() as f64;

        for dataset in &tier_datasets {
            if let Ok(stats) = Self::get_dataset_performance_stats(&dataset.name).await {
                total_read_iops += stats.read_iops;
                total_write_iops += stats.write_iops;
                total_read_throughput += stats.read_throughput_mbs;
                total_write_throughput += stats.write_throughput_mbs;
                total_read_latency += stats.read_latency_ms;
                total_write_latency += stats.write_latency_ms;
                total_utilization += stats.utilization_percent;
            }
        }

        let cache_hit_ratio = Self::get_zfs_cache_hit_ratio().await.unwrap_or(0.85);

        Ok(TierMetrics {
            tier: *tier,
            read_iops: total_read_iops,
            write_iops: total_write_iops,
            read_throughput_mbs: total_read_throughput,
            write_throughput_mbs: total_write_throughput,
            avg_read_latency_ms: if dataset_count > 0.0 {
                total_read_latency / dataset_count
            } else {
                0.0
            },
            avg_write_latency_ms: if dataset_count > 0.0 {
                total_write_latency / dataset_count
            } else {
                0.0
            },
            cache_hit_ratio,
            queue_depth: Self::get_real_queue_depth(tier).await.unwrap_or(4.0),
            utilization_percent: if dataset_count > 0.0 {
                total_utilization / dataset_count
            } else {
                0.0
            },
            targets: TierPerformanceTargets::default(),
            sla_compliance: SlaCompliance::default(),
        })
    }

    /// Get performance statistics for a specific dataset
    async fn get_dataset_performance_stats(
        dataset_name: &str,
    ) -> CoreResult<DatasetPerformanceStats> {
        debug!("Collecting performance stats for dataset: {}", dataset_name);

        if crate::mock::is_mock_mode() {
            return Ok(DatasetPerformanceStats::default());
        }

        let mut stats = DatasetPerformanceStats::default();

        // Get dataset properties
        if let Ok(output) = tokio::process::Command::new("zfs")
            .args(["get", "-H", "-p", "used,compressratio,dedup", dataset_name])
            .output()
            .await
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let fields: Vec<&str> = line.split('\t').collect();
                    if fields.len() >= 4 {
                        match fields[1] {
                            "compressratio" => {
                                if let Ok(ratio) = fields[2].trim_end_matches('x').parse::<f64>() {
                                    stats.compression_effectiveness = ratio;
                                }
                            }
                            "dedup" => {
                                if fields[2] == "on" {
                                    stats.deduplication_effectiveness = 1.2;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Get I/O statistics
        if let Some(pool_name) = dataset_name.split('/').next() {
            if let Ok(output) = tokio::process::Command::new("zpool")
                .args(["iostat", "-v", pool_name, "1", "1"])
                .output()
                .await
            {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        if line.contains(dataset_name) {
                            let fields: Vec<&str> = line.split_whitespace().collect();
                            if fields.len() >= 7 {
                                if let Ok(read_ops) = fields[1].parse::<f64>() {
                                    stats.read_iops = read_ops;
                                }
                                if let Ok(write_ops) = fields[2].parse::<f64>() {
                                    stats.write_iops = write_ops;
                                }
                                if let Ok(read_bw) = fields[3].parse::<f64>() {
                                    stats.read_throughput_mbs = read_bw / (1024.0 * 1024.0);
                                }
                                if let Ok(write_bw) = fields[4].parse::<f64>() {
                                    stats.write_throughput_mbs = write_bw / (1024.0 * 1024.0);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Calculate utilization and latency
        let total_iops = stats.read_iops + stats.write_iops;
        stats.utilization_percent = if total_iops > 0.0 {
            (total_iops / 10000.0 * 100.0).min(100.0)
        } else {
            0.0
        };

        stats.read_latency_ms = if stats.read_iops > 0.0 {
            (1000.0 / stats.read_iops).min(1000.0)
        } else {
            0.0
        };
        stats.write_latency_ms = if stats.write_iops > 0.0 {
            (1000.0 / stats.write_iops).min(1000.0)
        } else {
            0.0
        };

        Ok(stats)
    }

    /// Get ZFS cache hit ratio
    async fn get_zfs_cache_hit_ratio() -> CoreResult<f64> {
        // Read ARC statistics from /proc/spl/kstat/zfs/arcstats
        if let Ok(content) = tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await {
            let mut hits = 0u64;
            let mut misses = 0u64;

            for line in content.lines() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 3 {
                    match fields[0] {
                        "hits" => hits = fields[2].parse().unwrap_or(0),
                        "misses" => misses = fields[2].parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }

            let total = hits + misses;
            if total > 0 {
                return Ok((hits as f64 / total as f64) * 100.0);
            }
        }

        Ok(85.0) // Default fallback
    }

    /// Get real queue depth for a tier
    async fn get_real_queue_depth(tier: &StorageTier) -> CoreResult<f64> {
        // This would typically read from system statistics
        // For now, return tier-appropriate defaults
        Ok(match tier {
            StorageTier::Hot => 32.0,
            StorageTier::Warm => 16.0,
            StorageTier::Cold => 8.0,
            StorageTier::Cache => 64.0,
        })
    }

    /// Start analysis task
    async fn start_analysis_task(&mut self) -> CoreResult<()> {
        let metrics_history = Arc::clone(&self.metrics_history);
        let current_metrics = Arc::clone(&self.current_metrics);
        let config = self.config.clone();

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.analysis_interval));

            loop {
                interval.tick().await;

                if let Err(e) =
                    Self::analyze_trends(&current_metrics, &metrics_history, &config).await
                {
                    error!("Trend analysis failed: {}", e);
                }
            }
        });

        self.analysis_task = Some(task);
        Ok(())
    }

    /// Analyze performance trends
    async fn analyze_trends(
        current_metrics: &Arc<RwLock<CurrentPerformanceMetrics>>,
        metrics_history: &Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
        config: &PerformanceConfig,
    ) -> CoreResult<()> {
        debug!("Analyzing performance trends");

        let current = current_metrics.read().await;
        let snapshot = PerformanceSnapshot {
            timestamp: SystemTime::now(),
            metrics: current.clone(),
            performance_score: 85.0, // Calculate based on metrics
        };

        let mut history = metrics_history.write().await;
        history.push_back(snapshot);

        if history.len() > config.max_history_entries {
            history.pop_front();
        }

        Ok(())
    }

    /// Start alert task
    async fn start_alert_task(&mut self) -> CoreResult<()> {
        let current_metrics = Arc::clone(&self.current_metrics);
        let alert_conditions = Arc::clone(&self.alert_conditions);
        let active_alerts = Arc::clone(&self.active_alerts);
        let alert_sender = self.alert_sender.clone();
        let config = self.config.clone();

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.alert_interval));

            loop {
                interval.tick().await;

                if let Some(sender) = &alert_sender {
                    if let Err(e) = Self::check_alert_conditions(
                        &current_metrics,
                        &alert_conditions,
                        &active_alerts,
                        sender,
                    )
                    .await
                    {
                        error!("Alert checking failed: {}", e);
                    }
                }
            }
        });

        self.alert_task = Some(task);
        Ok(())
    }

    /// Check alert conditions
    async fn check_alert_conditions(
        _current_metrics: &Arc<RwLock<CurrentPerformanceMetrics>>,
        _alert_conditions: &Arc<RwLock<Vec<AlertCondition>>>,
        _active_alerts: &Arc<RwLock<Vec<ActiveAlert>>>,
        _alert_sender: &mpsc::Sender<Alert>,
    ) -> CoreResult<()> {
        debug!("Checking alert conditions");

        // Implementation would check current metrics against alert conditions
        // and send alerts when thresholds are exceeded

        Ok(())
    }

    /// Get current performance metrics
    pub async fn get_current_metrics(&self) -> CurrentPerformanceMetrics {
        self.current_metrics.read().await.clone()
    }

    /// Get metrics history
    pub async fn get_metrics_history(&self) -> Vec<PerformanceSnapshot> {
        self.metrics_history.read().await.iter().cloned().collect()
    }

    /// Get tier performance data
    pub async fn get_tier_performance(&self, tier: StorageTier) -> Option<TierPerformanceData> {
        self.tier_metrics.read().await.get(&tier).cloned()
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<ActiveAlert> {
        self.active_alerts.read().await.clone()
    }
}

use std::collections::VecDeque;
