// **DOMAIN CONFIGURATION TYPES**
//! Configuration types and utilities.
// This module provides domain-specific configuration structures for NestGate,
//! organized by functional area (API, Storage, Network, Security, etc.).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use super::unified_types::CacheConfig;

// ==================== SECTION ====================

/// **CONST GENERIC API CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig<
    const API_PORT: u16 = 8080,
    const TIMEOUT_MS: u64 = 30000,
> {
    pub bind_endpoint: String,
    pub port_override: Option<u16>,        // Runtime override for API_PORT
    pub timeout_override: Option<Duration>, // Runtime override for TIMEOUT_MS
    pub max_request_size: u64,
    pub cors_enabled: bool,
    pub cors_origins: Vec<String>,
    pub rate_limiting: RateLimitingConfig,
    pub middleware: Vec<String>,
    pub tls: TlsConfig,
}
impl<const API_PORT: u16, const TIMEOUT_MS: u64> ApiConfig<API_PORT, TIMEOUT_MS> {
    /// Get API port - compile-time optimized
    pub const fn api_port() -> u16 {
        API_PORT
    }

    /// Get timeout - compile-time optimized
    pub const fn timeout_ms() -> u64 {
        TIMEOUT_MS
    }

    /// Get effective port (runtime override or compile-time)
    pub const fn effective_port(&self) -> u16 {
        self.port_override.unwrap_or(API_PORT)
    }

    /// Get effective timeout (runtime override or compile-time)
    pub const fn effective_timeout(&self) -> Duration {
        self.timeout_override.unwrap_or(Duration::from_millis(TIMEOUT_MS))
    }
}

impl<const API_PORT: u16, const TIMEOUT_MS: u64> Default for ApiConfig<API_PORT, TIMEOUT_MS> {
    fn default() -> Self {
        Self {
            bind_endpoint: "127.0.0.1".to_string(),
            port_override: None,
            timeout_override: None,
            max_request_size: 10 * 1024 * 1024, // 10MB
            cors_enabled: false,
            cors_origins: vec!["*".to_string()],
            rate_limiting: RateLimitingConfig::default(),
            middleware: Vec::new(),
            tls: TlsConfig::default(),
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window_seconds: u64,
}
/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    pub enabled: bool,
    pub verify_client: bool,
}
// ==================== SECTION ====================

/// **CONST GENERIC SERVER CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig<const MAX_CONNECTIONS: usize = 1000> {
    pub bind_endpoint: String,
    pub max_connections_override: Option<usize>, // Runtime override for MAX_CONNECTIONS
    pub keep_alive: bool,
    pub connection_timeout: Duration,
    pub tls_enabled: bool,
    pub worker_threads: Option<usize>,
    
    // **CONSOLIDATED**: HTTP server configuration
    pub max_request_size: u64,
    pub keep_alive_timeout: Duration,
    pub request_timeout: Duration,
    
    // **CONSOLIDATED**: Streaming configuration
    pub streaming_buffer_size: usize,
    pub streaming_max_connections: u32,
    pub streaming_heartbeat_interval: Duration,
    
    // **CONSOLIDATED**: Rate limiting configuration
    pub rate_limiting_enabled: bool,
    pub rate_limiting_requests_per_minute: u32,
    pub rate_limiting_burst_size: u32,
}
impl<const MAX_CONNECTIONS: usize> Default for ServerConfig<MAX_CONNECTIONS> {
    fn default() -> Self {
        Self {
            bind_endpoint: crate::constants::DEFAULT_BIND_ADDRESS.to_string(),
            max_connections_override: None,
            keep_alive: true,
            connection_timeout: Duration::from_secs(30),
            tls_enabled: false,
            worker_threads: None,
            
            // **CONSOLIDATED**: HTTP server defaults
            max_request_size: 10 * 1024 * 1024, // 10MB
            keep_alive_timeout: Duration::from_secs(60),
            request_timeout: Duration::from_secs(30),
            
            // **CONSOLIDATED**: Streaming defaults
            streaming_buffer_size: 65536, // 64KB
            streaming_max_connections: 1000,
            streaming_heartbeat_interval: Duration::from_secs(30),
            
            // **CONSOLIDATED**: Rate limiting defaults
            rate_limiting_enabled: false,
            rate_limiting_requests_per_minute: 100,
            rate_limiting_burst_size: 10,
        }
    }
}

impl<const MAX_CONNECTIONS: usize> ServerConfig<MAX_CONNECTIONS> {
    /// Get max connections - compile-time optimized
    pub const fn max_connections() -> usize {
        MAX_CONNECTIONS
    }

    /// Get effective max connections (runtime override or compile-time default)
    pub const fn effective_max_connections(&self) -> usize {
        self.max_connections_override.unwrap_or(MAX_CONNECTIONS)
    }
}

// ==================== SECTION ====================

/// Storage and ZFS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageConfig {
    /// Default storage backend
    pub backend: StorageBackend,
    /// ZFS-specific configuration
    pub zfs: ZfsConfig,
    /// Storage tiers configuration
    pub tiers: TierConfig,
    /// Backup configuration
    pub backup: BackupConfig,
    /// Compression settings
    pub compression: CompressionConfig,
    /// Replication settings
    pub replication: ReplicationConfig,
    /// Cache configuration
    pub cache: CacheConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum StorageBackend {
    #[default]
    Zfs,
    Filesystem,
    ObjectStorage,
    Memory,
}

/// ZFS configuration (consolidated from multiple ZFS config definitions)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsConfig {
    /// ZFS pool configuration
    pub pools: Vec<String>,
    /// Dataset configuration
    pub datasets: Vec<String>,
    /// Snapshot configuration
    pub snapshots: SnapshotConfig,
    /// Performance tuning
    pub performance: ZfsPerformanceConfig,
    /// Compression algorithm
    pub compression: String,
    /// Enable deduplication
    pub deduplication: bool,
}
/// ZFS snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapshotConfig {
    /// Enable automatic snapshots
    pub auto_snapshot: bool,
    /// Snapshot interval in seconds
    pub interval_seconds: u64,
    /// Retention policy in days
    pub retention_days: u32,
}
/// ZFS performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {
    /// Record size
    pub recordsize: String,
    /// ATime enabled
    pub atime: bool,
    /// Sync mode
    pub sync: String,
    /// Cache mode
    pub cache_mode: String,
}
/// Storage tier configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TierConfig {
    /// Tier name
    pub name: String,
    /// Storage path
    /// Maximum size in bytes
    pub max_size_bytes: u64,
    /// Compression level
    pub compression_level: u8,
}
/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupConfig {
    /// Enable backups
    pub enabled: bool,
    /// Backup interval in seconds
    pub interval_seconds: u64,
    /// Backup retention in days
    pub retention_days: u32,
    /// Backup destination
    pub destination: String,
}
/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompressionConfig {
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level
    pub level: u8,
    /// Enable compression
    pub enabled: bool,
}
/// Replication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplicationConfig {
    /// Enable replication
    pub enabled: bool,
    /// Replication targets
    pub targets: Vec<String>,
    /// Replication interval in seconds
    pub interval_seconds: u64,
}
// ==================== SECTION ====================

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    /// Service discovery configuration
    pub discovery: DiscoveryConfig,
    /// Load balancer configuration
    pub load_balancer: LoadBalancerConfig,
    /// Internal network configuration
    pub internal: InternalNetworkConfig,
    /// External network configuration
    pub external: ExternalNetworkConfig,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Connection pool configuration
    pub connection_pool: ConnectionPoolConfig,
}
/// Network discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery interval in seconds
    pub interval_seconds: u64,
    /// Discovery timeout in seconds
    pub timeout_seconds: u64,
    /// Discovery protocols
    pub protocols: Vec<String>,
}
/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancerConfig {
    /// Load balancing strategy
    pub strategy: String,
    /// Health check configuration
    pub health_check_interval: u64,
    /// Maximum connections
    pub max_connections: usize,
}
/// Internal network configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InternalNetworkConfig {
    /// Internal network CIDR
    pub cidr: String,
    /// Internal ports
    pub ports: Vec<u16>,
    /// Enable internal TLS
    pub tls_enabled: bool,
}
/// External network configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalNetworkConfig {
    /// External host
    pub host: String,
    /// External port
    pub port: u16,
    /// Enable external TLS
    pub tls_enabled: bool,
}
/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    /// Failure threshold
    pub failure_threshold: u32,
    /// Reset timeout in seconds
    pub reset_timeout_seconds: u64,
    /// Enable circuit breaker
    pub enabled: bool,
}
/// Connection pool configuration  
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionPoolConfig {
    pub max_connections: usize,
    pub min_connections: usize,
    pub max_size: usize,
    pub connection_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}
// ==================== SECTION ====================

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// Authentication configuration
    pub authentication: AuthenticationConfig,
    /// Session configuration
    pub session: SessionConfig,
    /// Authorization configuration
    pub authorization: AuthorizationConfig,
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    /// Key management configuration
    pub key_management: KeyManagementConfig,
    /// Encryption at rest configuration
    pub encryption_at_rest: EncryptionAtRestConfig,
}
/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    /// Enable authentication
    pub enabled: bool,
    /// Authentication method
    pub method: String,
    /// Token expiration in seconds
    pub token_expiration_seconds: u64,
    /// Enable multi-factor authentication
    pub mfa_enabled: bool,
}
/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    /// Session timeout in seconds
    pub timeout_seconds: u64,
    /// Enable secure cookies
    pub secure_cookies: bool,
    /// Session storage type
    pub storage_type: String,
}
/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    /// Enable role-based access control
    pub rbac_enabled: bool,
    /// Default role
    pub default_role: String,
    /// Authorization rules
    pub rules: Vec<String>,
}
/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: String,
    /// Key size in bits
    pub key_size: u32,
    /// Enable encryption at rest
    pub at_rest_enabled: bool,
    /// Enable encryption in transit
    pub in_transit_enabled: bool,
}
/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyManagementConfig {
    /// Key rotation interval in seconds
    pub rotation_interval_seconds: u64,
    /// Key storage type
    pub storage_type: String,
    /// Enable hardware security module
    pub hsm_enabled: bool,
}
/// Encryption at rest configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionAtRestConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_rotation_enabled: bool,
    pub key_rotation_interval_hours: u64,
}
// ==================== SECTION ====================

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Tracing configuration
    pub tracing: TracingConfig,
    /// Alerting configuration
    pub alerting: AlertingConfig,
    /// Health check configuration
    pub health_checks: HealthCheckConfig,
}
/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Collection interval in seconds
    pub collection_interval_seconds: u64,
    /// Retention period in days
    pub retention_days: u32,
}
/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: String,
    /// Log output destination
    pub output: String,
    /// Enable structured logging
    pub structured: bool,
}
/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    /// Tracing endpoint
    pub endpoint: String,
    /// Sampling rate
    pub sampling_rate: f64,
    /// Service name for tracing
    pub service_name: String,
}
/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert channels
    pub channels: Vec<String>,
    /// Alert thresholds
    pub thresholds: HashMap<String, f64>,
    /// Notification interval in seconds
    pub notification_interval_seconds: u64,
}
/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval in seconds
    pub interval_seconds: u64,
    /// Health check timeout in seconds
    pub timeout_seconds: u64,
    /// Health check endpoints
    pub endpoints: Vec<String>,
}
// ==================== SECTION ====================

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    /// Environment name
    pub name: String,
    /// Environment variables
    pub variables: HashMap<String, String>,
    /// Configuration overrides
    pub overrides: HashMap<String, String>,
    /// Feature flags
    pub feature_flags: HashMap<String, bool>,
}
// ==================== SECTION ====================

/// MCP (Model Context Protocol) configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpConfig {
    /// Enable MCP
    pub enabled: bool,
    /// MCP server configuration
    pub server: McpServerConfig,
    /// MCP client configuration
    pub client: McpClientConfig,
    /// Protocol configuration
    pub protocol: McpProtocolConfig,
}
/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout
    pub timeout_seconds: u64,
}
/// MCP client configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpClientConfig {
    /// Client timeout
    pub timeout_seconds: u64,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Retry delay
    pub retry_delay_seconds: u64,
}
/// MCP protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpProtocolConfig {
    /// Protocol version
    pub version: String,
    /// Enable compression
    pub compression: bool,
    /// Buffer size
    pub buffer_size: usize,
} 