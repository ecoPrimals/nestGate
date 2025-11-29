// **CONSOLIDATED DOMAIN CONFIGURATIONS**
//! Consolidated Domains functionality and utilities.
// This module provides the unified domain configuration structures that consolidate
//! all scattered Config structs across the `NestGate` ecosystem into a single,
//! canonical system.
//! Consolidated Domains functionality and utilities.
// **CONSOLIDATES**:
//! - 100+ scattered Config structs across all crates
//! - Domain-specific configuration fragments
//! - Integration-specific configurations
//! - Test and development configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::constants::{DEFAULT_API_PORT, LOCALHOST};
use crate::error::Result;

// ==================== CORE DOMAIN CONFIGURATIONS ====================

/// **CONSOLIDATED DOMAIN CONFIGURATIONS**
///
/// This structure brings together all domain-specific configurations
/// under a single, well-organized hierarchy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Consolidateddomainconfigs
pub struct ConsolidatedDomainConfigs {
    /// ZFS storage management configuration
    pub zfs: ZfsDomainConfig,

    /// API and HTTP services configuration
    pub api: ApiDomainConfig,

    /// MCP protocol handling configuration
    pub mcp: McpDomainConfig,

    /// Network orchestration configuration
    pub network_services: NetworkServicesDomainConfig,

    /// Automation and workflows configuration
    pub automation: AutomationDomainConfig,

    /// File system monitoring configuration
    pub fsmonitor: FsMonitorDomainConfig,

    /// Installation and deployment configuration
    pub installer: InstallerDomainConfig,

    /// Performance monitoring and optimization
    pub performance: PerformanceDomainConfig,

    /// Binary and executable configuration
    pub binary: BinaryDomainConfig,
}

// ==================== ZFS DOMAIN CONFIGURATION ====================

/// **ZFS DOMAIN CONFIGURATION**
///
/// Consolidates all ZFS-related configurations:
/// - `PoolConfig`, `DatasetConfig`, `SnapshotConfig`
/// - `ZfsServiceConfig`, `ZfsHandlerConfig`
/// - Performance and monitoring configurations
///
/// Configuration for ZfsDomain
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDomainConfig {
    /// Pool management configuration
    pub pools: ZfsPoolsConfig,

    /// Dataset management configuration
    pub datasets: ZfsDatasetsConfig,

    /// Snapshot management configuration
    pub snapshots: ZfsSnapshotsConfig,

    /// ZFS service configuration
    pub service: ZfsServiceConfig,

    /// Performance optimization configuration
    pub performance: ZfsPerformanceConfig,

    /// Monitoring and metrics configuration
    pub monitoring: ZfsMonitoringConfig,

    /// Failover and redundancy configuration
    pub failover: ZfsFailoverConfig,
}

/// ZFS pools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsPools
pub struct ZfsPoolsConfig {
    /// Default pool settings
    pub default_pool: String,

    /// Pool-specific configurations
    pub pool_configs: HashMap<String, ZfsPoolConfig>,

    /// Auto-creation settings
    pub auto_create: bool,

    /// Scrub scheduling
    pub scrub_schedule: Option<String>,
}

/// Individual ZFS pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsPool
pub struct ZfsPoolConfig {
    /// Pool name
    pub name: String,

    /// Pool size limits
    pub size_limits: ZfsSizeLimits,

    /// Compression settings
    pub compression: ZfsCompressionConfig,

    /// Deduplication settings
    pub deduplication: bool,

    /// Custom properties
    pub properties: HashMap<String, String>,
}

/// ZFS datasets configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsDatasets
pub struct ZfsDatasetsConfig {
    /// Default dataset settings
    pub defaults: ZfsDatasetDefaults,

    /// Dataset-specific configurations
    pub dataset_configs: HashMap<String, ZfsDatasetConfig>,

    /// Auto-snapshot settings
    pub auto_snapshot: ZfsAutoSnapshotConfig,
}

/// ZFS snapshots configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsSnapshots
pub struct ZfsSnapshotsConfig {
    /// Retention policies
    pub retention: ZfsRetentionConfig,

    /// Snapshot naming patterns
    pub naming_pattern: String,

    /// Automatic snapshot intervals
    pub intervals: ZfsSnapshotIntervals,

    /// Cleanup policies
    pub cleanup: ZfsCleanupConfig,
}

// ==================== API DOMAIN CONFIGURATION ====================

/// **API DOMAIN CONFIGURATION**
///
/// Consolidates all API-related configurations:
/// - `UnifiedApiConfig`, `ApiServerConfig`
/// - Handler-specific configurations
/// - Performance and security settings
///
/// Configuration for ApiDomain
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiDomainConfig {
    /// HTTP server configuration
    pub server: ApiServerConfig,

    /// Handler configurations
    pub handlers: ConsolidatedApiHandlersConfig,

    /// Security configuration
    pub security: ApiSecurityConfig,

    /// Performance configuration
    pub performance: ApiPerformanceConfig,

    /// Monitoring configuration
    pub monitoring: ApiMonitoringConfig,

    /// Rate limiting configuration
    pub rate_limiting: ApiRateLimitingConfig,

    /// CORS configuration
    pub cors: ApiCorsConfig,
}

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ApiServer
pub struct ApiServerConfig {
    /// Server bind address
    pub bind_address: String,

    /// Server port
    pub port: u16,

    /// Worker thread count
    pub workers: Option<usize>,

    /// Request timeout
    pub request_timeout: Duration,

    /// Maximum request body size
    pub max_request_size: usize,

    /// Keep-alive timeout
    pub keep_alive: Duration,
}

/// Consolidated API handlers configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ConsolidatedApiHandlers
pub struct ConsolidatedApiHandlersConfig {
    /// ZFS handler configuration
    pub zfs: ZfsHandlerConfig,

    /// Performance handler configuration
    pub performance: PerformanceHandlerConfig,

    /// Dashboard handler configuration
    pub dashboard: DashboardHandlerConfig,

    /// Load testing handler configuration
    pub load_testing: LoadTestingHandlerConfig,

    /// Authentication handler configuration
    pub auth: AuthHandlerConfig,

    /// Workspace handler configuration
    pub workspace: WorkspaceHandlerConfig,
}

// ==================== MCP DOMAIN CONFIGURATION ====================

/// **MCP DOMAIN CONFIGURATION**
///
/// Consolidates all MCP protocol configurations:
/// - Protocol settings, streaming configuration
/// - Connection management, security settings
///
/// Configuration for McpDomain
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpDomainConfig {
    /// Protocol configuration
    pub protocol: McpProtocolConfig,

    /// Streaming configuration
    pub streaming: McpStreamingConfig,

    /// Connection configuration
    pub connections: McpConnectionConfig,

    /// Security configuration
    pub security: McpSecurityConfig,

    /// Performance configuration
    pub performance: McpPerformanceConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Model Context Protocol (MCP) integration
pub struct McpProtocolConfig {
    /// Protocol version
    pub version: String,

    /// Message format
    pub message_format: McpMessageFormat,

    /// Compression enabled
    pub compression: bool,

    /// Heartbeat interval
    pub heartbeat_interval: Duration,

    /// Timeout settings
    pub timeouts: McpTimeoutConfig,
}

// ==================== INTEGRATION CONFIGURATIONS ====================

/// **INTEGRATION CONFIGURATIONS**
///
/// Consolidates all external service and protocol integrations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Consolidatedintegrationconfigs
pub struct ConsolidatedIntegrationConfigs {
    /// External service integrations
    pub external_services: HashMap<String, ExternalServiceConfig>,

    /// Protocol-specific configurations
    pub protocols: ProtocolConfigs,

    /// Ecosystem integrations (Management, etc.)
    pub ecosystem: EcosystemConfig,

    /// Development and testing integrations
    pub development: DevelopmentIntegrationConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for external service integrations
pub struct ExternalServiceConfig {
    /// Service endpoint URL
    pub endpoint: String,

    /// Authentication configuration
    pub auth: ExternalAuthConfig,

    /// Connection settings
    pub connection: ExternalConnectionConfig,

    /// Retry configuration
    pub retry: ExternalRetryConfig,

    /// Health check configuration
    pub health_check: ExternalHealthCheckConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Container for all protocol-specific configurations
pub struct ProtocolConfigs {
    /// HTTP/HTTPS configuration
    pub http: HttpProtocolConfig,

    /// gRPC configuration
    pub grpc: GrpcProtocolConfig,

    /// WebSocket configuration
    pub websocket: WebSocketProtocolConfig,

    /// Custom protocol configurations
    pub custom: HashMap<String, CustomProtocolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ecosystem-wide settings and integrations
pub struct EcosystemConfig {
    /// Management integration settings
    pub management: BiomeOsIntegrationConfig,

    /// Primal ecosystem settings
    pub primal_ecosystem: PrimalEcosystemConfig,

    /// Service discovery settings
    pub service_discovery: ServiceDiscoveryConfig,

    /// Capability-based routing
    pub capability_routing: CapabilityRoutingConfig,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for ApiServerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            bind_address: LOCALHOST.to_string(),
            port: DEFAULT_API_PORT,
            workers: None, // Auto-detect
            request_timeout: Duration::from_secs(30),
            max_request_size: 10 * 1024 * 1024, // 10MB
            keep_alive: Duration::from_secs(75),
        }
    }
}

// ==================== VALIDATION FRAMEWORK ====================

/// Configuration validation trait for all domain configs
pub trait DomainConfigValidation {
    /// Validate the configuration for correctness
    fn validate(&self) -> Result<Vec<String>>;

    /// Validate for a specific environment
    fn validate_for_environment(&self, env: &str) -> Result<()>;

    /// Get required configuration fields
    fn required_fields() -> Vec<&'static str>;

    /// Get optional configuration fields
    fn optional_fields() -> Vec<&'static str>;
}
impl DomainConfigValidation for ConsolidatedDomainConfigs {
    /// Validates data
    fn validate(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate each domain configuration
        warnings.extend(self.zfs.validate()?);
        warnings.extend(self.api.validate()?);
        warnings.extend(self.mcp.validate()?);
        warnings.extend(self.network_services.validate()?);
        warnings.extend(self.automation.validate()?);
        warnings.extend(self.fsmonitor.validate()?);
        warnings.extend(self.installer.validate()?);
        warnings.extend(self.performance.validate()?);
        warnings.extend(self.binary.validate()?);

        Ok(warnings)
    }

    /// Validates  For Environment
    fn validate_for_environment(&self, env: &str) -> Result<()> {
        // Environment-specific validation logic
        match env {
            "production" => {
                // Stricter validation for production
                if self.api.server.port == DEFAULT_API_PORT {
                    return Err(crate::error::NestGateError::configuration_error_detailed(
                        "api.server.port".to_string(),
                        "Port 8080 not recommended for production".to_string(),
                        Some("8080".to_string()),
                        Some("443 or custom port".to_string()),
                        true,
                    ));
                }
            }
            "development" => {
                // More lenient validation for development
            }
            _ => {
                // Default validation
            }
        }

        Ok(())
    }

    /// Required Fields
    fn required_fields() -> Vec<&'static str> {
        vec![
            "zfs.pools.default_pool",
            "api.server.bind_address",
            "api.server.port",
        ]
    }

    /// Optional Fields
    fn optional_fields() -> Vec<&'static str> {
        vec![
            "zfs.performance.optimization_level",
            "api.handlers.custom_handlers",
            "mcp.streaming.buffer_size",
        ]
    }
}

// ==================== MIGRATION UTILITIES ====================

/// Utilities for migrating from old configuration structures
pub mod migration {
    use super::{ApiDomainConfig, ConsolidatedDomainConfigs, HashMap, Result, ZfsDomainConfig};

    /// Migrate from legacy configuration structures
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn migrate_from_legacy(
        legacy_configs: HashMap<String, serde_json::Value>,
    ) -> Result<ConsolidatedDomainConfigs> {
        let mut domains = ConsolidatedDomainConfigs::default();

        // Migrate ZFS configurations
        if let Some(zfs_config) = legacy_configs.get("zfs") {
            domains.zfs = migrate_zfs_config(zfs_config)?;
        }

        // Migrate API configurations
        if let Some(api_config) = legacy_configs.get("api") {
            domains.api = migrate_api_config(api_config)?;
        }

        // Add more migration logic as needed

        Ok(domains)
    }

    /// Migrate Zfs Config
    fn migrate_zfs_config(_legacy: &serde_json::Value) -> Result<ZfsDomainConfig> {
        // Implementation for ZFS config migration
        Ok(ZfsDomainConfig::default())
    }

    /// Migrate Api Config
    fn migrate_api_config(_legacy: &serde_json::Value) -> Result<ApiDomainConfig> {
        // Implementation for API config migration
        Ok(ApiDomainConfig::default())
    }
}
// ==================== TYPE STUBS FOR COMPILATION ====================
// These will be properly implemented as we migrate each domain

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Size limits for ZFS operations
pub struct ZfsSizeLimits {
    /// Minimum size in bytes
    pub min_size: u64,
    /// Maximum size in bytes
    pub max_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZFS compression settings
pub struct ZfsCompressionConfig {
    /// Compression algorithm (e.g., "lz4", "zstd")
    pub algorithm: String,
    /// Compression level (0-9)
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Default settings for ZFS datasets
pub struct ZfsDatasetDefaults {
    /// Default quota in bytes
    pub quota: Option<u64>,
    /// Default reservation in bytes
    pub reservation: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for a specific ZFS dataset
pub struct ZfsDatasetConfig {
    /// Dataset name
    pub name: String,
    /// Quota in bytes
    pub quota: Option<u64>,
}

/// Configuration for ZFS automatic snapshot scheduling
///
/// Controls whether automatic snapshots are taken and at what interval.
/// Snapshots provide point-in-time recovery capabilities for ZFS datasets.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsAutoSnapshot
pub struct ZfsAutoSnapshotConfig {
    /// Enable or disable automatic snapshots
    pub enabled: bool,
    /// Interval between automatic snapshots
    pub interval: Duration,
}

/// Configuration for ZFS snapshot retention policies
///
/// Defines how many snapshots to retain at each time interval.
/// Older snapshots beyond these limits are automatically cleaned up.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsRetention
pub struct ZfsRetentionConfig {
    /// Number of daily snapshots to retain
    pub daily: u32,
    /// Number of weekly snapshots to retain
    pub weekly: u32,
    /// Number of monthly snapshots to retain
    pub monthly: u32,
}

/// Configuration for ZFS snapshot interval scheduling
///
/// Controls which snapshot intervals are enabled. Multiple intervals
/// can be active simultaneously for comprehensive backup coverage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Zfssnapshotintervals
pub struct ZfsSnapshotIntervals {
    /// Enable hourly snapshots
    pub hourly: bool,
    /// Enable daily snapshots
    pub daily: bool,
    /// Enable weekly snapshots
    pub weekly: bool,
}

/// Configuration for ZFS automatic cleanup behavior
///
/// Controls when and how ZFS performs automatic cleanup of old snapshots
/// and other temporary data to prevent disk space exhaustion.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsCleanup
pub struct ZfsCleanupConfig {
    /// Enable automatic cleanup of old snapshots
    pub auto_cleanup: bool,
    /// Disk usage threshold (0.0-1.0) that triggers cleanup
    pub cleanup_threshold: f64,
}

// Additional type stubs - these will be expanded as we implement each domain

/// Configuration for ZFS service-level settings
///
/// Placeholder for future ZFS service configuration options such as
/// service discovery, health checks, and service-specific parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsService
pub struct ZfsServiceConfig {}

/// Configuration for ZFS performance tuning
///
/// Placeholder for future ZFS performance configuration options such as
/// cache sizes, I/O scheduling, and prefetch behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsPerformance
pub struct ZfsPerformanceConfig {}

/// Configuration for ZFS monitoring and metrics
///
/// Placeholder for future ZFS monitoring configuration options such as
/// metrics collection intervals, alert thresholds, and logging levels.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsMonitoring
pub struct ZfsMonitoringConfig {}

/// Configuration for ZFS high-availability and failover
///
/// Placeholder for future ZFS failover configuration options such as
/// replication targets, failover policies, and health check intervals.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsFailover
pub struct ZfsFailoverConfig {}

// ApiHandlersConfig is already defined above - removing duplicate

/// Configuration for API security settings
///
/// Placeholder for future API security configuration such as authentication,
/// authorization, API keys, and security headers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ApiSecurity
pub struct ApiSecurityConfig {}

/// Configuration for API performance tuning
///
/// Placeholder for future API performance configuration such as connection
/// pooling, request timeouts, and caching strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ApiPerformance
pub struct ApiPerformanceConfig {}

/// Configuration for API monitoring and observability
///
/// Placeholder for future API monitoring configuration such as request logging,
/// metrics collection, and distributed tracing.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ApiMonitoring
pub struct ApiMonitoringConfig {}

/// Configuration for API rate limiting
///
/// Placeholder for future API rate limiting configuration such as request
/// limits per IP, token bucket parameters, and burst allowances.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ApiRateLimiting
pub struct ApiRateLimitingConfig {}

/// Configuration for Cross-Origin Resource Sharing (CORS)
///
/// Placeholder for future CORS configuration such as allowed origins,
/// methods, headers, and credentials handling.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ApiCors
pub struct ApiCorsConfig {}

/// Configuration for Model Context Protocol streaming behavior
///
/// Placeholder for future MCP streaming configuration such as buffer sizes,
/// flow control, and backpressure handling.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for McpStreaming
pub struct McpStreamingConfig {}

/// Configuration for Model Context Protocol connections
///
/// Placeholder for future MCP connection configuration such as timeouts,
/// keep-alive settings, and reconnection policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for McpConnection
pub struct McpConnectionConfig {}

/// Configuration for Model Context Protocol security
///
/// Placeholder for future MCP security configuration such as encryption,
/// authentication, and authorization for MCP connections.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for McpSecurity
pub struct McpSecurityConfig {}

/// Configuration for Model Context Protocol performance
///
/// Placeholder for future MCP performance configuration such as parallelism,
/// batching, and resource limits.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for McpPerformance
pub struct McpPerformanceConfig {}

/// Configuration for network services domain
///
/// Placeholder for future network services configuration such as service
/// discovery, load balancing, and network topology.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for NetworkServicesDomain
pub struct NetworkServicesDomainConfig {}

/// Configuration for automation domain
///
/// Placeholder for future automation configuration such as scheduled tasks,
/// workflow automation, and event-driven automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for AutomationDomain
pub struct AutomationDomainConfig {}

/// Configuration for filesystem monitoring domain
///
/// Placeholder for future filesystem monitoring configuration such as watch
/// paths, polling intervals, and change detection settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for FsMonitorDomain
pub struct FsMonitorDomainConfig {}

/// Configuration for installer domain
///
/// Placeholder for future installer configuration such as installation paths,
/// dependency management, and post-install hooks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for InstallerDomain
pub struct InstallerDomainConfig {}

/// Configuration for performance domain
///
/// Placeholder for future performance domain configuration such as profiling,
/// benchmarking, and performance optimization settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PerformanceDomain
pub struct PerformanceDomainConfig {}

/// Configuration for binary execution domain
///
/// Placeholder for future binary domain configuration such as executable paths,
/// environment variables, and execution policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for BinaryDomain
pub struct BinaryDomainConfig {}

// Additional implementation stubs
impl DomainConfigValidation for ZfsDomainConfig {
    /// Validates data
    fn validate(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
    /// Validates  For Environment
    fn validate_for_environment(&self, _env: &str) -> Result<()> {
        Ok(())
    }
    /// Required Fields
    fn required_fields() -> Vec<&'static str> {
        vec![]
    }
    /// Optional Fields
    fn optional_fields() -> Vec<&'static str> {
        vec![]
    }
}

impl DomainConfigValidation for ApiDomainConfig {
    /// Validates data
    fn validate(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
    /// Validates  For Environment
    fn validate_for_environment(&self, _env: &str) -> Result<()> {
        Ok(())
    }
    /// Required Fields
    fn required_fields() -> Vec<&'static str> {
        vec![]
    }
    /// Optional Fields
    fn optional_fields() -> Vec<&'static str> {
        vec![]
    }
}

// Implement for other domain configs...
macro_rules! impl_domain_validation {
    ($type:ty) => {
        impl DomainConfigValidation for $type {
            /// Validates data
            fn validate(&self) -> Result<Vec<String>> {
                Ok(vec![])
            }
            /// Validates  For Environment
            fn validate_for_environment(&self, _env: &str) -> Result<()> {
                Ok(())
            }
            /// Required Fields
            fn required_fields() -> Vec<&'static str> {
                vec![]
            }
            /// Optional Fields
            fn optional_fields() -> Vec<&'static str> {
                vec![]
            }
        }
    };
}

impl_domain_validation!(McpDomainConfig);
impl_domain_validation!(NetworkServicesDomainConfig);
impl_domain_validation!(AutomationDomainConfig);
impl_domain_validation!(FsMonitorDomainConfig);
impl_domain_validation!(InstallerDomainConfig);
impl_domain_validation!(PerformanceDomainConfig);
impl_domain_validation!(BinaryDomainConfig);

// ==================== MISSING DEFAULT IMPLEMENTATIONS ====================

impl Default for ZfsPoolsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_pool: "tank".to_string(),
            pool_configs: HashMap::new(),
            auto_create: false,
            scrub_schedule: Some("0 2 * * 0".to_string()), // Weekly at 2 AM
        }
    }
}

impl Default for ZfsSnapshotsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            retention: ZfsRetentionConfig::default(),
            naming_pattern: "%Y%m%d_%H%M%S".to_string(),
            intervals: ZfsSnapshotIntervals::default(),
            cleanup: ZfsCleanupConfig::default(),
        }
    }
}

impl Default for McpProtocolConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            message_format: McpMessageFormat::Json,
            compression: false,
            heartbeat_interval: Duration::from_secs(30),
            timeouts: McpTimeoutConfig::default(),
        }
    }
}

// More type stubs for compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mcpmessageformat
pub enum McpMessageFormat {
    /// Json
    Json,
    /// Messagepack
    MessagePack,
    /// Protobuf
    Protobuf,
}

impl Default for McpMessageFormat {
    /// Returns the default instance
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for McpTimeout
pub struct McpTimeoutConfig {
    /// Connect
    pub connect: Duration,
    /// Read
    pub read: Duration,
    /// Write
    pub write: Duration,
}

impl Default for McpTimeoutConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            connect: Duration::from_secs(5),
            read: Duration::from_secs(30),
            write: Duration::from_secs(30),
        }
    }
}

// McpTimeoutConfig already has Default derived - removing duplicate implementation

// More implementation stubs...
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ExternalAuth
pub struct ExternalAuthConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ExternalConnection
pub struct ExternalConnectionConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ExternalRetry
pub struct ExternalRetryConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ExternalHealthCheck
pub struct ExternalHealthCheckConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for HttpProtocol
pub struct HttpProtocolConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for GrpcProtocol
pub struct GrpcProtocolConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for WebSocketProtocol
pub struct WebSocketProtocolConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CustomProtocol
pub struct CustomProtocolConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for BiomeOsIntegration
pub struct BiomeOsIntegrationConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PrimalEcosystem
pub struct PrimalEcosystemConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ServiceDiscovery
pub struct ServiceDiscoveryConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CapabilityRouting
pub struct CapabilityRoutingConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for DevelopmentIntegration
pub struct DevelopmentIntegrationConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsHandler
pub struct ZfsHandlerConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PerformanceHandler
pub struct PerformanceHandlerConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for DashboardHandler
pub struct DashboardHandlerConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for LoadTestingHandler
pub struct LoadTestingHandlerConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for AuthHandler
pub struct AuthHandlerConfig {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for WorkspaceHandler
pub struct WorkspaceHandlerConfig {}
