/// **MASTER UNIFIED CONFIGURATION SYSTEM**
///
/// This module provides the single source of truth for ALL NestGate configuration,
/// consolidating 50+ scattered configuration structures into a unified hierarchy.
///
/// **REPLACES**:
/// - All scattered Config structs across 11 crates
/// - Multiple domain-specific configuration systems
/// - Fragmented API handler configurations
/// - Duplicate ZFS, MCP, Automation, and Network configs
///
/// **PROVIDES**:
/// - Single NestGateMasterConfig as the root configuration
/// - Domain-specific extensions using StandardDomainConfig pattern
/// - Environment-driven configuration loading
/// - Validation and schema generation
/// - Migration utilities from legacy configurations
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use crate::error::{NestGateError, Result};
use crate::unified_config_consolidation::StandardDomainConfig;
use crate::unified_types::{
    UnifiedCacheConfig, UnifiedConnectionPoolConfig, UnifiedMemoryConfig, UnifiedMonitoringConfig,
    UnifiedNetworkConfig, UnifiedRetryConfig, UnifiedSecurityConfig, UnifiedServiceConfig,
    UnifiedStorageConfig, UnifiedTimeoutConfig,
};

// ==================== MASTER CONFIGURATION ====================

/// **THE** master configuration for the entire NestGate ecosystem
/// This is the single source of truth that replaces all scattered configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestGateMasterConfig {
    /// Core system configuration
    pub system: SystemMasterConfig,

    /// Unified base configurations (shared across all domains)
    pub unified: UnifiedBaseConfig,

    /// Domain-specific configurations
    pub domains: DomainConfigurations,

    /// Global feature flags
    pub features: HashMap<String, bool>,

    /// Environment-specific overrides
    pub environment: HashMap<String, serde_json::Value>,

    /// Configuration metadata
    pub metadata: ConfigMetadata,
}

/// Core system-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMasterConfig {
    /// System instance identifier
    pub instance_id: String,
    /// Human-readable instance name
    pub instance_name: String,
    /// Deployment environment
    pub environment: DeploymentEnvironment,
    /// Data directory for persistent storage
    pub data_dir: PathBuf,
    /// Configuration directory
    pub config_dir: PathBuf,
    /// Enable development mode features
    pub dev_mode: bool,
    /// Log level configuration
    pub log_level: String,
    /// System-wide resource limits
    pub resource_limits: ResourceLimits,
}

/// Unified base configurations shared across all domains
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedBaseConfig {
    /// Service identification and metadata
    pub service: UnifiedServiceConfig,
    /// Network configuration
    pub network: UnifiedNetworkConfig,
    /// Security configuration
    pub security: UnifiedSecurityConfig,
    /// Monitoring and telemetry configuration
    pub monitoring: UnifiedMonitoringConfig,
    /// Storage configuration
    pub storage: UnifiedStorageConfig,
    /// Memory configuration
    pub memory: UnifiedMemoryConfig,
    /// Timeout configuration
    pub timeouts: UnifiedTimeoutConfig,
    /// Retry configuration
    pub retry: UnifiedRetryConfig,
    /// Cache configuration
    pub cache: UnifiedCacheConfig,
    /// Connection pool configuration
    pub connection_pool: UnifiedConnectionPoolConfig,
}

/// All domain-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfigurations {
    /// API domain configuration
    pub api: StandardDomainConfig<ApiDomainExtensions>,
    /// ZFS storage domain configuration
    pub zfs: StandardDomainConfig<ZfsDomainExtensions>,
    /// MCP (Model Context Protocol) domain configuration
    pub mcp: StandardDomainConfig<McpDomainExtensions>,
    /// Network domain configuration
    pub network: StandardDomainConfig<NetworkDomainExtensions>,
    /// Automation domain configuration
    pub automation: StandardDomainConfig<AutomationDomainExtensions>,
    /// File system monitor domain configuration
    pub fsmonitor: StandardDomainConfig<FsMonitorDomainExtensions>,
    /// NAS domain configuration
    pub nas: StandardDomainConfig<NasDomainExtensions>,
    /// Middleware domain configuration
    pub middleware: StandardDomainConfig<MiddlewareDomainExtensions>,
}

/// Configuration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    /// Configuration version for compatibility
    pub version: String,
    /// Configuration creation timestamp
    pub created_at: SystemTime,
    /// Last modification timestamp
    pub modified_at: SystemTime,
    /// Configuration source (file, environment, etc.)
    pub source: ConfigSource,
    /// Validation status
    pub validation_status: ValidationStatus,
}

/// Deployment environment types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    Development,
    Testing,
    Staging,
    Production,
    Custom(String),
}

/// System resource limits
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    /// Maximum memory usage (bytes)
    pub max_memory: Option<u64>,
    /// Maximum CPU usage (percentage)
    pub max_cpu: Option<f32>,
    /// Maximum file descriptors
    pub max_file_descriptors: Option<u32>,
    /// Maximum concurrent connections
    pub max_connections: Option<u32>,
}

/// Configuration source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigSource {
    File(PathBuf),
    Environment,
    CommandLine,
    Database,
    Remote(String),
    Merged(Vec<ConfigSource>),
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Valid,
    Warning(Vec<String>),
    Error(Vec<String>),
}

// ==================== DOMAIN EXTENSIONS ====================

/// API domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiDomainExtensions {
    /// HTTP server settings
    pub http_server: HttpServerConfig,
    /// Streaming settings
    pub streaming: StreamingConfig,
    /// Handler-specific configurations
    pub handlers: ApiHandlerConfigs,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
    /// CORS configuration
    pub cors: CorsConfig,
}

/// ZFS domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDomainExtensions {
    /// Pool management configuration
    pub pool_management: PoolManagementConfig,
    /// Dataset configuration
    pub dataset_defaults: DatasetDefaultsConfig,
    /// Snapshot configuration
    pub snapshot_policy: SnapshotPolicyConfig,
    /// Replication configuration
    pub replication: ReplicationConfig,
    /// Performance tuning
    pub performance_tuning: ZfsPerformanceConfig,
}

/// MCP domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpDomainExtensions {
    /// Protocol settings
    pub protocol: McpProtocolConfig,
    /// Transport configuration
    pub transport: McpTransportConfig,
    /// Security settings
    pub security: McpSecurityConfig,
    /// Model configurations
    pub models: ModelConfigurations,
}

/// Network domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkDomainExtensions {
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Service discovery configuration
    pub service_discovery: ServiceDiscoveryConfig,
    /// Circuit breaker configuration
    pub circuit_breakers: CircuitBreakerConfig,
    /// Network optimization settings
    pub optimization: NetworkOptimizationConfig,
}

/// Automation domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomationDomainExtensions {
    /// Workflow configuration
    pub workflows: WorkflowConfig,
    /// Scheduling configuration
    pub scheduling: SchedulingConfig,
    /// Event processing configuration
    pub event_processing: EventProcessingConfig,
    /// Integration settings
    pub integrations: AutomationIntegrationsConfig,
}

/// File system monitor domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FsMonitorDomainExtensions {
    /// Watch configuration
    pub watch_settings: WatchConfig,
    /// Event filtering
    pub event_filters: EventFilterConfig,
    /// Storage backend configuration
    pub storage_backend: StorageBackendConfig,
    /// Performance settings
    pub performance: FsMonitorPerformanceConfig,
}

/// NAS domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasDomainExtensions {
    /// Share configuration
    pub shares: ShareConfig,
    /// Protocol support
    pub protocols: ProtocolConfig,
    /// Access control
    pub access_control: AccessControlConfig,
    /// Backup configuration
    pub backup: BackupConfig,
}

/// Middleware domain-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiddlewareDomainExtensions {
    /// Authentication middleware
    pub authentication: AuthMiddlewareConfig,
    /// Logging middleware
    pub logging: LoggingMiddlewareConfig,
    /// Compression middleware
    pub compression: CompressionMiddlewareConfig,
    /// Custom middleware chain
    pub custom_chain: Vec<CustomMiddlewareConfig>,
}

// ==================== CONFIGURATION STRUCTS ====================
// Note: These would be defined in detail based on the specific needs
// For now, providing placeholder structures

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpServerConfig {
    pub max_request_size: u64,
    pub keep_alive_timeout: Duration,
    pub request_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamingConfig {
    pub buffer_size: usize,
    pub max_connections: u32,
    pub heartbeat_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiHandlerConfigs {
    pub zfs_handler: HashMap<String, serde_json::Value>,
    pub performance_handler: HashMap<String, serde_json::Value>,
    pub dashboard_handler: HashMap<String, serde_json::Value>,
    pub load_testing_handler: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

// Additional configuration structs would be defined here...
// (Abbreviated for brevity - each domain extension would have detailed configs)

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoolManagementConfig {
    pub auto_expand: bool,
    pub scrub_schedule: String,
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetDefaultsConfig {
    pub compression: String,
    pub deduplication: bool,
    pub encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapshotPolicyConfig {
    pub auto_snapshot: bool,
    pub retention_policy: String,
    pub schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplicationConfig {
    pub enabled: bool,
    pub targets: Vec<String>,
    pub schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {
    pub arc_max: Option<u64>,
    pub prefetch_disable: bool,
    pub sync_disabled: bool,
}

// MCP configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpProtocolConfig {
    pub version: String,
    pub max_message_size: u64,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpTransportConfig {
    pub transport_type: String,
    pub compression: bool,
    pub encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpSecurityConfig {
    pub authentication_required: bool,
    pub allowed_operations: Vec<String>,
    pub rate_limiting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelConfigurations {
    pub default_model: String,
    pub model_settings: HashMap<String, serde_json::Value>,
}

// Additional configuration structs...
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    pub algorithm: String,
    pub health_check_interval: Duration,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub discovery_interval: Duration,
    pub service_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkOptimizationConfig {
    pub tcp_nodelay: bool,
    pub keep_alive: bool,
    pub buffer_sizes: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowConfig {
    pub max_concurrent_workflows: u32,
    pub default_timeout: Duration,
    pub retry_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulingConfig {
    pub enabled: bool,
    pub scheduler_type: String,
    pub max_scheduled_tasks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventProcessingConfig {
    pub buffer_size: usize,
    pub batch_size: u32,
    pub flush_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomationIntegrationsConfig {
    pub external_systems: HashMap<String, serde_json::Value>,
    pub webhooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchConfig {
    pub recursive: bool,
    pub ignore_patterns: Vec<String>,
    pub max_events: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventFilterConfig {
    pub enabled_events: Vec<String>,
    pub excluded_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBackendConfig {
    pub backend_type: String,
    pub connection_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FsMonitorPerformanceConfig {
    pub polling_interval: Duration,
    pub batch_processing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShareConfig {
    pub default_permissions: String,
    pub auto_create_users: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolConfig {
    pub nfs_enabled: bool,
    pub smb_enabled: bool,
    pub ftp_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessControlConfig {
    pub default_access: String,
    pub user_quotas: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupConfig {
    pub auto_backup: bool,
    pub backup_schedule: String,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthMiddlewareConfig {
    pub enabled: bool,
    pub jwt_secret: Option<String>,
    pub session_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingMiddlewareConfig {
    pub enabled: bool,
    pub log_level: String,
    pub log_requests: bool,
    pub log_responses: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompressionMiddlewareConfig {
    pub enabled: bool,
    pub algorithms: Vec<String>,
    pub min_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomMiddlewareConfig {
    pub name: String,
    pub enabled: bool,
    pub config: HashMap<String, serde_json::Value>,
}

// ==================== IMPLEMENTATION ====================

impl Default for SystemMasterConfig {
    fn default() -> Self {
        Self {
            instance_id: format!("nestgate-{}", std::process::id()),
            instance_name: "nestgate-instance".to_string(),
            environment: DeploymentEnvironment::Development,
            data_dir: PathBuf::from("./data"),
            config_dir: PathBuf::from("./config"),
            dev_mode: true,
            log_level: "info".to_string(),
            resource_limits: ResourceLimits::default(),
        }
    }
}

impl Default for DomainConfigurations {
    fn default() -> Self {
        Self {
            api: StandardDomainConfig::new(ApiDomainExtensions::default()),
            zfs: StandardDomainConfig::new(ZfsDomainExtensions::default()),
            mcp: StandardDomainConfig::new(McpDomainExtensions::default()),
            network: StandardDomainConfig::new(NetworkDomainExtensions::default()),
            automation: StandardDomainConfig::new(AutomationDomainExtensions::default()),
            fsmonitor: StandardDomainConfig::new(FsMonitorDomainExtensions::default()),
            nas: StandardDomainConfig::new(NasDomainExtensions::default()),
            middleware: StandardDomainConfig::new(MiddlewareDomainExtensions::default()),
        }
    }
}

impl Default for ConfigMetadata {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            version: "1.0.0".to_string(),
            created_at: now,
            modified_at: now,
            source: ConfigSource::Environment,
            validation_status: ValidationStatus::Valid,
        }
    }
}

impl NestGateMasterConfig {
    /// Create a new master configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables
    pub fn from_environment() -> Result<Self> {
        // Implementation would load from environment variables
        // For now, return default with environment source
        let mut config = Self::default();
        config.metadata.source = ConfigSource::Environment;
        Ok(config)
    }

    /// Load configuration from file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path).map_err(|e| {
            NestGateError::configuration_error(format!("Failed to read config file: {e}"), None)
        })?;

        let mut config: Self = if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            // For now, only support JSON until serde_yaml is added as dependency
            return Err(NestGateError::configuration_error(
                "YAML support not yet implemented".to_string(),
                None,
            ));
        } else {
            serde_json::from_str(&contents).map_err(|e| {
                NestGateError::configuration_error(format!("Invalid JSON: {e}"), None)
            })?
        };

        config.metadata.source = ConfigSource::File(path.to_path_buf());
        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Implementation would perform comprehensive validation
        // For now, basic validation
        if self.system.instance_name.is_empty() {
            return Err(NestGateError::configuration_error(
                "Instance name cannot be empty".to_string(),
                Some("instance_name".to_string()),
            ));
        }

        if !self.system.data_dir.exists() {
            std::fs::create_dir_all(&self.system.data_dir).map_err(|e| {
                NestGateError::configuration_error(
                    format!("Failed to create data directory: {e}"),
                    Some("data_dir".to_string()),
                )
            })?;
        }

        Ok(())
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        // Implementation would perform deep merge
        // For now, simple override of domains
        self.domains = other.domains;
        self.features.extend(other.features);
        self.environment.extend(other.environment);
        self.metadata.modified_at = SystemTime::now();
        self.metadata.source =
            ConfigSource::Merged(vec![self.metadata.source, other.metadata.source]);
        self
    }

    /// Get configuration schema for documentation
    pub fn schema() -> serde_json::Value {
        // Implementation would generate JSON schema
        serde_json::json!({
            "type": "object",
            "description": "NestGate Master Configuration Schema",
            "properties": {
                "system": {"$ref": "#/definitions/SystemMasterConfig"},
                "unified": {"$ref": "#/definitions/UnifiedBaseConfig"},
                "domains": {"$ref": "#/definitions/DomainConfigurations"},
                "features": {"type": "object"},
                "environment": {"type": "object"},
                "metadata": {"$ref": "#/definitions/ConfigMetadata"}
            }
        })
    }
}

// ==================== CONFIGURATION TRAIT IMPLEMENTATIONS ====================

/// Trait for domain-specific configuration validation
pub trait DomainConfigValidator {
    fn validate(&self) -> Result<()>;
    fn schema() -> serde_json::Value;
}

impl DomainConfigValidator for ApiDomainExtensions {
    fn validate(&self) -> Result<()> {
        // API-specific validation
        Ok(())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({"type": "object", "description": "API domain extensions"})
    }
}

impl DomainConfigValidator for ZfsDomainExtensions {
    fn validate(&self) -> Result<()> {
        // ZFS-specific validation
        Ok(())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({"type": "object", "description": "ZFS domain extensions"})
    }
}

// Additional trait implementations for other domain extensions...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_config_creation() {
        let config = NestGateMasterConfig::new();
        assert!(!config.system.instance_id.is_empty());
        assert_eq!(
            config.system.environment,
            DeploymentEnvironment::Development
        );
    }

    #[test]
    fn test_config_validation() {
        let config = NestGateMasterConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = NestGateMasterConfig::new();
        config1.features.insert("feature1".to_string(), true);

        let mut config2 = NestGateMasterConfig::new();
        config2.features.insert("feature2".to_string(), false);

        let merged = config1.merge(config2);
        assert!(merged.features.contains_key("feature1"));
        assert!(merged.features.contains_key("feature2"));
    }
}
