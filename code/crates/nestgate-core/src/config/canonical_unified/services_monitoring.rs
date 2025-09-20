//
// Service-specific and monitoring configuration structures for the canonical unified configuration system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct PerformanceConfig {
    /// Buffer configuration
    pub buffers: BufferConfig,
    /// Thread pool configuration
    pub thread_pools: ThreadPoolConfig,
    /// Memory configuration
    pub memory: MemoryConfig,
    /// I/O configuration
    pub io: IoConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BufferConfig {
    pub read_buffer_size: usize,
    pub write_buffer_size: usize,
    pub network_buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreadPoolConfig {
    pub core_threads: usize,
    pub max_threads: usize,
    pub queue_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    pub max_heap_size: Option<u64>,
    pub gc_strategy: String,
    pub memory_pool_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IoConfig {
    pub async_io: bool,
    pub io_threads: usize,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    pub enabled: bool,
    pub max_size: u64,
    pub ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Metrics retention period  
    pub retention_period: Duration,
    /// Enable detailed metrics (for debugging/development)
    pub detailed_metrics: bool,
    /// Metrics storage path (optional, for file-based storage)
    /// Export format (prometheus, json, etc.)
    pub export_format: MetricsFormat,
    /// Maximum number of metrics to retain in memory
    pub max_memory_metrics: usize,
    /// Enable real-time metrics streaming
    pub enable_streaming: bool,
}

/// **CANONICAL METRICS FORMAT ENUM**
/// Consolidates all metrics format variants from across the codebase
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MetricsFormat {
    #[default]
    Prometheus,
    Json,
    Csv,
    InfluxDB,
    Grafana,
    Custom(String),
}
impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            detailed_metrics: false,
            export_format: MetricsFormat::default(),
            max_memory_metrics: 10000,
            enable_streaming: false,
        }
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ServiceConfigs {
    /// MCP service configuration
    pub mcp: McpServiceConfig,
    /// File system monitor service
    pub fsmonitor: FsMonitorServiceConfig,
    /// Network service configuration
    pub network: NetworkServiceConfig,
    /// ZFS service configuration
    pub zfs_service: ZfsServiceConfig,
    /// API service configuration
    pub api_service: ApiServiceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpServiceConfig {
    pub enabled: bool,
    pub port: u16,
    pub timeout: Duration,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FsMonitorServiceConfig {
    pub enabled: bool,
    pub watch_paths: Vec<PathBuf>,
    pub poll_interval: Duration,
    pub max_events: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkServiceConfig {
    pub enabled: bool,
    pub interface: String,
    pub discovery_interval: Duration,
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsServiceConfig {
    pub enabled: bool,
    pub pool_scan_interval: Duration,
    pub scrub_schedule: String,
    pub auto_snapshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiServiceConfig {
    pub enabled: bool,
    pub rate_limit: u32,
    pub request_timeout: Duration,
    pub max_payload_size: u64,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TestingConfigs {
    /// Unit test configuration
    pub unit: UnitTestConfig,
    /// Integration test configuration
    pub integration: IntegrationTestConfig,
    /// End-to-end test configuration
    pub e2e: E2eTestConfig,
    /// Performance test configuration
    pub performance: PerformanceTestConfig,
    /// Security test configuration
    pub security: SecurityTestConfig,
    /// Chaos test configuration
    pub chaos: ChaosTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnitTestConfig {
    pub enabled: bool,
    pub parallel: bool,
    pub timeout: Duration,
    pub coverage_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationTestConfig {
    pub enabled: bool,
    pub cleanup_after: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct E2eTestConfig {
    pub enabled: bool,
    pub browser: String,
    pub headless: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceTestConfig {
    pub enabled: bool,
    pub duration: Duration,
    pub concurrent_users: u32,
    pub target_rps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityTestConfig {
    pub enabled: bool,
    pub vulnerability_scan: bool,
    pub penetration_test: bool,
    pub compliance_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChaosTestConfig {
    pub enabled: bool,
    pub failure_rate: f32,
    pub recovery_time: Duration,
    pub target_services: Vec<String>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_port: u16,
    pub health_check_port: u16,
    pub alerting: AlertingConfig,
    pub logging: LoggingConfig,
    pub tracing: TracingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub webhook_url: Option<String>,
    pub email_recipients: Vec<String>,
    pub severity_threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub rotation: LogRotationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogRotationConfig {
    pub enabled: bool,
    pub max_size: u64,
    pub max_files: u32,
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    pub enabled: bool,
    pub sampling_rate: f32,
    pub jaeger_endpoint: Option<String>,
    pub service_name: String,
}

// ==================== SECTION ====================



 