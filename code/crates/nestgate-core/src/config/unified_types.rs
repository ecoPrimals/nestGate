//! **UNIFIED CONFIGURATION SUPPORTING TYPES**
//!
//! This module contains all the supporting type definitions for the unified configuration system.
//! These types are used by the main NestGateCanonicalConfig structure.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== SECTION ====================

/// Environment types for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

// ==================== SECTION ====================

/// System configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub service_name: String,
    pub version: String,
    pub environment: Environment,
    pub log_level: String,
    pub debug_mode: bool,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
}

// ==================== SECTION ====================

/// Network configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    /// API server host
    pub host: String,
    /// API server port
    pub port: u16,
    /// Connection timeout
    pub timeout: Duration,
    /// Enable TLS
    pub tls_enabled: bool,
    /// Maximum connections
    pub max_connections: usize,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HandlerConfig {
    /// ZFS handler configuration
    pub zfs: ZfsHandlerConfig,
    /// Performance monitoring handler
    pub performance: PerformanceHandlerConfig,
    /// Load testing handler
    pub load_testing: LoadTestingConfig,
    /// Dashboard handler
    pub dashboard: DashboardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsHandlerConfig {
    /// Enable ZFS operations
    pub enabled: bool,
    /// Maximum pool operations per second
    pub max_operations_per_sec: u32,
    /// Pool operation timeout
    pub pool_timeout: Duration,
    /// Enable advanced ZFS features
    pub advanced_features: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceHandlerConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Enable detailed profiling
    pub detailed_profiling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestingConfig {
    /// Enable load testing endpoints
    pub enabled: bool,
    /// Maximum concurrent test connections
    pub max_test_connections: usize,
    /// Test duration limit
    pub max_test_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardConfig {
    /// Enable web dashboard
    pub enabled: bool,
    /// Dashboard refresh interval
    pub refresh_interval: Duration,
    /// Enable real-time updates
    pub real_time: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst limit
    pub burst_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// Authentication method
    pub method: AuthMethod,
    /// Token expiration time
    pub token_expiry: Duration,
    /// Enable refresh tokens
    pub refresh_tokens: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    None,
    Basic,
    Bearer,
    Oauth2,
}

impl Default for AuthMethod {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorsConfig {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate file path
    pub cert_path: PathBuf,
    /// Private key file path
    pub key_path: PathBuf,
    /// CA certificate path (for mutual TLS)
    pub ca_path: Option<PathBuf>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsConfig {
    /// ZFS pool configuration
    pub pools: PoolConfig,
    /// Dataset configuration
    pub datasets: DatasetConfig,
    /// Snapshot configuration
    pub snapshots: SnapshotConfig,
    /// Performance tuning
    pub performance: ZfsPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoolConfig {
    /// Default pool name
    pub default_pool: String,
    /// Pool creation settings
    pub creation: PoolCreationConfig,
    /// Health check settings
    pub health_check: PoolHealthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoolCreationConfig {
    /// Default RAID level
    pub raid_level: String,
    /// Enable compression
    pub compression: bool,
    /// Enable deduplication
    pub deduplication: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoolHealthConfig {
    /// Health check interval
    pub check_interval: Duration,
    /// Enable automatic repair
    pub auto_repair: bool,
    /// Scrub schedule
    pub scrub_schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetConfig {
    /// Default quota per dataset
    pub default_quota: u64,
    /// Enable quota enforcement
    pub quota_enforcement: bool,
    /// Default record size
    pub record_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapshotConfig {
    /// Enable automatic snapshots
    pub auto_snapshots: bool,
    /// Snapshot retention policy
    pub retention_days: u32,
    /// Snapshot naming pattern
    pub naming_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {
    /// ARC size limit (bytes)
    pub arc_size: u64,
    /// Enable L2ARC
    pub l2arc_enabled: bool,
    /// Prefetch settings
    pub prefetch: PrefetchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrefetchConfig {
    /// Enable prefetching
    pub enabled: bool,
    /// Prefetch distance
    pub distance: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TierConfig {
    /// Enable storage tiering
    pub enabled: bool,
    /// Hot tier configuration
    pub hot: TierSettings,
    /// Warm tier configuration
    pub warm: TierSettings,
    /// Cold tier configuration
    pub cold: TierSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TierSettings {
    /// Storage backend for this tier
    pub backend: String,
    /// Size limit (bytes)
    pub size_limit: u64,
    /// Access frequency threshold
    pub access_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupConfig {
    /// Enable backups
    pub enabled: bool,
    /// Backup destination
    pub destination: PathBuf,
    /// Backup schedule
    pub schedule: String,
    /// Retention policy
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompressionConfig {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level (1-9)
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplicationConfig {
    /// Enable replication
    pub enabled: bool,
    /// Replication targets
    pub targets: Vec<ReplicationTarget>,
    /// Replication mode
    pub mode: ReplicationMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationTarget {
    /// Target host
    pub host: String,
    /// Target port
    pub port: u16,
    /// Authentication credentials
    pub auth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationMode {
    Sync,
    Async,
    SemiSync,
}

impl Default for ReplicationMode {
    fn default() -> Self {
        Self::Async
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery method
    pub method: DiscoveryMethod,
    /// Discovery endpoints
    pub endpoints: Vec<String>,
    /// Discovery timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Static,
    Dns,
    Consul,
    Etcd,
}

impl Default for DiscoveryMethod {
    fn default() -> Self {
        Self::Static
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancerConfig {
    /// Enable load balancing
    pub enabled: bool,
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    Random,
}

impl Default for LoadBalancingAlgorithm {
    fn default() -> Self {
        Self::RoundRobin
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
    /// Healthy threshold
    pub healthy_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InternalNetworkConfig {
    /// Internal port range
    pub port_range: (u16, u16),
    /// Enable encryption for internal communication
    pub encryption: bool,
    /// Internal network buffer size
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalNetworkConfig {
    /// External API port
    pub api_port: u16,
    /// Enable external access
    pub external_access: bool,
    /// Allowed external IP ranges
    pub allowed_ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    /// Enable circuit breaker
    pub enabled: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery timeout
    pub recovery_timeout: Duration,
    /// Half-open max calls
    pub half_open_max_calls: u32,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    /// Authentication providers
    pub providers: Vec<AuthProvider>,
    /// Default provider
    pub default_provider: String,
    /// Session configuration
    pub session: SessionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    /// Provider name
    pub name: String,
    /// Provider type
    pub provider_type: String,
    /// Provider configuration
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Enable session persistence
    pub persistent: bool,
    /// Session storage backend
    pub storage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    /// Enable authorization
    pub enabled: bool,
    /// Default role
    pub default_role: String,
    /// Role definitions
    pub roles: HashMap<String, RoleDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    /// Role permissions
    pub permissions: Vec<String>,
    /// Role description
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    /// Enable encryption at rest
    pub at_rest: bool,
    /// Enable encryption in transit
    pub in_transit: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key size
    pub key_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyManagementConfig {
    /// Key storage backend
    pub storage: String,
    /// Key rotation interval
    pub rotation_interval: Duration,
    /// Enable hardware security module
    pub hsm_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityScanConfig {
    /// Enable security scanning
    pub enabled: bool,
    /// Scan interval
    pub scan_interval: Duration,
    /// Enable vulnerability scanning
    pub vulnerability_scanning: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    /// Thread pool configuration
    pub threads: ThreadConfig,
    /// Memory configuration
    pub memory: MemoryConfig,
    /// I/O configuration
    pub io: IoConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Performance testing configuration
    pub testing: PerformanceTestingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestingConfig {
    /// Number of test iterations to run
    pub test_iterations: usize,
    /// Target percentile for performance measurements (e.g., 95.0 for P95)
    pub percentile_target: f64,
    /// Baseline timeout in seconds
    pub baseline_timeout_seconds: u64,
    /// Maximum timeout in seconds
    pub max_timeout_seconds: u64,
}

impl Default for PerformanceTestingConfig {
    fn default() -> Self {
        Self {
            test_iterations: 100,
            percentile_target: 95.0,
            baseline_timeout_seconds: 30,
            max_timeout_seconds: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreadConfig {
    /// Worker thread count
    pub worker_threads: usize,
    /// Blocking thread count
    pub blocking_threads: usize,
    /// Enable work stealing
    pub work_stealing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    /// Maximum memory usage (bytes)
    pub max_memory: u64,
    /// Enable memory mapping
    pub memory_mapping: bool,
    /// Buffer pool size
    pub buffer_pool_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IoConfig {
    /// I/O buffer size
    pub buffer_size: usize,
    /// Enable direct I/O
    pub direct_io: bool,
    /// Async I/O queue depth
    pub queue_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size (bytes)
    pub size: u64,
    /// Cache TTL
    pub ttl: Duration,
    /// Cache eviction policy
    pub eviction_policy: String,
    /// Cache directory path
    pub cache_dir: String,
    /// Hot tier size (bytes)
    pub hot_tier_size: u64,
    /// Warm tier size (bytes)  
    pub warm_tier_size: u64,
    /// TTL in seconds
    pub ttl_seconds: u64,
    /// Cold tier unlimited flag
    pub cold_tier_unlimited: bool,
    /// Cache policy string
    pub policy: String,
}

impl CacheConfig {
    pub fn development() -> Self {
        Self {
            enabled: true,
            size: 100 * 1024 * 1024, // 100MB
            ttl: Duration::from_secs(3600),
            eviction_policy: "lru".to_string(),
            cache_dir: "/tmp/nestgate-cache".to_string(),
            hot_tier_size: 100 * 1024 * 1024,
            warm_tier_size: 0,
            ttl_seconds: 3600,
            cold_tier_unlimited: false,
            policy: "development".to_string(),
        }
    }

    pub fn high_performance() -> Self {
        Self {
            enabled: true,
            size: 1024 * 1024 * 1024, // 1GB
            ttl: Duration::from_secs(7200),
            eviction_policy: "lru".to_string(),
            cache_dir: "/var/cache/nestgate".to_string(),
            hot_tier_size: 1024 * 1024 * 1024,
            warm_tier_size: 1024 * 1024 * 1024,
            ttl_seconds: 7200,
            cold_tier_unlimited: true,
            policy: "high-performance".to_string(),
        }
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Tracing configuration
    pub tracing: TracingConfig,
    /// Alerting configuration
    pub alerting: AlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics export endpoint
    pub export_endpoint: Option<String>,
    /// Collection interval
    pub collection_interval: Duration,
    /// Metrics retention period
    pub retention_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log file path
    pub file_path: Option<PathBuf>,
    /// Enable structured logging
    pub structured: bool,
    /// Log rotation configuration
    pub rotation: LogRotationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogRotationConfig {
    /// Maximum log file size
    pub max_size: u64,
    /// Maximum number of log files
    pub max_files: u32,
    /// Enable compression of rotated logs
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    /// Enable distributed tracing
    pub enabled: bool,
    /// Tracing endpoint
    pub endpoint: Option<String>,
    /// Sample rate (0.0-1.0)
    pub sample_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert channels
    pub channels: Vec<AlertChannel>,
    /// Alert rules
    pub rules: Vec<AlertRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    /// Channel name
    pub name: String,
    /// Channel type (email, slack, webhook)
    pub channel_type: String,
    /// Channel configuration
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Metric to monitor
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator
    pub operator: String,
    /// Alert severity
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpConfig {
    /// Enable MCP protocol
    pub enabled: bool,
    /// MCP server configuration
    pub server: McpServerConfig,
    /// Protocol settings
    pub protocol: McpProtocolConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpServerConfig {
    /// MCP server port
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpProtocolConfig {
    /// Protocol version
    pub version: String,
    /// Enable compression
    pub compression: bool,
    /// Message size limit
    pub max_message_size: usize,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomationConfig {
    /// Enable automation services
    pub enabled: bool,
    /// Automation workflows
    pub workflows: WorkflowConfig,
    /// Scheduler configuration
    pub scheduler: SchedulerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowConfig {
    /// Maximum concurrent workflows
    pub max_concurrent: usize,
    /// Workflow timeout
    pub timeout: Duration,
    /// Enable workflow persistence
    pub persistent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulerConfig {
    /// Scheduler tick interval
    pub tick_interval: Duration,
    /// Maximum scheduled tasks
    pub max_tasks: usize,
    /// Enable cron-style scheduling
    pub cron_enabled: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationsConfig {
    /// External service integrations
    pub services: HashMap<String, ServiceIntegration>,
    /// Integration timeout
    pub timeout: Duration,
    /// Enable integration health checks
    pub health_checks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceIntegration {
    /// Service endpoint
    pub endpoint: String,
    /// Authentication configuration
    pub auth: Option<AuthConfig>,
    /// Integration-specific settings
    pub settings: HashMap<String, String>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    /// Environment variables to set
    pub variables: HashMap<String, String>,
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Environment-specific overrides
    pub overrides: HashMap<String, serde_json::Value>,
} 