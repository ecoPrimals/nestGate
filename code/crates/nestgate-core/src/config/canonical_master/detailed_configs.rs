/// **DETAILED CONFIGURATION TYPES**
///
/// This module contains all the detailed configuration structures
/// used by the canonical master configuration system.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    pub metrics: MetricsConfig,
    pub logging: LoggingConfig,
    pub tracing: TracingConfig,
    pub health_checks: HealthChecksConfig,
    pub alerting: AlertingConfig,
    pub dashboards: DashboardsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub exporters: Vec<MetricExporter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricExporter {
    Prometheus { endpoint: String },
    Grafana { endpoint: String, api_key: String },
    StdOut,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
    pub outputs: Vec<LogOutput>,
    pub structured: bool,
    pub include_timestamps: bool,
    pub include_caller: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    StdOut,
    StdErr,
    Syslog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRotation {
    pub max_size_mb: u64,
    pub max_files: u32,
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    pub enabled: bool,
    pub sampling_rate: f64,
    pub service_name: String,
    pub jaeger_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthChecksConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub timeout: Duration,
    pub endpoints: Vec<HealthCheckEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckEndpoint {
    pub name: String,
    pub url: String,
    pub expected_status: u16,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub channels: Vec<AlertChannel>,
    pub rules: Vec<AlertRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    Email { recipients: Vec<String> },
    Slack { webhook_url: String, channel: String },
    Webhook { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub condition: String,
    pub severity: AlertSeverity,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardsConfig {
    pub enabled: bool,
    pub grafana_url: Option<String>,
    pub custom_dashboards: Vec<PathBuf>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpConfig {
    pub enabled: bool,
    pub server: McpServerConfig,
    pub client: McpClientConfig,
    pub security: McpSecurityConfig,
    pub streaming: McpStreamingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpServerConfig {
    pub bind_endpoint: String,
    pub port: u16,
    pub max_connections: usize,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpClientConfig {
    pub default_timeout: Duration,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpSecurityConfig {
    pub authentication_required: bool,
    pub rate_limiting: bool,
    pub max_requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpStreamingConfig {
    pub enabled: bool,
    pub buffer_size: usize,
    pub max_streams: usize,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomationConfig {
    pub enabled: bool,
    pub workflows: WorkflowsConfig,
    pub scheduling: SchedulingConfig,
    pub triggers: TriggersConfig,
    pub actions: ActionsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowsConfig {
    pub max_concurrent: usize,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulingConfig {
    pub enabled: bool,
    pub timezone: String,
    pub max_scheduled_tasks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TriggersConfig {
    pub file_system: bool,
    pub time_based: bool,
    pub event_based: bool,
    pub api_based: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActionsConfig {
    pub shell_commands: bool,
    pub http_requests: bool,
    pub database_operations: bool,
    pub file_operations: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FsMonitorConfig {
    pub enabled: bool,
    pub watch_paths: Vec<WatchPath>,
    pub events: EventsConfig,
    pub filters: FiltersConfig,
    pub notifications: NotificationsConfig,
    pub performance: FsMonitorPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPath {
    pub recursive: bool,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventsConfig {
    pub create: bool,
    pub modify: bool,
    pub delete: bool,
    pub move_: bool,
    pub chmod: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FiltersConfig {
    pub min_file_size: Option<u64>,
    pub max_file_size: Option<u64>,
    pub file_extensions: Vec<String>,
    pub ignore_hidden: bool,
    pub ignore_temp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationsConfig {
    pub enabled: bool,
    pub channels: Vec<NotificationChannel>,
    pub batch_size: usize,
    pub batch_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Webhook { url: String },
    Database { connection_string: String },
    Queue { queue_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FsMonitorPerformanceConfig {
    pub buffer_size: usize,
    pub worker_threads: usize,
    pub batch_processing: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasConfig {
    pub enabled: bool,
    pub shares: Vec<NasShare>,
    pub protocols: NasProtocolsConfig,
    pub security: NasSecurityConfig,
    pub performance: NasPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasShare {
    pub name: String,
    pub read_only: bool,
    pub allowed_hosts: Vec<String>,
    pub protocols: Vec<NasProtocol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NasProtocol {
    Nfs,
    Smb,
    Ftp,
    Sftp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasProtocolsConfig {
    pub nfs: NfsConfig,
    pub smb: SmbConfig,
    pub ftp: FtpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NfsConfig {
    pub enabled: bool,
    pub version: NfsVersion,
    pub port: u16,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NfsVersion {
    V3,
    V4,
    V41,
    V42,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmbConfig {
    pub enabled: bool,
    pub version: SmbVersion,
    pub port: u16,
    pub workgroup: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmbVersion {
    V2,
    V3,
    V31,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FtpConfig {
    pub enabled: bool,
    pub port: u16,
    pub passive_mode: bool,
    pub ssl_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasSecurityConfig {
    pub authentication_required: bool,
    pub encryption_enabled: bool,
    pub access_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasPerformanceConfig {
    pub buffer_size: usize,
    pub cache_size: usize,
    pub max_concurrent_transfers: usize,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiddlewareConfig {
    pub enabled: bool,
    pub request_middleware: Vec<RequestMiddleware>,
    pub response_middleware: Vec<ResponseMiddleware>,
    pub global_middleware: GlobalMiddlewareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestMiddleware {
    Authentication,
    Authorization,
    RateLimiting,
    RequestLogging,
    Compression,
    Cors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseMiddleware {
    ResponseLogging,
    Compression,
    Headers,
    ErrorHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalMiddlewareConfig {
    pub timeout: Duration,
    pub max_request_size: usize,
    pub cors_enabled: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    pub name: String,
    pub variables: HashMap<String, String>,
    pub secrets: HashMap<String, String>,
    pub overrides: HashMap<String, serde_json::Value>,
}

// ==================== SECTION ====================
// These are referenced in the main config but will be fully implemented later

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsConfig {
    pub enabled: bool,
    pub pools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UniversalStorageConfig {
    pub backends: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBackendsConfig {
    pub primary: String,
    pub fallback: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplicationConfig {
    pub enabled: bool,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupConfig {
    pub enabled: bool,
    pub schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageTiersConfig {
    pub hot: StorageTierConfig,
    pub warm: StorageTierConfig,
    pub cold: StorageTierConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageTierConfig {
    pub enabled: bool,
    pub threshold_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageEncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoragePerformanceConfig {
    pub cache_size: usize,
    pub io_threads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    pub enabled: bool,
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    pub enabled: bool,
    pub rbac: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub at_rest: bool,
    pub in_transit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityRateLimitingConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityPoliciesConfig {
    pub password_policy: PasswordPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PasswordPolicy {
    pub min_length: u32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditConfig {
    pub enabled: bool,
    pub log_all_requests: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityTlsConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    pub timeout: Duration,
    pub secure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuthConfig {
    pub enabled: bool,
    pub providers: Vec<OAuthProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiHandlersConfig {
    pub zfs: bool,
    pub performance: bool,
    pub dashboard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamingConfig {
    pub enabled: bool,
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiRpcConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiMiddlewareConfig {
    pub request_logging: bool,
    pub response_compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestResponseConfig {
    pub max_request_size: usize,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationConfig {
    pub strict_mode: bool,
    pub custom_validators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializationConfig {
    pub format: SerializationFormat,
    pub pretty_print: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializationFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig<const BUFFER_SIZE: usize = 65536> {
    pub heap_size: Option<usize>,
    pub buffer_size_override: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuConfig {
    pub max_cores: Option<usize>,
    pub affinity: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IoConfig {
    pub async_io: bool,
    pub io_uring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    pub enabled: bool,
    pub size: usize,
    pub ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig<const MAX_CONNECTIONS: usize = 1000> {
    pub min_threads: usize,
    pub max_threads: usize,
    pub max_connections_override: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub sampling_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimitsConfig {
    pub max_memory: Option<u64>,
    pub max_cpu: Option<f64>,
    pub max_file_descriptors: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrchestrationConfig {
    pub enabled: bool,
    pub discovery_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub registry_url: Option<String>,
}

// ==================== SECTION ====================

impl Default for LogFormat {
    fn default() -> Self {
        LogFormat::Text
    }
}

impl Default for NfsVersion {
    fn default() -> Self {
        NfsVersion::V4
    }
}

impl Default for SmbVersion {
    fn default() -> Self {
        SmbVersion::V3
    }
}

impl Default for SerializationFormat {
    fn default() -> Self {
        SerializationFormat::Json
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

impl<const BUFFER_SIZE: usize> Default for MemoryConfig<BUFFER_SIZE> {
    fn default() -> Self {
        Self {
            heap_size: None,
            buffer_size_override: None,
        }
    }
}

impl<const MAX_CONNECTIONS: usize> Default for ThreadPoolConfig<MAX_CONNECTIONS> {
    fn default() -> Self {
        Self {
            min_threads: 1,
            max_threads: num_cpus::get(),
            max_connections_override: None,
        }
    }
} 