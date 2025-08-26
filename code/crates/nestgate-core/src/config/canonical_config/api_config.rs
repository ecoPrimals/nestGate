use std::collections::HashMap;
//
// API-related configuration including REST API, streaming, WebSocket,
// authentication handlers, and dashboard settings.
// 
// **CONSOLIDATES**:
// - UnifiedApiHandlerConfig (nestgate-api)
// - ZfsHandlerConfig and all ZFS-related configs
// - PerformanceHandlerConfig and analytics configs
// - DashboardHandlerConfig and UI configs
// - All scattered handler-specific configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// API configuration (consolidates 20+ API configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ApiConfig {
    /// REST API configuration
    pub rest: RestApiConfig,
    /// Streaming configuration
    pub streaming: StreamingConfig,
    /// Server-Sent Events configuration
    pub sse: SseConfig,
    /// WebSocket configuration
    pub websocket: WebSocketConfig,
    /// Authentication handlers
    pub auth_handlers: AuthHandlerConfig,
    /// Dashboard configuration
    pub dashboard: DashboardConfig,
    /// Load testing configuration
    pub load_testing: LoadTestingConfig,
    /// Workspace management configuration
    pub workspace: WorkspaceConfig,
    /// **CONSOLIDATED**: ZFS handler configurations (absorbs UnifiedApiHandlerConfig.zfs)
    pub zfs_handlers: ZfsHandlerConfig,
    /// **CONSOLIDATED**: Performance handler configurations (absorbs fragmented performance configs)
    pub performance_handlers: PerformanceHandlerConfig,
    /// **CONSOLIDATED**: Handler extensions (absorbs remaining scattered configs)
    pub handler_extensions: ApiHandlerExtensions,
}

// ==================== CONSOLIDATED HANDLER CONFIGURATIONS ====================

/// **CONSOLIDATED ZFS HANDLER CONFIGURATION**
/// Absorbs: UnifiedApiHandlerConfig.zfs, ZfsHandlerConfig from nestgate-api
/// Replaces: PoolConfig (3+ variants), DatasetConfig (2+ variants), SnapshotConfig, ZfsServiceConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ZfsHandlerConfig {
    /// Pool management settings
    pub pool: UnifiedPoolConfig,
    /// Dataset management settings  
    pub dataset: UnifiedDatasetConfig,
    /// Snapshot management settings
    pub snapshot: UnifiedSnapshotConfig,
    /// Service-level ZFS settings
    pub service: ZfsServiceSettings,
    /// Handler-specific performance settings
    pub performance: ZfsPerformanceConfig,
}

/// **UNIFIED POOL CONFIGURATION**
/// Consolidates: handlers/zfs/types.rs::PoolConfig, zero_cost_api_handlers.rs::PoolConfig, universal_zfs/types.rs::PoolConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPoolConfig {
    /// RAID level (mirror, raidz1, raidz2, raidz3)
    pub raid_level: Option<String>,
    /// Compression algorithm (lz4, gzip, zstd, etc.)
    pub compression: Option<String>,
    /// Deduplication enabled
    pub dedup: Option<bool>,
    /// Encryption enabled
    pub encryption: Option<bool>,
    /// Pool properties
    pub properties: HashMap<String, String>,
    /// Auto-expansion settings
    pub auto_expand: bool,
    /// Cache device settings
    pub cache_devices: Vec<String>,
    /// Log device settings
    pub log_devices: Vec<String>,
    /// Record size for optimal performance
    pub record_size: Option<String>,
    /// ARC size limits
    pub arc_max: Option<u64>,
    /// Scrub scheduling
    pub scrub_schedule: Option<String>,
}

/// **UNIFIED DATASET CONFIGURATION**
/// Consolidates: Multiple DatasetConfig variants across handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDatasetConfig {
    /// Default mount point
    pub mount_point: Option<String>,
    /// Default quota settings
    pub quota: Option<String>,
    /// Default reservation settings
    pub reservation: Option<String>,
    /// Dataset properties
    pub properties: HashMap<String, String>,
    /// Compression settings
    pub compression: Option<String>,
    /// Deduplication settings
    pub dedup: Option<bool>,
    /// Encryption settings
    pub encryption: Option<bool>,
}

/// **UNIFIED SNAPSHOT CONFIGURATION**
/// Consolidates: Multiple SnapshotConfig variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSnapshotConfig {
    /// Retention policy
    pub retention_policy: RetentionPolicy,
    /// Automatic snapshot scheduling
    pub auto_snapshot: bool,
    /// Snapshot naming pattern
    pub naming_pattern: String,
    /// Maximum snapshots to keep
    pub max_snapshots: u32,
    /// Snapshot compression
    pub compress_snapshots: bool,
}

/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Daily snapshots to keep
    pub daily: u32,
    /// Weekly snapshots to keep  
    pub weekly: u32,
    /// Monthly snapshots to keep
    pub monthly: u32,
    /// Yearly snapshots to keep
    pub yearly: u32,
}

/// ZFS service-level settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceSettings {
    /// Enable ZFS service
    pub enabled: bool,
    /// Service timeout
    pub timeout: Duration,
    /// Maximum concurrent operations
    pub max_concurrent_ops: u32,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Service endpoint
    pub endpoint: Option<String>,
}

/// ZFS performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceConfig {
    /// Enable performance monitoring
    pub monitoring_enabled: bool,
    /// Performance metrics collection interval
    pub metrics_interval: Duration,
    /// Cache size for operations
    pub operation_cache_size: u32,
    /// Batch size for bulk operations
    pub batch_size: u32,
}

/// **CONSOLIDATED PERFORMANCE HANDLER CONFIGURATION**
/// Absorbs: PerformanceHandlerConfig from nestgate-api, analytics configs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct PerformanceHandlerConfig {
    /// Analytics settings
    pub analytics: PerformanceAnalyticsConfig,
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    /// Dashboard integration settings
    pub dashboard_integration: DashboardIntegrationConfig,
    /// Load testing settings
    pub load_testing: LoadTestingHandlerConfig,
}

/// Performance analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalyticsConfig {
    /// Enable analytics
    pub enabled: bool,
    /// Data retention period
    pub retention_days: u32,
    /// Sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
    /// Storage backend for analytics data
    pub storage_backend: String,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Collection interval
    pub interval: Duration,
    /// Metrics storage path
    pub storage_path: String,
    /// Maximum metrics history
    pub max_history_entries: u32,
}

/// Dashboard integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardIntegrationConfig {
    /// Enable dashboard integration
    pub enabled: bool,
    /// Dashboard endpoint
    pub endpoint: String,
    /// Update interval
    pub update_interval: Duration,
    /// Authentication token
    pub auth_token: Option<String>,
}

/// Load testing handler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestingHandlerConfig {
    /// Enable load testing endpoints
    pub enabled: bool,
    /// Maximum concurrent test sessions
    pub max_concurrent_sessions: u32,
    /// Test duration limits
    pub max_test_duration: Duration,
    /// Resource limits for tests
    pub resource_limits: LoadTestResourceLimits,
}

/// Load test resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestResourceLimits {
    /// Maximum memory usage (MB)
    pub max_memory_mb: u32,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: u32,
    /// Maximum network bandwidth (Mbps)
    pub max_bandwidth_mbps: u32,
}

/// **API HANDLER EXTENSIONS**
/// Consolidates remaining handler-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ApiHandlerExtensions {
    /// Custom handler configurations
    pub custom_handlers: HashMap<String, serde_json::Value>,
    /// Feature flags for handlers
    pub feature_flags: HandlerFeatureFlags,
    /// Security settings for handlers
    pub security: HandlerSecurityConfig,
    /// Monitoring settings for handlers
    pub monitoring: HandlerMonitoringConfig,
}

/// Handler feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerFeatureFlags {
    /// Enable experimental handlers
    pub experimental_handlers: bool,
    /// Enable debug endpoints
    pub debug_endpoints: bool,
    /// Enable admin endpoints
    pub admin_endpoints: bool,
    /// Enable metrics endpoints
    pub metrics_endpoints: bool,
}

/// Handler security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerSecurityConfig {
    /// Enable authentication for all handlers
    pub require_auth: bool,
    /// Rate limiting settings
    pub rate_limiting: RateLimitingConfig,
    /// CORS settings
    pub cors: CorsConfig,
    /// Security headers
    pub security_headers: SecurityHeadersConfig,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
    /// Rate limit storage backend
    pub storage_backend: String,
}

/// Security headers configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersConfig {
    /// Enable security headers
    pub enabled: bool,
    /// Custom headers
    pub custom_headers: HashMap<String, String>,
    /// Content Security Policy
    pub csp: Option<String>,
}

/// Handler monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerMonitoringConfig {
    /// Enable handler monitoring
    pub enabled: bool,
    /// Monitoring interval
    pub interval: Duration,
    /// Health check settings
    pub health_checks: HealthCheckConfig,
    /// Alerting settings
    pub alerting: AlertingConfig,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Failure threshold
    pub failure_threshold: u32,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert channels
    pub channels: Vec<String>,
    /// Alert thresholds
    pub thresholds: HashMap<String, f64>,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================


impl Default for UnifiedPoolConfig {
    fn default() -> Self {
        Self {
            raid_level: Some("mirror".to_string()),
            compression: Some("lz4".to_string()),
            dedup: Some(false),
            encryption: Some(false),
            properties: HashMap::new(),
            auto_expand: false,
            cache_devices: Vec::new(),
            log_devices: Vec::new(),
            record_size: Some("128K".to_string()),
            arc_max: None,
            scrub_schedule: Some("0 2 * * 0".to_string()), // Weekly at 2 AM
        }
    }
}

impl Default for UnifiedDatasetConfig {
    fn default() -> Self {
        Self {
            mount_point: None,
            quota: None,
            reservation: None,
            properties: HashMap::new(),
            compression: Some("lz4".to_string()),
            dedup: Some(false),
            encryption: Some(false),
        }
    }
}

impl Default for UnifiedSnapshotConfig {
    fn default() -> Self {
        Self {
            retention_policy: RetentionPolicy::default(),
            auto_snapshot: true,
            naming_pattern: "auto-%Y%m%d-%H%M%S".to_string(),
            max_snapshots: 100,
            compress_snapshots: true,
        }
    }
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            daily: 7,
            weekly: 4,
            monthly: 12,
            yearly: 3,
        }
    }
}

impl Default for ZfsServiceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(30),
            max_concurrent_ops: 10,
            health_check_interval: Duration::from_secs(60),
            endpoint: None,
        }
    }
}

impl Default for ZfsPerformanceConfig {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            metrics_interval: Duration::from_secs(60),
            operation_cache_size: 1000,
            batch_size: 100,
        }
    }
}


impl Default for PerformanceAnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 30,
            sampling_rate: 1.0,
            storage_backend: "memory".to_string(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            storage_path: "/tmp/nestgate/metrics".to_string(),
            max_history_entries: 10000,
        }
    }
}

impl Default for DashboardIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:3000/api/metrics".to_string(),
            update_interval: Duration::from_secs(30),
            auth_token: None,
        }
    }
}

impl Default for LoadTestingHandlerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_concurrent_sessions: 5,
            max_test_duration: Duration::from_secs(3600), // 1 hour
            resource_limits: LoadTestResourceLimits::default(),
        }
    }
}

impl Default for LoadTestResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024, // 1GB
            max_cpu_percent: 50,
            max_bandwidth_mbps: 100,
        }
    }
}


impl Default for HandlerFeatureFlags {
    fn default() -> Self {
        Self {
            experimental_handlers: false,
            debug_endpoints: false,
            admin_endpoints: false,
            metrics_endpoints: true,
        }
    }
}

impl Default for HandlerSecurityConfig {
    fn default() -> Self {
        Self {
            require_auth: true,
            rate_limiting: RateLimitingConfig::default(),
            cors: CorsConfig::default(),
            security_headers: SecurityHeadersConfig::default(),
        }
    }
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 1000,
            burst_size: 100,
            storage_backend: "memory".to_string(),
        }
    }
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            custom_headers: HashMap::new(),
            csp: Some("default-src 'self'".to_string()),
        }
    }
}

impl Default for HandlerMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            health_checks: HealthCheckConfig::default(),
            alerting: AlertingConfig::default(),
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
        }
    }
}


// ==================== EXISTING API CONFIG TYPES ====================

/// REST API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestApiConfig {
    /// API version
    pub version: String,
    /// Base path
    pub base_path: String,
    /// Enable OpenAPI documentation
    pub enable_openapi: bool,
    /// CORS configuration
    pub cors: CorsConfig,
    /// Request/response logging
    pub logging: ApiLoggingConfig,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Max age for preflight requests
    pub max_age: Duration,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            max_age: Duration::from_secs(3600),
        }
    }
}

/// API logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiLoggingConfig {
    /// Enable request logging
    pub log_requests: bool,
    /// Enable response logging
    pub log_responses: bool,
    /// Log level
    pub level: String,
}

/// Streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    /// Buffer size
    pub buffer_size: usize,
    /// Timeout
    pub timeout: Duration,
    /// Enable compression
    pub enable_compression: bool,
}

/// Server-Sent Events configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseConfig {
    /// Keep alive interval
    pub keep_alive_interval: Duration,
    /// Maximum connections
    pub max_connections: usize,
    /// Buffer size
    pub buffer_size: usize,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// Maximum message size
    pub max_message_size: usize,
    /// Ping interval
    pub ping_interval: Duration,
    /// Connection timeout
    pub connection_timeout: Duration,
}

/// Authentication handler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AuthHandlerConfig {
    /// JWT configuration
    pub jwt: JwtConfig,
    /// OAuth configuration
    pub oauth: OAuthConfig,
    /// Session configuration
    pub session: SessionConfig,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Secret key
    pub secret: String,
    /// Token expiry
    pub expiry: Duration,
    /// Issuer
    pub issuer: String,
}

/// OAuth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Redirect URL
    pub redirect_url: String,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Cookie name
    pub cookie_name: String,
    /// Secure cookies
    pub secure: bool,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Enable dashboard
    pub enabled: bool,
    /// Dashboard path
    pub path: String,
    /// Refresh interval
    pub refresh_interval: Duration,
}

/// Load testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestingConfig {
    /// Enable load testing endpoints
    pub enabled: bool,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Test duration limit
    pub max_duration: Duration,
}

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Default workspace path
    pub default_path: String,
    /// Maximum workspaces per user
    pub max_per_user: usize,
    /// Workspace timeout
    pub timeout: Duration,
}


impl Default for RestApiConfig {
    fn default() -> Self {
        Self {
            version: "v1".to_string(),
            base_path: "/api".to_string(),
            enable_openapi: true,
            cors: CorsConfig::default(),
            logging: ApiLoggingConfig::default(),
        }
    }
}

impl Default for ApiLoggingConfig {
    fn default() -> Self {
        Self {
            log_requests: true,
            log_responses: false,
            level: "info".to_string(),
        }
    }
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            timeout: Duration::from_secs(30),
            enable_compression: true,
        }
    }
}

impl Default for SseConfig {
    fn default() -> Self {
        Self {
            keep_alive_interval: Duration::from_secs(30),
            max_connections: 1000,
            buffer_size: 4096,
        }
    }
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
            ping_interval: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(60),
        }
    }
}


impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "change-me-in-production".to_string(),
            expiry: Duration::from_secs(3600),
            issuer: "nestgate".to_string(),
        }
    }
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            redirect_url: "http://localhost:8080/auth/callback".to_string(),
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(1800),
            cookie_name: "nestgate-session".to_string(),
            secure: false,
        }
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: "/dashboard".to_string(),
            refresh_interval: Duration::from_secs(5),
        }
    }
}

impl Default for LoadTestingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_concurrent_requests: 1000,
            max_duration: Duration::from_secs(300),
        }
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            default_path: "/tmp/nestgate-workspace".to_string(),
            max_per_user: 10,
            timeout: Duration::from_secs(3600),
        }
    }
} 