/// Consolidates all fragmented MCP configuration structs into a single,
/// comprehensive configuration system using the StandardDomainConfig pattern.
///
/// **ELIMINATES**:
/// - McpAdapterConfig (adapter.rs)
/// - McpSessionConfig (security.rs)
/// - VolumeConfig (storage.rs)
/// - Multiple fragmented MCP configs across modules
///
/// **PROVIDES**:
/// - Single source of truth for all MCP configuration
/// - Consistent configuration patterns with base unified configs
/// - Extensible architecture for MCP-specific settings
use nestgate_core::unified_final_config::supporting_types::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// **UNIFIED MCP EXTENSIONS**
/// Consolidates all MCP-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMcpExtensions {
    /// Protocol and communication settings
    pub protocol: McpProtocolSettings,
    /// Session and security settings
    pub session: McpSessionSettings,
    /// Storage and volume settings
    pub storage: McpStorageSettings,
    /// Adapter and bridging settings
    pub adapter: McpAdapterSettings,
    /// Performance and optimization settings
    pub performance: McpPerformanceSettings,
    /// Quality of Service settings
    pub qos: McpQosSettings,
    }
/// MCP protocol configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpProtocolSettings {
    /// MCP protocol version
    pub protocol_version: String,
    /// Enable protocol compression
    pub enable_compression: bool,
    /// Compression threshold for messages (bytes)
    pub compression_threshold: usize,
    /// Message format (json, binary, etc.)
    pub message_format: String,
    /// Custom protocol headers
    pub custom_headers: HashMap<String, String>,
    /// Enable protocol debugging
    pub enable_debug: bool,
    }
/// MCP session configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSessionSettings {
    /// Session timeout duration
    pub session_timeout: Duration,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: usize,
    /// Enable session persistence
    pub enable_persistence: bool,
    /// Session cleanup interval
    pub cleanup_interval: Duration,
    /// Authentication requirements
    pub require_authentication: bool,
    /// Session encryption settings
    pub encryption: McpSessionEncryption,
    }
/// MCP storage configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpStorageSettings {
    /// Default volume size (MB)
    pub default_volume_size_mb: u64,
    /// Maximum volume size (MB)
    pub max_volume_size_mb: u64,
    /// Enable volume encryption
    pub enable_volume_encryption: bool,
    /// Volume replication factor
    pub replication_factor: u32,
    /// Storage backend type
    pub backend_type: String,
    /// Volume mount options
    pub mount_options: Vec<String>,
    /// Enable volume snapshots
    pub enable_snapshots: bool,
    /// Snapshot retention policy
    pub snapshot_retention: McpSnapshotRetention,
    }
/// MCP adapter configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAdapterSettings {
    /// Enable protocol bridging
    pub enable_bridging: bool,
    /// Supported protocol versions
    pub supported_versions: Vec<String>,
    /// Adapter buffer size
    pub buffer_size: usize,
    /// Connection pool settings
    pub connection_pool: McpConnectionPool,
    /// Failover settings
    pub failover: McpFailoverSettings,
    }
/// MCP performance configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPerformanceSettings {
    /// Worker thread count
    pub worker_threads: usize,
    /// Message queue size
    pub message_queue_size: usize,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Performance metrics collection interval
    pub metrics_interval: Duration,
    /// Batch processing settings
    pub batch_processing: McpBatchSettings,
    }
/// MCP Quality of Service settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpQosSettings {
    /// Message prioritization enabled
    pub enable_prioritization: bool,
    /// Rate limiting settings
    pub rate_limiting: McpRateLimitSettings,
    /// Circuit breaker settings
    pub circuit_breaker: McpCircuitBreakerSettings,
    /// Load balancing strategy
    pub load_balancing_strategy: String,
    }
/// MCP session encryption settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSessionEncryption {
    /// Encryption algorithm
    pub algorithm: String,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
    /// Enable forward secrecy
    pub enable_forward_secrecy: bool,
    }
/// MCP snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSnapshotRetention {
    /// Maximum snapshots to keep
    pub max_snapshots: u32,
    /// Retention period (days)
    pub retention_days: u32,
    /// Enable automatic cleanup
    pub enable_auto_cleanup: bool,
    }
/// MCP connection pool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConnectionPool {
    /// Minimum connections
    pub min_connections: usize,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    }
/// MCP failover settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpFailoverSettings {
    /// Enable automatic failover
    pub enable_failover: bool,
    /// Failover timeout
    pub failover_timeout: Duration,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Retry delay
    pub retry_delay: Duration,
    }
/// MCP batch processing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpBatchSettings {
    /// Enable batch processing
    pub enable_batching: bool,
    /// Batch size
    pub batch_size: usize,
    /// Batch timeout
    pub batch_timeout: Duration,
    /// Maximum batch delay
    pub max_batch_delay: Duration,
    }
/// MCP rate limiting settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRateLimitSettings {
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Requests per second limit
    pub requests_per_second: u32,
    /// Burst capacity
    pub burst_capacity: u32,
    /// Rate limiting algorithm
    pub algorithm: String,
    }
/// MCP circuit breaker settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCircuitBreakerSettings {
    /// Enable circuit breaker
    pub enable_circuit_breaker: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Reset timeout
    pub reset_timeout: Duration,
    /// Half-open timeout
    pub half_open_timeout: Duration,
    }
impl Default for UnifiedMcpExtensions {
    fn default() -> Self { Self {
            protocol: McpProtocolSettings {
                protocol_version: "1.0".to_string(),
                enable_compression: true,
                compression_threshold: 1024, // 1KB
                message_format: "json".to_string(),
                custom_headers: HashMap::new(),
                enable_debug: false,
            , session: McpSessionSettings {
                session_timeout: Duration::from_secs(3600), // 1 hour
                max_concurrent_sessions: 1000,
                enable_persistence: true,
                cleanup_interval: Duration::from_secs(300), // 5 minutes
                require_authentication: true,
                encryption: McpSessionEncryption {
                    algorithm: "AES-256-GCM".to_string(),
                    key_rotation_interval: Duration::from_secs(86400), // 24 hours
                    enable_forward_secrecy: true }
            }
            storage: McpStorageSettings {
                default_volume_size_mb: 1024, // 1GB
                max_volume_size_mb: 10240,    // 10GB
                enable_volume_encryption: true,
                replication_factor: 3,
                backend_type: "zfs".to_string(),
                mount_options: vec!["rw".to_string(), "async".to_string()],
                enable_snapshots: true,
                snapshot_retention: McpSnapshotRetention {
                    max_snapshots: 30,
                    retention_days: 7,
                    enable_auto_cleanup: true,
                }
            }
            adapter: McpAdapterSettings {
                enable_bridging: true,
                supported_versions: vec!["1.0".to_string(), "0.9".to_string()],
                buffer_size: 8192,
                connection_pool: McpConnectionPool {
                    min_connections: 5,
                    max_connections: 100,
                    connection_timeout: Duration::from_secs(10),
                    idle_timeout: Duration::from_secs(300),
                }
                failover: McpFailoverSettings {
                    enable_failover: true,
                    failover_timeout: Duration::from_secs(30),
                    retry_attempts: 3,
                    retry_delay: Duration::from_secs(5),
                }
            }
            performance: McpPerformanceSettings {
                worker_threads: num_cpus::get(),
                message_queue_size: 10_000,
                enable_monitoring: true,
                metrics_interval: Duration::from_secs(60),
                batch_processing: McpBatchSettings {
                    enable_batching: true,
                    batch_size: 100,
                    batch_timeout: Duration::from_millis(100),
                    max_batch_delay: Duration::from_millis(500),
                }
            }
            qos: McpQosSettings {
                enable_prioritization: true,
                rate_limiting: McpRateLimitSettings {
                    enable_rate_limiting: true,
                    requests_per_second: 1000,
                    burst_capacity: 2000,
                    algorithm: "token_bucket".to_string(),
                }
                circuit_breaker: McpCircuitBreakerSettings {
                    enable_circuit_breaker: true,
                    failure_threshold: 5,
                    reset_timeout: Duration::from_secs(60),
                    half_open_timeout: Duration::from_secs(30),
                }
                load_balancing_strategy: "round_robin".to_string(),
            }
    }
    }
    }

impl UnifiedMcpExtensions {
    /// Create production-optimized MCP extensions
    #[must_use]
    pub fn production() -> Self { let mut config = Self::default();
        // Override specific settings for production
        config.performance.max_concurrent_requests = 1000;
        config.security.authentication_required = true;
        config.logging.log_level = "info".to_string();
        config
    , /// Create high-performance MCP extensions
    #[must_use]
    pub fn high_performance() -> Self {
        let mut config = Self::default();
        // Override specific settings for high performance
        config.performance.max_concurrent_requests = 5000;
        config.performance.connection_pool_size = 100;
        config.security.authentication_required = true;
        config.logging.log_level = "warn".to_string();
        config
     }
    }

/// **UNIFIED MCP CONFIGURATION**
/// Single configuration type that replaces all MCP config structs
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedMcpConfig = StandardDomainConfig;
impl UnifiedMcpConfig {
    /// Create development-focused MCP configuration
    #[must_use]
    pub fn development() -> Self { let mut config = StandardDomainConfig::new(UnifiedMcpExtensions::default());

        // Configure service settings for MCP development
        config.service.name = "nestgate-mcp".to_string();
        config.service.version = env!("CARGO_PKG_VERSION").to_string();
        config.service.description = "NestGate MCP Service".to_string();
        config.service.service_type = "mcp".to_string();
        config.service.environment = "development".to_string();

        // Development-friendly network settings
        config.network.port = 8087; // MCP default port
        config.network.bind_address = "127.0.0.1".to_string();
        config.network.enable_tls = false; // Dev uses plain connections
        config.network.max_connections = 100;

        // Development security settings
        config.security.require_auth = false; // Dev bypasses auth
        config.security.enable_tls = false;
        config.security.allowed_origins = vec!["*".to_string()];
        config.security.allowed_ip_ranges = vec!["127.0.0.0/8".to_string()];

        // Development MCP extensions configured through domain-specific settings
        // Session and protocol settings are now handled by the unified config system

        config
    , /// Create production-ready MCP configuration
    #[must_use]
    pub fn production() -> Self {
        let mut config = StandardDomainConfig::new(UnifiedMcpExtensions::default());

        // Configure service settings for MCP production
        config.service.name = "nestgate-mcp".to_string();
        config.service.version = env!("CARGO_PKG_VERSION").to_string();
        config.service.description = "NestGate MCP Service".to_string();
        config.service.service_type = "mcp".to_string();
        config.service.environment = "production".to_string();

        // Production network settings
        config.network.port = 8087;
        config.network.bind_address = "0.0.0.0".to_string();
        config.network.enable_tls = true; // Production requires TLS
        config.network.max_connections = 1000;

        // Production security settings
        config.security.require_auth = true;
        config.security.enable_tls = true;
        config.security.allowed_origins = vec![]; // Restrict origins in production
        config.security.allowed_ip_ranges = vec!["10.0.0.0/8".to_string()]; // Private networks only

        // Production MCP extensions configured through domain-specific settings
        // Security, performance, and QoS settings are now handled by the unified config system

        config
     }

    /// Create high-performance MCP configuration
    #[must_use]
    pub fn high_performance() -> Self { let mut config = Self::production();

        // High-performance optimizations
        // config.extensions.performance.worker_threads = num_cpus::get() * 2;
        // config.extensions.performance.message_queue_size = 50000;
        config
            .extensions
            .performance
            .batch_processing
            .enable_batching = true;
        // config.extensions.performance.batch_processing.batch_size = 500;

        // Optimize connection pool
        // config.extensions.adapter.connection_pool.min_connections = 20;
        // config.extensions.adapter.connection_pool.max_connections = 1000;

        // Optimize rate limiting
        // config.extensions.qos.rate_limiting.requests_per_second = 10_000;
        // config.extensions.qos.rate_limiting.burst_capacity = 20000;

        config
     }

/// **MIGRATION HELPERS**
/// Functions to migrate from legacy MCP configs to unified system
/// Migrate legacy McpAdapterConfig to unified system
pub fn migrate_adapter_config(legacy_config: crate::adapter::McpAdapterConfig) -> UnifiedMcpConfig {
    let mut unified = UnifiedMcpConfig::development();
    // Migrate base configuration
    unified.service = legacy_config.base.service;
    unified.network = legacy_config.base.network;
    unified.security = legacy_config.base.security;
    unified.monitoring = legacy_config.base.monitoring;

    // Migrate MCP-specific settings
    unified.extensions.protocol.protocol_version = legacy_config.mcp_extensions.protocol_version;
    unified.extensions.protocol.compression_threshold =
        legacy_config.mcp_extensions.compression_threshold;
    unified.extensions.protocol.custom_headers = legacy_config.mcp_extensions.custom_headers;

    unified
    }

/// Migrate legacy McpSessionConfig to unified system
pub fn migrate_session_config(
    legacy_session: crate::security::McpSessionConfig,
) -> UnifiedMcpConfig {
    let mut unified = UnifiedMcpConfig::development();
    // Extract session settings from legacy config and apply to unified extensions
    // This would involve mapping the legacy fields to the new structure
    // Implementation depends on the exact structure of McpSessionConfig

    unified
    }

/// Migrate legacy VolumeConfig to unified system  
pub fn migrate_volume_config(legacy_volume: crate::storage::VolumeConfig) -> UnifiedMcpConfig {
    let mut unified = UnifiedMcpConfig::development();
    // Extract storage settings from legacy config and apply to unified extensions
    // This would involve mapping the legacy fields to the new structure
    // Implementation depends on the exact structure of VolumeConfig

    unified
    }
