// Core Configuration Types
//! Type definitions and data structures.
// This module contains only the data structures for configuration.
// No validation, loading, or business logic - just pure data types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// The canonical NestGate configuration
/// This is the single source of truth for all NestGate configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalConfig {
    /// Global system settings
    pub system: SystemConfig,
    /// Network and connectivity settings
    pub network: NetworkConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Security and authentication configuration
    pub security: SecurityConfig,
    /// Performance and optimization
    pub performance: PerformanceConfig,
    /// Monitoring and observability
    pub monitoring: MonitoringConfig,
    /// Integration with external systems
    pub integrations: IntegrationsConfig,
    /// Environment-specific overrides
}
// ==================== SECTION ====================

/// System-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// System identifier (auto-generated if not provided)
    pub instance_id: Option<String>,
    /// Human-readable name for this instance
    pub instance_name: String,
    /// Environment (dev, staging, prod)
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,
    /// Data directory for persistent storage
    pub data_dir: PathBuf,
    /// Configuration directory
    pub config_dir: PathBuf,
    /// Enable development mode features
    pub dev_mode: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

// ==================== SECTION ====================

/// Network configuration
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_master::domains::network`
#[deprecated(since = "0.9.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    /// Primary API server configuration
    pub api: ApiServerConfig,
    /// Internal service communication
    pub internal: InternalNetworkConfig,
    /// External service discovery
    pub discovery: ServiceDiscoveryConfig,
    /// Load balancing configuration
    pub load_balancer: Option<LoadBalancerConfig>,
}
/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServerConfig {
    /// Bind address (defaults to 127.0.0.1 for security)
    pub host: IpAddr,
    /// Port to bind to
    pub port: u16,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Request timeout
    pub timeout: Duration,
    /// Enable HTTP/2
    pub http2: bool,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
}
/// Internal network configuration
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_master::domains::network`
#[deprecated(since = "0.9.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalNetworkConfig {
    /// Internal communication port range
    pub port_range: (u16, u16),
    /// Cluster membership configuration
    pub cluster: Option<ClusterConfig>,
    /// Health check settings
    pub health_check: HealthCheckConfig,
}
/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// Enable automatic service discovery
    pub enabled: bool,
// DEPRECATED: Consul service discovery - migrate to capability-based discovery
// Capability-based discovery implemented
// DEPRECATED: Kubernetes orchestration - migrate to capability-based orchestration
// Capability-based discovery implemented
// DEPRECATED: etcd key-value store - migrate to capability-based storage
// Capability-based discovery implemented
    /// Discovery method (dns, consul, etcd, kubernetes)
    pub method: String,
    /// Discovery endpoint
    pub endpoint: Option<String>,
    /// Refresh interval
    pub refresh_interval: Duration,
}
/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancing strategy
    pub strategy: String,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Failover settings
    pub failover: FailoverConfig,
}
/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert_file: PathBuf,
    /// Private key file path
    pub key_file: PathBuf,
    /// CA certificate file path
    pub ca_file: Option<PathBuf>,
    /// Require client certificates
    pub client_auth: bool,
}
/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Cluster name
    pub name: String,
    /// Node ID
    pub node_id: String,
    /// Seed nodes
    pub seed_nodes: Vec<String>,
}
/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of retries before marking unhealthy
    pub retries: u32,
}
/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover timeout
    pub timeout: Duration,
}
// ==================== SECTION ====================

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageConfig {
    /// ZFS-specific settings
    pub zfs: ZfsConfig,
    /// Performance optimization settings
    pub performance: StoragePerformanceConfig,
    /// Backup and replication settings
    pub backup: BackupConfig,
}
/// ZFS-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {
    pub pools: Vec<String>,
    pub compression: String,
    pub deduplication: bool,
}
/// Storage performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceConfig {
    /// Cache size in bytes
    pub cache_size: u64,
    /// Read-ahead buffer size
    pub read_ahead_size: u64,
    /// Write buffer size
    pub write_buffer_size: u64,
}
/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    pub enabled: bool,
    /// Backup schedule (cron format)
    pub schedule: Option<String>,
    /// Backup retention policy
    pub retention: RetentionPolicy,
}
/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Number of daily backups to keep
    pub daily: u32,
    /// Number of weekly backups to keep
    pub weekly: u32,
    /// Number of monthly backups to keep
    pub monthly: u32,
}
// ==================== SECTION ====================

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// Authentication settings
    pub authentication: AuthConfig,
    /// Authorization settings
    pub authorization: AuthzConfig,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Rate limiting settings
    pub rate_limiting: RateLimitConfig,
}
/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub enabled: bool,
    pub method: String,
    pub providers: Vec<String>,
}
/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthzConfig {
    pub enabled: bool,
    pub policy_file: Option<PathBuf>,
}
/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_file: Option<PathBuf>,
}
/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
}
// ==================== SECTION ====================

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    /// Thread pool settings
    pub threads: ThreadConfig,
    /// Memory settings
    pub memory: MemoryConfig,
    /// I/O settings
    pub io: IoConfig,
}
/// Thread configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadConfig {
    /// Number of worker threads
    pub worker_threads: Option<usize>,
    /// Thread stack size
    pub stack_size: Option<usize>,
}
/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum memory usage in bytes
    pub max_memory: Option<u64>,
    /// Memory pool size
    pub pool_size: u64,
}
/// I/O configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoConfig {
    /// I/O buffer size
    pub buffer_size: u64,
    /// Maximum concurrent I/O operations
    pub max_concurrent_ops: u32,
}
// ==================== SECTION ====================

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    /// Logging settings
    pub logging: LoggingConfig,
    /// Alerting settings
    pub alerting: AlertConfig,
}
/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Collection interval
    pub interval: Duration,
}
/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format (json, text)
    pub format: String,
    /// Log output (stdout, file)
    pub output: String,
    /// Log file path (if output is file)
}
/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertConfig {
    pub enabled: bool,
    pub webhook_url: Option<String>,
    pub email_config: Option<HashMap<String, String>>,
}
// ==================== SECTION ====================

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationsConfig {
    /// External service integrations
    pub external_services: HashMap<String, ExternalServiceConfig>,
    /// Webhook configurations
    pub webhooks: Vec<WebhookConfig>,
}
/// External service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServiceConfig {
    /// Service endpoint
    pub endpoint: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Connection timeout
    pub timeout: Duration,
    /// Retry configuration
    pub retry: RetryConfig,
}
/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook name
    pub name: String,
    /// Webhook URL
    pub url: String,
    /// Events to trigger on
    pub events: Vec<String>,
}
/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
}
// ==================== SECTION ====================

/// Environment-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment name
    pub name: String,
    /// Environment variables
    pub variables: HashMap<String, String>,
    /// Feature flags
    pub features: HashMap<String, bool>,
}
