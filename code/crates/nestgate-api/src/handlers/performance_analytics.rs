use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

/// Performance analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Collection interval in seconds
    pub collection_interval: u64,
    /// Retention period in days
    pub retention_days: u32,
    /// Enable predictive analytics
    pub predictive_enabled: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage threshold (percentage)
    pub cpu_threshold: f64,
    /// Memory usage threshold (percentage)
    pub memory_threshold: f64,
    /// Disk usage threshold (percentage)
    pub disk_threshold: f64,
    /// Network latency threshold (ms)
    pub network_latency_threshold: f64,
    /// ZFS pool health threshold
    pub zfs_health_threshold: f64,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Timestamp of collection
    pub timestamp: DateTime<Utc>,
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk I/O metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// ZFS specific metrics
    pub zfs: ZfsMetrics,
    /// Application metrics
    pub application: ApplicationMetrics,
}

/// CPU performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Overall CPU usage percentage
    pub usage_percent: f64,
    /// Per-core usage percentages
    pub core_usage: Vec<f64>,
    /// Load averages (1min, 5min, 15min)
    pub load_average: [f64; 3],
    /// Context switches per second
    pub context_switches: u64,
    /// Interrupts per second
    pub interrupts: u64,
    /// CPU frequency (MHz)
    pub frequency: f64,
    /// Temperature (Celsius)
    pub temperature: Option<f64>,
}

/// Memory performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total memory (bytes)
    pub total: u64,
    /// Available memory (bytes)
    pub available: u64,
    /// Used memory (bytes)
    pub used: u64,
    /// Free memory (bytes)
    pub free: u64,
    /// Cached memory (bytes)
    pub cached: u64,
    /// Buffer memory (bytes)
    pub buffers: u64,
    /// Swap total (bytes)
    pub swap_total: u64,
    /// Swap used (bytes)
    pub swap_used: u64,
    /// Usage percentage
    pub usage_percent: f64,
}

/// Disk I/O performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Per-device metrics
    pub devices: HashMap<String, DiskDeviceMetrics>,
    /// Overall I/O wait percentage
    pub io_wait_percent: f64,
    /// Total read operations per second
    pub total_reads_per_sec: u64,
    /// Total write operations per second
    pub total_writes_per_sec: u64,
    /// Total read throughput (bytes/sec)
    pub total_read_throughput: u64,
    /// Total write throughput (bytes/sec)
    pub total_write_throughput: u64,
}

/// Per-device disk metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDeviceMetrics {
    /// Device name
    pub device: String,
    /// Total space (bytes)
    pub total_space: u64,
    /// Used space (bytes)
    pub used_space: u64,
    /// Available space (bytes)
    pub available_space: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Read operations per second
    pub reads_per_sec: u64,
    /// Write operations per second
    pub writes_per_sec: u64,
    /// Read throughput (bytes/sec)
    pub read_throughput: u64,
    /// Write throughput (bytes/sec)
    pub write_throughput: u64,
    /// Average queue depth
    pub queue_depth: f64,
    /// Average latency (ms)
    pub latency_ms: f64,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Per-interface metrics
    pub interfaces: HashMap<String, NetworkInterfaceMetrics>,
    /// Total bytes received per second
    pub total_rx_bytes_per_sec: u64,
    /// Total bytes transmitted per second
    pub total_tx_bytes_per_sec: u64,
    /// Total packets received per second
    pub total_rx_packets_per_sec: u64,
    /// Total packets transmitted per second
    pub total_tx_packets_per_sec: u64,
    /// Network errors per second
    pub errors_per_sec: u64,
}

/// Per-interface network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceMetrics {
    /// Interface name
    pub interface: String,
    /// Bytes received per second
    pub rx_bytes_per_sec: u64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: u64,
    /// Packets received per second
    pub rx_packets_per_sec: u64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: u64,
    /// Receive errors
    pub rx_errors: u64,
    /// Transmit errors
    pub tx_errors: u64,
    /// MTU size
    pub mtu: u32,
    /// Link speed (Mbps)
    pub speed: u64,
}

/// ZFS specific performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMetrics {
    /// Per-pool metrics
    pub pools: HashMap<String, ZfsPoolMetrics>,
    /// ARC (Adaptive Replacement Cache) metrics
    pub arc: ZfsArcMetrics,
    /// L2ARC metrics
    pub l2arc: Option<ZfsL2ArcMetrics>,
    /// ZIL (ZFS Intent Log) metrics
    pub zil: ZfsZilMetrics,
}

/// Per-ZFS pool metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolMetrics {
    /// Pool name
    pub pool: String,
    /// Pool health status
    pub health: String,
    /// Total capacity (bytes)
    pub capacity: u64,
    /// Used space (bytes)
    pub used: u64,
    /// Available space (bytes)
    pub available: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Deduplication ratio
    pub dedup_ratio: f64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Read operations per second
    pub reads_per_sec: u64,
    /// Write operations per second
    pub writes_per_sec: u64,
    /// Read throughput (bytes/sec)
    pub read_throughput: u64,
    /// Write throughput (bytes/sec)
    pub write_throughput: u64,
    /// Fragmentation percentage
    pub fragmentation: f64,
}

/// ZFS ARC metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsArcMetrics {
    /// ARC size (bytes)
    pub size: u64,
    /// ARC target size (bytes)
    pub target_size: u64,
    /// ARC maximum size (bytes)
    pub max_size: u64,
    /// ARC hit ratio
    pub hit_ratio: f64,
    /// ARC miss ratio
    pub miss_ratio: f64,
    /// Recently used cache size
    pub mru_size: u64,
    /// Most frequently used cache size
    pub mfu_size: u64,
}

/// ZFS L2ARC metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsL2ArcMetrics {
    /// L2ARC size (bytes)
    pub size: u64,
    /// L2ARC hit ratio
    pub hit_ratio: f64,
    /// L2ARC miss ratio
    pub miss_ratio: f64,
    /// L2ARC reads per second
    pub reads_per_sec: u64,
    /// L2ARC writes per second
    pub writes_per_sec: u64,
}

/// ZFS ZIL metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsZilMetrics {
    /// ZIL commits per second
    pub commits_per_sec: u64,
    /// ZIL writes per second
    pub writes_per_sec: u64,
    /// ZIL sync writes per second
    pub sync_writes_per_sec: u64,
    /// ZIL throughput (bytes/sec)
    pub throughput: u64,
}

/// Application-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// NestGate API metrics
    pub api: ApiMetrics,
    /// Active connections
    pub active_connections: u64,
    /// Request rate (requests/sec)
    pub request_rate: f64,
    /// Error rate (errors/sec)
    pub error_rate: f64,
    /// Average response time (ms)
    pub avg_response_time: f64,
    /// Memory usage (bytes)
    pub memory_usage: u64,
    /// CPU usage percentage
    pub cpu_usage: f64,
}

/// API-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Active WebSocket connections
    pub websocket_connections: u64,
    /// Active SSE connections
    pub sse_connections: u64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert ID
    pub id: Uuid,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Metric that triggered the alert
    pub metric: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Component that triggered the alert
    pub component: String,
    /// Suggested actions
    pub suggested_actions: Vec<String>,
}

/// Types of performance alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// Resource usage threshold exceeded
    ResourceUsage,
    /// Performance degradation detected
    PerformanceDegradation,
    /// System health issue
    HealthIssue,
    /// Predictive alert
    Predictive,
    /// Configuration optimization needed
    ConfigurationOptimization,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation ID
    pub id: Uuid,
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Expected improvement
    pub expected_improvement: String,
    /// Implementation effort
    pub effort_level: EffortLevel,
    /// Priority
    pub priority: Priority,
    /// Actions to take
    pub actions: Vec<RecommendationAction>,
    /// Estimated impact
    pub estimated_impact: ImpactEstimate,
}

/// Types of performance recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Hardware configuration
    Hardware,
    /// ZFS configuration
    ZfsConfiguration,
    /// Network optimization
    Network,
    /// Application tuning
    Application,
    /// Resource allocation
    ResourceAllocation,
}

/// Effort level for implementing recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// Recommendation priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Specific action in a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationAction {
    /// Action description
    pub description: String,
    /// Command to execute (if applicable)
    pub command: Option<String>,
    /// Configuration changes required
    pub config_changes: Option<HashMap<String, String>>,
    /// Risk level
    pub risk_level: RiskLevel,
}

/// Risk level for recommendation actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Impact estimate for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactEstimate {
    /// Performance improvement percentage
    pub performance_improvement: f64,
    /// Resource usage reduction percentage
    pub resource_reduction: f64,
    /// Estimated timeframe for benefits
    pub timeframe: String,
    /// Confidence level (0-100)
    pub confidence: u8,
}

/// Performance analytics manager
pub struct PerformanceAnalytics {
    /// Configuration
    config: Arc<RwLock<PerformanceConfig>>,
    /// Historical metrics storage
    metrics_history: Arc<RwLock<Vec<SystemMetrics>>>,
    /// Active alerts
    active_alerts: Arc<RwLock<Vec<PerformanceAlert>>>,
    /// Performance recommendations
    recommendations: Arc<RwLock<Vec<PerformanceRecommendation>>>,
    /// Collection task handle
    collection_task: Option<tokio::task::JoinHandle<()>>,
}

impl PerformanceAnalytics {
    /// Create new performance analytics manager
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            active_alerts: Arc::new(RwLock::new(Vec::new())),
            recommendations: Arc::new(RwLock::new(Vec::new())),
            collection_task: None,
        }
    }

    /// Start performance monitoring
    pub async fn start_monitoring(
        &mut self,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🔬 Starting performance analytics monitoring");

        let config = self.config.clone();
        let metrics_history = self.metrics_history.clone();
        let active_alerts = self.active_alerts.clone();
        let recommendations = self.recommendations.clone();

        let collection_task = tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_secs(config.read().await.collection_interval));

            loop {
                interval.tick().await;

                match Self::collect_system_metrics().await {
                    Ok(metrics) => {
                        // Store metrics
                        {
                            let mut history = metrics_history.write().await;
                            history.push(metrics.clone());

                            // Limit history size based on retention
                            let retention_hours = config.read().await.retention_days as u64 * 24;
                            let max_entries =
                                retention_hours * 3600 / config.read().await.collection_interval;

                            let len = history.len();
                            if len > max_entries as usize {
                                history.drain(0..(len - max_entries as usize));
                            }
                        }

                        // Check for alerts
                        if let Ok(alerts) =
                            Self::check_alerts(&metrics, &*config.read().await).await
                        {
                            if !alerts.is_empty() {
                                let mut active = active_alerts.write().await;
                                active.extend(alerts);

                                // Keep only recent alerts (last 24 hours)
                                let cutoff = Utc::now() - chrono::Duration::hours(24);
                                active.retain(|alert| alert.timestamp > cutoff);
                            }
                        }

                        // Generate recommendations
                        if config.read().await.predictive_enabled {
                            if let Ok(recs) = Self::generate_recommendations(
                                &metrics,
                                &*metrics_history.read().await,
                            )
                            .await
                            {
                                if !recs.is_empty() {
                                    let mut recommendations_guard = recommendations.write().await;
                                    for rec in recs {
                                        // Only add if not already present
                                        if !recommendations_guard
                                            .iter()
                                            .any(|r| r.title == rec.title)
                                        {
                                            recommendations_guard.push(rec);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to collect system metrics: {}", e),
                }
            }
        });

        self.collection_task = Some(collection_task);
        Ok(())
    }

    /// Stop performance monitoring
    pub async fn stop_monitoring(&mut self) {
        if let Some(task) = self.collection_task.take() {
            task.abort();
            info!("⏹️ Stopped performance analytics monitoring");
        }
    }

    /// Get current system metrics
    pub async fn get_current_metrics(
        &self,
    ) -> std::result::Result<SystemMetrics, Box<dyn std::error::Error + Send + Sync>> {
        Self::collect_system_metrics().await
    }

    /// Get historical metrics
    pub async fn get_historical_metrics(&self, hours: u32) -> Vec<SystemMetrics> {
        let history = self.metrics_history.read().await;
        let cutoff = Utc::now() - chrono::Duration::hours(hours as i64);

        history
            .iter()
            .filter(|metrics| metrics.timestamp > cutoff)
            .cloned()
            .collect()
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<PerformanceAlert> {
        self.active_alerts.read().await.clone()
    }

    /// Get performance recommendations
    pub async fn get_recommendations(&self) -> Vec<PerformanceRecommendation> {
        self.recommendations.read().await.clone()
    }

    /// Collect current system metrics
    async fn collect_system_metrics(
    ) -> std::result::Result<SystemMetrics, Box<dyn std::error::Error + Send + Sync>> {
        // Implementation would collect real system metrics
        // For now, return mock data for demonstration
        Ok(SystemMetrics {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                usage_percent: 45.2,
                core_usage: vec![42.1, 48.3, 44.7, 46.8],
                load_average: [1.2, 1.5, 1.8],
                context_switches: 15000,
                interrupts: 8500,
                frequency: 3400.0,
                temperature: Some(65.0),
            },
            memory: MemoryMetrics {
                total: 32 * 1024 * 1024 * 1024,     // 32GB
                available: 18 * 1024 * 1024 * 1024, // 18GB
                used: 14 * 1024 * 1024 * 1024,      // 14GB
                free: 18 * 1024 * 1024 * 1024,      // 18GB
                cached: 8 * 1024 * 1024 * 1024,     // 8GB
                buffers: 2 * 1024 * 1024 * 1024,    // 2GB
                swap_total: 8 * 1024 * 1024 * 1024, // 8GB
                swap_used: 256 * 1024 * 1024,       // 256MB
                usage_percent: 43.75,
            },
            disk: DiskMetrics {
                devices: {
                    let mut devices = HashMap::new();
                    devices.insert(
                        "sda".to_string(),
                        DiskDeviceMetrics {
                            device: "sda".to_string(),
                            total_space: 1000 * 1024 * 1024 * 1024, // 1TB
                            used_space: 450 * 1024 * 1024 * 1024,   // 450GB
                            available_space: 550 * 1024 * 1024 * 1024, // 550GB
                            usage_percent: 45.0,
                            reads_per_sec: 150,
                            writes_per_sec: 89,
                            read_throughput: 25 * 1024 * 1024, // 25MB/s
                            write_throughput: 18 * 1024 * 1024, // 18MB/s
                            queue_depth: 2.3,
                            latency_ms: 8.5,
                        },
                    );
                    devices
                },
                io_wait_percent: 3.2,
                total_reads_per_sec: 150,
                total_writes_per_sec: 89,
                total_read_throughput: 25 * 1024 * 1024,
                total_write_throughput: 18 * 1024 * 1024,
            },
            network: NetworkMetrics {
                interfaces: {
                    let mut interfaces = HashMap::new();
                    interfaces.insert(
                        "eth0".to_string(),
                        NetworkInterfaceMetrics {
                            interface: "eth0".to_string(),
                            rx_bytes_per_sec: 2 * 1024 * 1024, // 2MB/s
                            tx_bytes_per_sec: 1024 * 1024,     // 1MB/s
                            rx_packets_per_sec: 1500,
                            tx_packets_per_sec: 800,
                            rx_errors: 0,
                            tx_errors: 0,
                            mtu: 1500,
                            speed: 1000, // 1Gbps
                        },
                    );
                    interfaces
                },
                total_rx_bytes_per_sec: 2 * 1024 * 1024,
                total_tx_bytes_per_sec: 1024 * 1024,
                total_rx_packets_per_sec: 1500,
                total_tx_packets_per_sec: 800,
                errors_per_sec: 0,
            },
            zfs: ZfsMetrics {
                pools: {
                    let mut pools = HashMap::new();
                    pools.insert(
                        "nestpool".to_string(),
                        ZfsPoolMetrics {
                            pool: "nestpool".to_string(),
                            health: "ONLINE".to_string(),
                            capacity: 1800 * 1024 * 1024 * 1024, // 1.8TB
                            used: 800 * 1024 * 1024 * 1024,      // 800GB
                            available: 1000 * 1024 * 1024 * 1024, // 1TB
                            usage_percent: 44.4,
                            dedup_ratio: 1.2,
                            compression_ratio: 1.8,
                            reads_per_sec: 120,
                            writes_per_sec: 75,
                            read_throughput: 20 * 1024 * 1024, // 20MB/s
                            write_throughput: 15 * 1024 * 1024, // 15MB/s
                            fragmentation: 12.0,
                        },
                    );
                    pools
                },
                arc: ZfsArcMetrics {
                    size: 8 * 1024 * 1024 * 1024,        // 8GB
                    target_size: 8 * 1024 * 1024 * 1024, // 8GB
                    max_size: 16 * 1024 * 1024 * 1024,   // 16GB
                    hit_ratio: 95.2,
                    miss_ratio: 4.8,
                    mru_size: 3 * 1024 * 1024 * 1024, // 3GB
                    mfu_size: 5 * 1024 * 1024 * 1024, // 5GB
                },
                l2arc: None,
                zil: ZfsZilMetrics {
                    commits_per_sec: 45,
                    writes_per_sec: 38,
                    sync_writes_per_sec: 12,
                    throughput: 5 * 1024 * 1024, // 5MB/s
                },
            },
            application: ApplicationMetrics {
                api: ApiMetrics {
                    total_requests: 125000,
                    successful_requests: 123500,
                    failed_requests: 1500,
                    websocket_connections: 25,
                    sse_connections: 15,
                    cache_hit_ratio: 88.5,
                },
                active_connections: 40,
                request_rate: 15.2,
                error_rate: 0.1,
                avg_response_time: 125.0,
                memory_usage: 512 * 1024 * 1024, // 512MB
                cpu_usage: 12.5,
            },
        })
    }

    /// Check for performance alerts
    async fn check_alerts(
        metrics: &SystemMetrics,
        config: &PerformanceConfig,
    ) -> std::result::Result<Vec<PerformanceAlert>, Box<dyn std::error::Error + Send + Sync>> {
        let mut alerts = Vec::new();

        // CPU usage alert
        if metrics.cpu.usage_percent > config.alert_thresholds.cpu_threshold {
            alerts.push(PerformanceAlert {
                id: Uuid::new_v4(),
                alert_type: AlertType::ResourceUsage,
                severity: if metrics.cpu.usage_percent > 90.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                message: format!("High CPU usage detected: {:.1}%", metrics.cpu.usage_percent),
                metric: "cpu.usage_percent".to_string(),
                current_value: metrics.cpu.usage_percent,
                threshold_value: config.alert_thresholds.cpu_threshold,
                timestamp: Utc::now(),
                component: "CPU".to_string(),
                suggested_actions: vec![
                    "Review running processes".to_string(),
                    "Consider CPU scaling or optimization".to_string(),
                ],
            });
        }

        // Memory usage alert
        if metrics.memory.usage_percent > config.alert_thresholds.memory_threshold {
            alerts.push(PerformanceAlert {
                id: Uuid::new_v4(),
                alert_type: AlertType::ResourceUsage,
                severity: if metrics.memory.usage_percent > 95.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                message: format!(
                    "High memory usage detected: {:.1}%",
                    metrics.memory.usage_percent
                ),
                metric: "memory.usage_percent".to_string(),
                current_value: metrics.memory.usage_percent,
                threshold_value: config.alert_thresholds.memory_threshold,
                timestamp: Utc::now(),
                component: "Memory".to_string(),
                suggested_actions: vec![
                    "Check for memory leaks".to_string(),
                    "Review ZFS ARC settings".to_string(),
                    "Consider adding more memory".to_string(),
                ],
            });
        }

        // Check ZFS pool health
        for (pool_name, pool_metrics) in &metrics.zfs.pools {
            if pool_metrics.health != "ONLINE" {
                alerts.push(PerformanceAlert {
                    id: Uuid::new_v4(),
                    alert_type: AlertType::HealthIssue,
                    severity: AlertSeverity::Critical,
                    message: format!(
                        "ZFS pool {} health issue: {}",
                        pool_name, pool_metrics.health
                    ),
                    metric: format!("zfs.pools.{}.health", pool_name),
                    current_value: 0.0, // Health is not numeric
                    threshold_value: 1.0,
                    timestamp: Utc::now(),
                    component: format!("ZFS Pool {}", pool_name),
                    suggested_actions: vec![
                        "Check pool status with 'zpool status'".to_string(),
                        "Review system logs for disk errors".to_string(),
                        "Consider immediate maintenance".to_string(),
                    ],
                });
            }
        }

        Ok(alerts)
    }

    /// Generate performance recommendations
    async fn generate_recommendations(
        current_metrics: &SystemMetrics,
        _historical_metrics: &[SystemMetrics],
    ) -> std::result::Result<Vec<PerformanceRecommendation>, Box<dyn std::error::Error + Send + Sync>>
    {
        let mut recommendations = Vec::new();

        // ZFS ARC tuning recommendation
        if current_metrics.zfs.arc.hit_ratio < 90.0 {
            recommendations.push(PerformanceRecommendation {
                id: Uuid::new_v4(),
                recommendation_type: RecommendationType::ZfsConfiguration,
                title: "Optimize ZFS ARC Configuration".to_string(),
                description: format!(
                    "ZFS ARC hit ratio is {:.1}%, which is below optimal. Consider tuning ARC parameters.",
                    current_metrics.zfs.arc.hit_ratio
                ),
                expected_improvement: "5-15% improvement in disk I/O performance".to_string(),
                effort_level: EffortLevel::Medium,
                priority: Priority::Medium,
                actions: vec![
                    RecommendationAction {
                        description: "Increase ARC maximum size".to_string(),
                        command: Some("echo 'zfs_arc_max=17179869184' >> /etc/modprobe.d/zfs.conf".to_string()),
                        config_changes: Some({
                            let mut changes = HashMap::new();
                            changes.insert("zfs_arc_max".to_string(), "16GB".to_string());
                            changes
                        }),
                        risk_level: RiskLevel::Low,
                    },
                ],
                estimated_impact: ImpactEstimate {
                    performance_improvement: 10.0,
                    resource_reduction: 0.0,
                    timeframe: "Immediate".to_string(),
                    confidence: 85,
                },
            });
        }

        // Disk fragmentation recommendation
        for (pool_name, pool_metrics) in &current_metrics.zfs.pools {
            if pool_metrics.fragmentation > 30.0 {
                recommendations.push(PerformanceRecommendation {
                    id: Uuid::new_v4(),
                    recommendation_type: RecommendationType::ZfsConfiguration,
                    title: format!("Defragment ZFS Pool {}", pool_name),
                    description: format!(
                        "Pool {} has {:.1}% fragmentation, which may impact performance.",
                        pool_name, pool_metrics.fragmentation
                    ),
                    expected_improvement: "10-20% improvement in sequential I/O".to_string(),
                    effort_level: EffortLevel::High,
                    priority: Priority::Medium,
                    actions: vec![RecommendationAction {
                        description: "Schedule ZFS pool defragmentation".to_string(),
                        command: Some(format!("zpool online -e {}", pool_name)),
                        config_changes: None,
                        risk_level: RiskLevel::Medium,
                    }],
                    estimated_impact: ImpactEstimate {
                        performance_improvement: 15.0,
                        resource_reduction: 0.0,
                        timeframe: "1-2 hours".to_string(),
                        confidence: 75,
                    },
                });
            }
        }

        Ok(recommendations)
    }
}

/// Create performance analytics router
pub fn create_performance_router() -> Router<crate::routes::AppState> {
    Router::new()
        .route("/metrics/current", get(get_current_metrics))
        .route("/metrics/historical", get(get_historical_metrics))
        .route("/alerts", get(get_alerts))
        .route("/alerts/:alert_id/acknowledge", post(acknowledge_alert))
        .route("/recommendations", get(get_recommendations))
        .route("/recommendations/:rec_id/apply", post(apply_recommendation))
        .route("/config", get(get_config))
        .route("/config", post(update_config))
        .route("/dashboard", get(get_dashboard))
}

/// Get current performance metrics
pub async fn get_current_metrics(
    State(_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    info!("📊 Getting current performance metrics");

    // In a real implementation, this would fetch from the performance analytics manager
    match PerformanceAnalytics::collect_system_metrics().await {
        Ok(metrics) => Json(serde_json::json!({
            "status": "success",
            "metrics": metrics,
            "timestamp": Utc::now()
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to collect current metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Failed to collect current metrics",
                    "error": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Query parameters for historical metrics
#[derive(Debug, Deserialize)]
pub struct HistoricalMetricsQuery {
    /// Hours of history to retrieve
    pub hours: Option<u32>,
    /// Specific metrics to include
    pub metrics: Option<String>,
    /// Aggregation interval (minutes)
    pub interval: Option<u32>,
}

/// Get historical performance metrics
pub async fn get_historical_metrics(
    Query(query): Query<HistoricalMetricsQuery>,
    State(_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    let hours = query.hours.unwrap_or(24);
    info!(
        "📈 Getting {} hours of historical performance metrics",
        hours
    );

    // In a real implementation, this would fetch from the performance analytics manager
    Json(serde_json::json!({
        "status": "success",
        "metrics": [],
        "hours": hours,
        "interval_minutes": query.interval.unwrap_or(5),
        "timestamp": Utc::now()
    }))
}

/// Get active performance alerts
pub async fn get_alerts(State(_state): State<crate::routes::AppState>) -> impl IntoResponse {
    info!("🚨 Getting active performance alerts");

    Json(serde_json::json!({
        "status": "success",
        "alerts": [],
        "count": 0,
        "timestamp": Utc::now()
    }))
}

/// Acknowledge a performance alert
pub async fn acknowledge_alert(
    Path(alert_id): Path<Uuid>,
    State(_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    info!("✅ Acknowledging performance alert: {}", alert_id);

    Json(serde_json::json!({
        "status": "success",
        "alert_id": alert_id,
        "acknowledged": true,
        "timestamp": Utc::now()
    }))
}

/// Get performance recommendations
pub async fn get_recommendations(
    State(_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    info!("💡 Getting performance recommendations");

    Json(serde_json::json!({
        "status": "success",
        "recommendations": [],
        "count": 0,
        "timestamp": Utc::now()
    }))
}

/// Apply a performance recommendation
pub async fn apply_recommendation(
    Path(rec_id): Path<Uuid>,
    State(_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    info!("🔧 Applying performance recommendation: {}", rec_id);

    Json(serde_json::json!({
        "status": "success",
        "recommendation_id": rec_id,
        "applied": true,
        "timestamp": Utc::now()
    }))
}

/// Get performance analytics configuration
pub async fn get_config(State(_state): State<crate::routes::AppState>) -> impl IntoResponse {
    info!("⚙️ Getting performance analytics configuration");

    let default_config = PerformanceConfig {
        collection_interval: 60,
        retention_days: 30,
        predictive_enabled: true,
        alert_thresholds: AlertThresholds {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            network_latency_threshold: 100.0,
            zfs_health_threshold: 95.0,
        },
    };

    Json(serde_json::json!({
        "status": "success",
        "config": default_config,
        "timestamp": Utc::now()
    }))
}

/// Update performance analytics configuration
pub async fn update_config(
    State(_state): State<crate::routes::AppState>,
    Json(config): Json<PerformanceConfig>,
) -> impl IntoResponse {
    info!("🔄 Updating performance analytics configuration");

    Json(serde_json::json!({
        "status": "success",
        "config": config,
        "updated": true,
        "timestamp": Utc::now()
    }))
}

/// Get performance dashboard data
pub async fn get_dashboard(State(_state): State<crate::routes::AppState>) -> impl IntoResponse {
    info!("📊 Getting performance dashboard data");

    Json(serde_json::json!({
        "status": "success",
        "dashboard": {
            "system_health": "Good",
            "overall_performance": 92.5,
            "active_alerts": 0,
            "recommendations": 2,
            "uptime_hours": 168
        },
        "timestamp": Utc::now()
    }))
}

// Simple wrapper functions for the routes

/// Get performance metrics (wrapper for get_current_metrics)
pub async fn get_performance_metrics() -> impl IntoResponse {
    info!("📊 Getting performance metrics");

    match PerformanceAnalytics::collect_system_metrics().await {
        Ok(metrics) => Json(serde_json::json!({
            "status": "success",
            "metrics": metrics,
            "timestamp": Utc::now()
        })),
        Err(e) => {
            error!("Failed to collect performance metrics: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to collect performance metrics",
                "error": e.to_string()
            }))
        }
    }
}

/// Get performance alerts (wrapper for get_alerts)
pub async fn get_performance_alerts() -> impl IntoResponse {
    info!("🚨 Getting performance alerts");

    Json(serde_json::json!({
        "status": "success",
        "alerts": [],
        "count": 0,
        "timestamp": Utc::now()
    }))
}

/// Get performance recommendations (wrapper for get_recommendations)
pub async fn get_performance_recommendations() -> impl IntoResponse {
    info!("💡 Getting performance recommendations");

    Json(serde_json::json!({
        "status": "success",
        "recommendations": [],
        "count": 0,
        "timestamp": Utc::now()
    }))
}
