//! Enhanced Configuration Management for NestGate v2
//!
//! Advanced configuration capabilities with v2 orchestrator-centric architecture
//!
//! ## Encryption Architecture Note
//!
//! NestGate is **encryption-agnostic** by design:
//! - NestGate handles storage, ZFS operations, and replication
//! - External systems (like BearDog) handle encryption, keys, and security
//! - This separation allows NestGate to be a pure storage layer
//! - BearDog (or other providers) can use NestGate for storage while handling encryption
//!
//! Configuration options marked as "encryption" are typically:
//! - Metadata tracking (is this data encrypted?)
//! - Hints/preferences for external encryption providers
//! - NOT actual encryption operations performed by NestGate

use config::{Config as ConfigBuilder, Environment as ConfigEnvironment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use uuid;

// Re-export from existing error module
use crate::error::{NestGateError, Result};

/// Main configuration structure for the NestGate v2 system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// System-wide settings
    pub system: SystemConfig,

    /// Storage configuration
    pub storage: StorageConfig,

    /// Security settings
    pub security: SecurityConfig,

    /// Monitoring configuration
    pub monitoring: MonitoringConfig,

    /// MCP integration configuration (from Phase 1)
    pub mcp: Option<McpConfig>,

    /// Federation configuration
    pub federation: Option<FederationConfig>,

    /// Service endpoints configuration (replaces hardcoded URLs)
    pub endpoints: ServiceEndpoints,
}

/// System-wide configuration settings with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Log level for the system
    pub log_level: String,

    /// Data directory path
    pub data_dir: String,

    /// Temporary directory path
    pub temp_dir: String,

    /// Maximum number of concurrent operations
    pub max_concurrent_ops: usize,

    /// System identification
    pub node_id: String,

    /// Environment (dev, test, prod)
    pub environment: String,
}

// Orchestrator configuration moved to nestgate-integration crate
// These are ecosystem responsibilities, not core NestGate functionality

/// Storage configuration settings with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Cache size in bytes
    pub cache_size: u64,

    /// Maximum file size in bytes
    pub max_file_size: u64,

    /// Storage tier configurations
    pub tiers: Vec<StorageTierConfig>,

    /// Storage protocols configuration
    pub protocols: StorageProtocolsConfig,
}

/// Storage tier configuration with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTierConfig {
    /// Tier name
    pub name: String,

    /// Tier type (hot, warm, cold, archive)
    pub tier_type: String,

    /// Storage path
    pub path: String,

    /// Maximum capacity in bytes
    pub capacity: u64,

    /// Performance configuration
    pub performance: TierPerformanceConfig,
}

/// Tier performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPerformanceConfig {
    /// Maximum IOPS
    pub max_iops: u64,

    /// Maximum throughput in MB/s
    pub max_throughput: u64,

    /// Latency target in milliseconds
    pub latency_target: f64,
}

/// Storage protocols configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProtocolsConfig {
    /// NFS configuration
    pub nfs: Option<NfsConfig>,

    /// SMB configuration
    pub smb: Option<SmbConfig>,

    /// iSCSI configuration
    pub iscsi: Option<IscsiConfig>,

    /// S3 configuration
    pub s3: Option<S3Config>,
}

/// NFS protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsConfig {
    /// NFS version (3, 4, 4.1, 4.2)
    pub version: String,

    /// Export path
    pub export_path: String,

    /// Allowed clients
    pub allowed_clients: Vec<String>,

    /// Mount options
    pub mount_options: HashMap<String, String>,
}

/// SMB protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbConfig {
    /// SMB version (2, 3, 3.1)
    pub version: String,

    /// Share name
    pub share_name: String,

    /// Workgroup
    pub workgroup: String,

    /// Authentication method
    pub auth_method: String,
}

/// iSCSI protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IscsiConfig {
    /// Target name
    pub target_name: String,

    /// Portal address
    pub portal: String,

    /// Authentication settings
    pub auth: IscsiAuthConfig,
}

/// iSCSI authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IscsiAuthConfig {
    /// CHAP username
    pub username: Option<String>,

    /// CHAP secret
    pub secret: Option<String>,

    /// Mutual CHAP
    pub mutual_chap: bool,
}

/// S3 protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 endpoint
    pub endpoint: String,

    /// Bucket name
    pub bucket: String,

    /// Region
    pub region: String,

    /// Access key ID
    pub access_key_id: String,

    /// Secret access key
    pub secret_access_key: String,
}

/// Security configuration with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication method to use
    pub auth_method: String,

    /// Encryption algorithm preference (for external providers like BearDog)
    /// Note: NestGate itself does not perform encryption - this is a hint for external systems
    pub encryption_algorithm: String,

    /// Number of days between key rotations
    pub key_rotation_days: u32,

    /// Maximum number of failed login attempts
    pub max_failed_attempts: u32,

    /// JWT configuration
    pub jwt: Option<JwtConfig>,

    /// TLS configuration
    pub tls: Option<TlsConfig>,

    /// RBAC configuration
    pub rbac: RbacConfig,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT secret key
    pub secret: String,

    /// Token expiration in seconds
    pub expiration: u64,

    /// Issuer
    pub issuer: String,

    /// Audience
    pub audience: String,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert_file: String,

    /// Private key file path
    pub key_file: String,

    /// CA certificate file path
    pub ca_file: Option<String>,

    /// Minimum TLS version
    pub min_version: String,
}

/// RBAC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfig {
    /// Enable RBAC
    pub enabled: bool,

    /// Default role for new users
    pub default_role: String,

    /// Role definitions
    pub roles: HashMap<String, RoleDefinition>,
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    /// Role name
    pub name: String,

    /// Role description
    pub description: String,

    /// Permissions
    pub permissions: Vec<String>,
}

/// Monitoring configuration with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Interval in seconds between metrics collection
    pub metrics_interval: u32,

    /// Log level for system logging
    pub log_level: String,

    /// Path to the log file
    pub log_file: String,

    /// Maximum size in bytes before log rotation
    pub log_rotation_size: u32,

    /// Number of days to retain log files
    pub log_retention_days: u32,

    /// Prometheus configuration
    pub prometheus: Option<PrometheusConfig>,

    /// Alert configuration
    pub alerts: AlertConfig,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,

    /// Metrics endpoint port
    pub port: u16,

    /// Metrics path
    pub path: String,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Enable alerts
    pub enabled: bool,

    /// Alert thresholds
    pub thresholds: AlertThresholds,

    /// Notification configuration
    pub notifications: NotificationConfig,
}

/// Alert thresholds for system metrics monitoring
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage threshold as a percentage (0.0 - 100.0)
    pub cpu_threshold: f64,

    /// Memory usage threshold as a percentage (0.0 - 100.0)
    pub memory_threshold: f64,

    /// Disk usage threshold as a percentage (0.0 - 100.0)
    pub disk_threshold: f64,

    /// Network latency threshold in milliseconds
    pub latency_threshold: f64,

    /// Error rate threshold as a percentage
    pub error_rate_threshold: f64,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Email notifications
    pub email: Option<EmailConfig>,

    /// Slack notifications
    pub slack: Option<SlackConfig>,

    /// Webhook notifications
    pub webhook: Option<WebhookConfig>,
}

/// Email notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP server
    pub smtp_server: String,

    /// SMTP port
    pub smtp_port: u16,

    /// Username
    pub username: String,

    /// Password
    pub password: String,

    /// From address
    pub from: String,

    /// To addresses
    pub to: Vec<String>,
}

/// Slack notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    /// Webhook URL
    pub webhook_url: String,

    /// Channel
    pub channel: String,

    /// Username
    pub username: String,
}

/// Webhook notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,

    /// HTTP method
    pub method: String,

    /// Headers
    pub headers: HashMap<String, String>,
}

/// MCP integration configuration (from Phase 1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Enable MCP integration
    pub enabled: bool,

    /// MCP cluster endpoint
    pub cluster_endpoint: String,

    /// Node identifier
    pub node_id: String,

    /// Federation enabled
    pub federation_enabled: bool,

    /// Capabilities configuration
    pub capabilities: McpCapabilitiesConfig,
}

/// MCP capabilities configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCapabilitiesConfig {
    /// Supported storage protocols
    pub storage_protocols: Vec<String>,

    /// Supported storage tiers
    pub storage_tiers: Vec<String>,

    /// Maximum volume size
    pub max_volume_size: u64,

    /// Maximum volumes
    pub max_volumes: u32,
}

/// Federation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    /// Enable federation
    pub enabled: bool,

    /// Cluster name
    pub cluster_name: String,

    /// Federation mode (leader, follower, observer)
    pub mode: String,

    /// Peer nodes
    pub peers: Vec<String>,

    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
}

/// Service endpoint configuration to eliminate hardcoded URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// NestGate API endpoints
    pub nestgate_api: String,
    pub nestgate_ui: String,

    /// External service endpoints
    pub songbird_orchestrator: String,
    pub beardog_security: String,
    pub ecosystem_orchestrator: String,

    /// Development and testing endpoints
    pub prometheus_metrics: Option<String>,
    pub grafana_dashboard: Option<String>,

    /// Discovery endpoints
    pub discovery_endpoints: Vec<String>,
}

impl Default for ServiceEndpoints {
    fn default() -> Self {
        Self {
            // NestGate services
            nestgate_api: std::env::var("NESTGATE_API_URL").unwrap_or_else(|_| {
                format!("http://localhost:{}", crate::constants::network::api_port())
            }),
            nestgate_ui: std::env::var("NESTGATE_UI_URL").unwrap_or_else(|_| {
                format!(
                    "http://localhost:{}",
                    std::env::var("NESTGATE_UI_PORT").unwrap_or_else(|_| "3000".to_string())
                )
            }),

            // External services with environment fallbacks
            songbird_orchestrator: std::env::var("SONGBIRD_ORCHESTRATOR_URL").unwrap_or_else(
                |_| {
                    format!(
                        "http://{}:{}",
                        std::env::var("SONGBIRD_HOST")
                            .unwrap_or_else(|_| "songbird-orchestrator".to_string()),
                        std::env::var("SONGBIRD_PORT").unwrap_or_else(|_| "8000".to_string())
                    )
                },
            ),
            beardog_security: std::env::var("BEARDOG_URL").unwrap_or_else(|_| {
                format!(
                    "https://{}:{}",
                    std::env::var("BEARDOG_HOST").unwrap_or_else(|_| "beardog.local".to_string()),
                    std::env::var("BEARDOG_PORT").unwrap_or_else(|_| "8443".to_string())
                )
            }),
            ecosystem_orchestrator: std::env::var("ECOSYSTEM_ORCHESTRATOR_URL").unwrap_or_else(
                |_| {
                    format!(
                        "http://localhost:{}",
                        crate::constants::network::orchestrator_port()
                    )
                },
            ),

            // Optional monitoring services
            prometheus_metrics: std::env::var("PROMETHEUS_URL").ok(),
            grafana_dashboard: std::env::var("GRAFANA_URL").ok(),

            // Discovery endpoints with smart defaults
            discovery_endpoints: std::env::var("DISCOVERY_ENDPOINTS")
                .map(|s| s.split(',').map(String::from).collect())
                .unwrap_or_else(|_| {
                    vec![
                        format!(
                            "http://localhost:{}/api/v1/discovery",
                            crate::constants::network::api_port()
                        ),
                        format!(
                            "http://{}:{}/api/v1/discovery",
                            crate::constants::addresses::localhost(),
                            crate::constants::network::discovery_port()
                        ),
                    ]
                }),
        }
    }
}

// Network configuration constants and utilities
pub const DEFAULT_LOCALHOST: &str = "127.0.0.1";
pub const DEFAULT_ALL_INTERFACES: &str = "0.0.0.0";
pub const DEFAULT_IPV6_LOCALHOST: &str = "::1";
pub const DEFAULT_IPV6_ALL_INTERFACES: &str = "::";

// Default ports for different services
// NOTE: All hardcoded ports removed - Songbird manages port allocation
// These are only used as fallbacks in standalone mode
pub mod default_ports {
    // All ports set to 0 - let OS assign in standalone mode
    // In ecosystem mode, Songbird manages all port allocation
    pub const API_FALLBACK: u16 = 0;
    pub const HEALTH_FALLBACK: u16 = 0;
    pub const METRICS_FALLBACK: u16 = 0;
}

/// Network binding configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfig {
    /// Interface to bind to (localhost, all interfaces, or specific IP)
    pub bind_interface: String,
    /// Port to bind to (0 for auto-assignment)
    pub port: u16,
    /// Whether to enable IPv6 support
    pub ipv6_enabled: bool,
    /// Whether to bind to localhost only (secure) or all interfaces
    pub localhost_only: bool,
    /// Custom host override
    pub custom_host: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        // Check if we're in Songbird mode or standalone mode
        let songbird_mode = std::env::var("SONGBIRD_URL").is_ok();

        if songbird_mode {
            // Songbird-enhanced mode: use service names
            Self {
                bind_interface: std::env::var("SONGBIRD_SERVICE_NAME")
                    .unwrap_or_else(|_| "nestgate-service".to_string()),
                port: 0, // Let Songbird allocate
                ipv6_enabled: false,
                localhost_only: false, // Songbird handles security
                custom_host: None,
            }
        } else {
            // Standalone mode: use localhost binding
            Self {
                bind_interface: "127.0.0.1".to_string(), // ✅ LOCALHOST FOR STANDALONE
                port: std::env::var("NESTGATE_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                ipv6_enabled: false,
                localhost_only: true, // ✅ SECURE BY DEFAULT
                custom_host: None,
            }
        }
    }
}

impl NetworkConfig {
    /// Create a new network config for localhost binding (secure)
    pub fn localhost(port: u16) -> Self {
        Self {
            bind_interface: DEFAULT_LOCALHOST.to_string(),
            port,
            ipv6_enabled: false,
            localhost_only: true,
            custom_host: None,
        }
    }

    /// Create a new network config for all interfaces (less secure, for production)
    pub fn all_interfaces(port: u16) -> Self {
        Self {
            bind_interface: DEFAULT_ALL_INTERFACES.to_string(),
            port,
            ipv6_enabled: false,
            localhost_only: false,
            custom_host: None,
        }
    }

    /// Create a new network config with custom host
    pub fn custom_host(host: &str, port: u16) -> Self {
        Self {
            bind_interface: host.to_string(),
            port,
            ipv6_enabled: false,
            localhost_only: host == DEFAULT_LOCALHOST || host == DEFAULT_IPV6_LOCALHOST,
            custom_host: Some(host.to_string()),
        }
    }

    /// Get the full bind address
    pub fn bind_address(&self) -> String {
        if let Some(ref custom) = self.custom_host {
            format!("{}:{}", custom, self.port)
        } else {
            format!("{}:{}", self.bind_interface, self.port)
        }
    }

    /// Get the interface to bind to
    pub fn interface(&self) -> &str {
        if let Some(ref custom) = self.custom_host {
            custom
        } else {
            &self.bind_interface
        }
    }

    /// Check if this is a secure localhost-only binding
    pub fn is_localhost_only(&self) -> bool {
        self.localhost_only
            || self.bind_interface == DEFAULT_LOCALHOST
            || self.bind_interface == DEFAULT_IPV6_LOCALHOST
            || self
                .custom_host
                .as_ref()
                .is_some_and(|h| h == DEFAULT_LOCALHOST || h == DEFAULT_IPV6_LOCALHOST)
    }

    /// Check if this exposes the service to external networks
    pub fn is_externally_accessible(&self) -> bool {
        !self.is_localhost_only()
    }
}

/// Environment-based configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentConfig {
    /// Environment type (development, testing, production)
    pub environment: RuntimeEnvironment,
    /// Whether to use secure defaults
    pub secure_defaults: bool,
    /// Whether to allow external access
    pub allow_external_access: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RuntimeEnvironment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            environment: RuntimeEnvironment::Development,
            secure_defaults: true,
            allow_external_access: false,
        }
    }
}

impl EnvironmentConfig {
    /// Get default network configuration for this environment
    pub fn default_network_config(&self, service_port: u16) -> NetworkConfig {
        // Check if we're in Songbird mode
        let songbird_mode = std::env::var("SONGBIRD_URL").is_ok();

        if songbird_mode {
            // Songbird-enhanced mode: service-based addressing
            NetworkConfig {
                bind_interface: std::env::var("SONGBIRD_SERVICE_NAME").unwrap_or_else(|_| {
                    format!("nestgate-{}", &uuid::Uuid::new_v4().to_string()[..8])
                }),
                port: 0, // Always let Songbird allocate
                ipv6_enabled: false,
                localhost_only: false, // Songbird handles security
                custom_host: None,
            }
        } else {
            // Standalone mode: environment-appropriate binding
            match (&self.environment, self.allow_external_access) {
                (RuntimeEnvironment::Development, false) => NetworkConfig {
                    bind_interface: "127.0.0.1".to_string(),
                    port: service_port,
                    ipv6_enabled: false,
                    localhost_only: true,
                    custom_host: None,
                },
                (RuntimeEnvironment::Production, true) => NetworkConfig {
                    bind_interface: "0.0.0.0".to_string(), // Allow external in production
                    port: service_port,
                    ipv6_enabled: false,
                    localhost_only: false,
                    custom_host: None,
                },
                _ => NetworkConfig {
                    bind_interface: "127.0.0.1".to_string(), // Default to secure
                    port: service_port,
                    ipv6_enabled: false,
                    localhost_only: true,
                    custom_host: None,
                },
            }
        }
    }
}

impl Config {
    /// Loads configuration from the specified path with environment variable support
    ///
    /// # Arguments
    /// * `path` - The path to the configuration file
    ///
    /// # Errors
    /// * Returns error if the configuration file cannot be read or parsed
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = ConfigBuilder::builder()
            .add_source(File::from(path.as_ref()))
            .add_source(ConfigEnvironment::with_prefix("NESTGATE"))
            .build()
            .map_err(|e| NestGateError::Configuration(format!("Failed to build config: {e}")))?;

        config
            .try_deserialize()
            .map_err(|e| NestGateError::Configuration(format!("Failed to deserialize config: {e}")))
    }

    /// Loads configuration from multiple sources with priority
    pub fn load_with_sources(sources: Vec<&str>) -> Result<Self> {
        let mut builder = ConfigBuilder::builder();

        for source in sources {
            builder = builder.add_source(File::with_name(source).required(false));
        }

        let config = builder
            .add_source(ConfigEnvironment::with_prefix("NESTGATE"))
            .build()
            .map_err(|e| NestGateError::Configuration(format!("Failed to build config: {e}")))?;

        config
            .try_deserialize()
            .map_err(|e| NestGateError::Configuration(format!("Failed to deserialize config: {e}")))
    }

    /// Validates the configuration
    ///
    /// # Errors
    /// * Returns error if any configuration values are invalid
    pub fn validate(&self) -> Result<()> {
        // Validate system config
        if self.system.max_concurrent_ops == 0 {
            return Err(NestGateError::Validation(
                "max_concurrent_ops must be greater than 0".to_string(),
            ));
        }

        // Orchestrator validation removed - this is Songbird's responsibility

        // Validate storage config
        if self.storage.cache_size == 0 {
            return Err(NestGateError::Validation(
                "cache_size must be greater than 0".to_string(),
            ));
        }

        // Validate security config
        if self.security.max_failed_attempts == 0 {
            return Err(NestGateError::Validation(
                "max_failed_attempts must be greater than 0".to_string(),
            ));
        }

        // Validate monitoring config
        if self.monitoring.metrics_interval == 0 {
            return Err(NestGateError::Validation(
                "metrics_interval must be greater than 0".to_string(),
            ));
        }

        // Validate MCP config if enabled
        if let Some(mcp) = &self.mcp {
            if mcp.enabled && mcp.cluster_endpoint.is_empty() {
                return Err(NestGateError::Validation(
                    "MCP cluster_endpoint cannot be empty when MCP is enabled".to_string(),
                ));
            }
        }

        // Validate federation config if enabled
        if let Some(federation) = &self.federation {
            if federation.enabled && federation.cluster_name.is_empty() {
                return Err(NestGateError::Validation(
                    "Federation cluster_name cannot be empty when federation is enabled"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Save configuration to a file
    pub async fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let serialized = toml::to_string_pretty(self).map_err(|e| {
            NestGateError::Configuration(format!("Failed to serialize config: {}", e))
        })?;

        tokio::fs::write(path, serialized).await.map_err(|e| {
            NestGateError::Configuration(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    /// Gets the configuration for a specific environment
    pub fn for_environment(env: &str) -> Result<Self> {
        let env_config = format!("config/{}", env);
        let config_files = vec!["config/default", &env_config, "config/local"];

        Self::load_with_sources(config_files)
    }
}

impl Default for Config {
    fn default() -> Self {
        // Generate dynamic node ID instead of hardcoding
        let node_id = format!(
            "nestgate-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        );

        Self {
            system: SystemConfig {
                log_level: "info".to_string(),
                // Use relative paths - Songbird manages absolute paths
                data_dir: "./data".to_string(),
                temp_dir: "./tmp".to_string(),
                max_concurrent_ops: 1000,
                node_id: node_id.clone(),
                environment: "development".to_string(),
            },
            // Orchestrator config removed - this is Songbird's responsibility
            storage: StorageConfig {
                cache_size: 1024 * 1024 * 1024,          // 1GB
                max_file_size: 1024 * 1024 * 1024 * 100, // 100GB
                tiers: vec![],
                protocols: StorageProtocolsConfig {
                    nfs: None,
                    smb: None,
                    iscsi: None,
                    s3: None,
                },
            },
            security: SecurityConfig {
                auth_method: "jwt".to_string(),
                encryption_algorithm: "aes-256-gcm".to_string(),
                key_rotation_days: 30,
                max_failed_attempts: 5,
                jwt: None,
                tls: None,
                rbac: RbacConfig {
                    enabled: true,
                    default_role: "user".to_string(),
                    roles: HashMap::new(),
                },
            },
            monitoring: MonitoringConfig {
                metrics_interval: 30,
                log_level: "info".to_string(),
                // Use relative path - Songbird manages absolute paths
                log_file: "./logs/nestgate.log".to_string(),
                log_rotation_size: std::env::var("NESTGATE_LOG_ROTATION_SIZE_BYTES")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1024 * 1024), // 1MB default
                log_retention_days: 30,
                prometheus: Some(PrometheusConfig {
                    enabled: true,
                    port: 0, // Let OS assign port - Songbird manages routing
                    path: "/metrics".to_string(),
                }),
                alerts: AlertConfig {
                    enabled: true,
                    thresholds: AlertThresholds {
                        cpu_threshold: 80.0,
                        memory_threshold: 85.0,
                        disk_threshold: 90.0,
                        latency_threshold: 1000.0,
                        error_rate_threshold: 5.0,
                    },
                    notifications: NotificationConfig {
                        email: None,
                        slack: None,
                        webhook: None,
                    },
                },
            },
            mcp: Some(McpConfig {
                enabled: false, // Disabled by default - Songbird manages MCP
                cluster_endpoint: std::env::var("NESTGATE_CLUSTER_ENDPOINT")
                    .unwrap_or_else(|_| "localhost:8080".to_string()),
                node_id: node_id.clone(),
                federation_enabled: false,
                capabilities: McpCapabilitiesConfig {
                    storage_protocols: vec!["nfs".to_string(), "smb".to_string(), "s3".to_string()],
                    storage_tiers: vec!["hot".to_string(), "warm".to_string(), "cold".to_string()],
                    max_volume_size: 1024 * 1024 * 1024 * 1024, // 1TB
                    max_volumes: 1000,
                },
            }),
            federation: Some(FederationConfig {
                enabled: false,                 // Disabled by default - Songbird manages federation
                cluster_name: "".to_string(),   // Empty - Songbird provides cluster name
                mode: "standalone".to_string(), // Default to standalone
                peers: vec![],                  // Empty - Songbird discovers peers
                heartbeat_interval: 30,
            }),
            endpoints: ServiceEndpoints::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.system.log_level, "info");
        // Orchestrator port removed - this is Songbird's responsibility
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Test invalid max_concurrent_ops
        config.system.max_concurrent_ops = 0;
        assert!(config.validate().is_err());

        // Reset and test valid config
        config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = serde_yaml::to_string(&config)
            .expect("Failed to serialize config - this should never fail with default config");
        let deserialized: Config = serde_yaml::from_str(&serialized)
            .expect("Failed to deserialize config - serialization format may be corrupted");

        assert_eq!(config.system.log_level, deserialized.system.log_level);
    }

    #[test]
    fn test_config_loading() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().with_extension("yaml");
        writeln!(
            temp_file,
            r#"
system:
  log_level: "debug"
  data_dir: "/tmp/test"
  temp_dir: "/tmp/test/temp"
  max_concurrent_ops: 500
  node_id: "test-node"
  environment: "test"
# orchestrator config removed - this is Songbird's responsibility
storage:
  cache_size: 2147483648
  max_file_size: 107374182400
  tiers: []
  protocols:
    nfs: null
    smb: null
    iscsi: null
    s3: null
security:
  auth_method: "oauth2"
  encryption_algorithm: "aes-256-gcm"
  key_rotation_days: 30
  max_failed_attempts: 5
  jwt: null
  tls: null
  rbac:
    enabled: true
    default_role: "user"
    roles: {{}}
monitoring:
  metrics_interval: 60
  log_level: "debug"
  log_file: "/tmp/test.log"
  log_rotation_size: 2097152
  log_retention_days: 7
  prometheus:
    enabled: true
    port: 9090
    path: "/metrics"
  alerts:
    enabled: true
    thresholds:
      cpu_threshold: 80.0
      memory_threshold: 85.0
      disk_threshold: 90.0
      latency_threshold: 1000.0
      error_rate_threshold: 5.0
    notifications:
      email: null
      slack: null
      webhook: null
mcp:
  enabled: true
  cluster_endpoint: "localhost:8080"  # Can be overridden with NESTGATE_CLUSTER_ENDPOINT
  node_id: "test-node"
  federation_enabled: false
  capabilities:
    storage_protocols: ["nfs", "smb", "s3"]
    storage_tiers: ["hot", "warm", "cold"]
    max_volume_size: 1099511627776
    max_volumes: 1000
federation:
  enabled: false
  cluster_name: "test-cluster"
  mode: "follower"
  peers: []
  heartbeat_interval: 30
endpoints:
  nestgate_api: "http://localhost:8080"
  nestgate_ui: "http://localhost:3000"
  songbird_orchestrator: "http://localhost:9000"
  beardog_security: "http://localhost:7000"
  ecosystem_orchestrator: "http://localhost:6000"
  prometheus_metrics: "http://localhost:9090"
  grafana_dashboard: "http://localhost:3001"
  discovery_endpoints: []
"#
        )
        .unwrap();

        // Copy to the new path with .yaml extension
        std::fs::copy(temp_file.path(), &temp_path).unwrap();

        let config = Config::load(&temp_path).unwrap();
        assert_eq!(config.system.log_level, "debug");
        // Orchestrator port removed - this is Songbird's responsibility
        assert_eq!(config.system.max_concurrent_ops, 500);
        assert!(config.validate().is_ok());

        // Clean up
        std::fs::remove_file(&temp_path).unwrap();
    }

    #[tokio::test]
    async fn test_config_file_operations() {
        let config = Config::default();
        let temp_file =
            NamedTempFile::new().expect("Failed to create temporary file for config test");

        // Test saving config
        config
            .save(temp_file.path())
            .await
            .expect("Failed to save config to temporary file");

        // Create a permanent temp path for loading test
        let temp_path = temp_file.path().with_extension("test.toml");
        std::fs::copy(temp_file.path(), &temp_path)
            .expect("Failed to copy temp file for loading test");

        let loaded_config =
            Config::load(&temp_path).expect("Failed to load config from temporary file");

        assert_eq!(config.system.log_level, loaded_config.system.log_level);

        // Cleanup
        std::fs::remove_file(&temp_path).expect("Failed to cleanup temporary config file");
    }
}

/// Network configuration constants
pub mod network {
    use super::*;

    /// Get the default bind address for services
    pub fn default_bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string())
    }

    /// Get the default API server address
    pub fn default_api_address() -> String {
        env::var("NESTGATE_API_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string())
    }

    /// Get the default streaming RPC address
    pub fn default_streaming_rpc_address() -> String {
        env::var("NESTGATE_STREAMING_RPC_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8081".to_string())
    }

    /// Get the default WebSocket address
    pub fn default_websocket_address() -> String {
        env::var("NESTGATE_WEBSOCKET_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8082".to_string())
    }

    /// Get the default server hostname
    pub fn default_hostname() -> String {
        env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
    }

    /// Get the default external hostname for client connections
    pub fn default_external_hostname() -> String {
        env::var("NESTGATE_EXTERNAL_HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
    }

    /// Get the default API port
    pub fn default_api_port() -> u16 {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080)
    }

    /// Get the default streaming RPC port
    pub fn default_streaming_rpc_port() -> u16 {
        env::var("NESTGATE_STREAMING_RPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8081)
    }

    /// Get the default WebSocket port
    pub fn default_websocket_port() -> u16 {
        env::var("NESTGATE_WEBSOCKET_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8082)
    }

    /// Get the default web interface port
    pub fn default_web_port() -> u16 {
        env::var("NESTGATE_WEB_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000)
    }

    /// Get the NAS bind address
    pub fn default_nas_bind_address() -> String {
        env::var("NESTGATE_NAS_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string())
    }

    /// Get the health check URL
    pub fn default_health_check_url() -> String {
        let hostname = default_external_hostname();
        let port = default_api_port();
        env::var("NESTGATE_HEALTH_CHECK_URL")
            .unwrap_or_else(|_| format!("http://{}:{}/health", hostname, port))
    }

    /// Get API base URL
    pub fn default_api_base_url() -> String {
        let hostname = default_external_hostname();
        let port = default_api_port();
        env::var("NESTGATE_API_BASE_URL")
            .unwrap_or_else(|_| format!("http://{}:{}", hostname, port))
    }
}

/// Service endpoint configuration
pub mod endpoints {
    use super::*;

    /// Get BearDog service endpoint
    pub fn beardog_endpoint() -> String {
        env::var("BEARDOG_URL").unwrap_or_else(|_| "http://beardog:8443".to_string())
    }

    /// Get Songbird service endpoint
    pub fn songbird_endpoint() -> String {
        env::var("SONGBIRD_URL").unwrap_or_else(|_| "http://songbird:8080".to_string())
    }

    /// Get Squirrel AI service endpoint
    pub fn squirrel_endpoint() -> String {
        env::var("SQUIRREL_URL").unwrap_or_else(|_| "http://squirrel:8080".to_string())
    }

    /// Get Toadstool compute service endpoint
    pub fn toadstool_endpoint() -> String {
        env::var("TOADSTOOL_URL").unwrap_or_else(|_| "http://toadstool-compute:8080".to_string())
    }

    /// Get orchestration service endpoint
    pub fn orchestration_endpoint() -> String {
        env::var("NESTGATE_ORCHESTRATION_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
    }

    /// Get BiomeOS service endpoint
    pub fn biomeos_endpoint() -> String {
        env::var("BIOMEOS_URL").unwrap_or_else(|_| "http://localhost:4000".to_string())
    }

    /// Get storage node endpoint
    pub fn storage_node_endpoint() -> String {
        env::var("NESTGATE_STORAGE_NODE_URL")
            .unwrap_or_else(|_| "http://storage-node:8080".to_string())
    }
}

/// File system paths configuration
pub mod paths {
    use super::*;

    /// Get the default cache directory
    pub fn default_cache_dir() -> PathBuf {
        env::var("NESTGATE_CACHE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = env::temp_dir();
                path.push("nestgate_cache");
                path
            })
    }

    /// Get the default temporary directory
    pub fn default_temp_dir() -> PathBuf {
        env::var("NESTGATE_TEMP_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = env::temp_dir();
                path.push("nestgate_tmp");
                path
            })
    }

    /// Get the default data directory
    pub fn default_data_dir() -> PathBuf {
        env::var("NESTGATE_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                if let Ok(home) = env::var("HOME") {
                    let mut path = PathBuf::from(home);
                    path.push(".nestgate");
                    path.push("data");
                    path
                } else {
                    PathBuf::from("/var/lib/nestgate/data")
                }
            })
    }

    /// Get the default configuration directory
    pub fn default_config_dir() -> PathBuf {
        env::var("NESTGATE_CONFIG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                if let Ok(home) = env::var("HOME") {
                    let mut path = PathBuf::from(home);
                    path.push(".nestgate");
                    path.push("config");
                    path
                } else {
                    PathBuf::from("/etc/nestgate")
                }
            })
    }

    /// Get the default log directory
    pub fn default_log_dir() -> PathBuf {
        env::var("NESTGATE_LOG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                if let Ok(home) = env::var("HOME") {
                    let mut path = PathBuf::from(home);
                    path.push(".nestgate");
                    path.push("logs");
                    path
                } else {
                    PathBuf::from("/var/log/nestgate")
                }
            })
    }

    /// Get the default HuggingFace cache directory
    pub fn default_huggingface_cache_dir() -> PathBuf {
        env::var("NESTGATE_HF_CACHE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = default_cache_dir();
                path.push("huggingface");
                path
            })
    }

    /// Get the default NFS exports path
    pub fn default_nfs_exports_path() -> PathBuf {
        env::var("NESTGATE_NFS_EXPORTS_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = default_temp_dir();
                path.push("nestgate_exports");
                path
            })
    }

    /// Get the default share root directory
    pub fn default_share_root() -> PathBuf {
        env::var("NESTGATE_SHARE_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = default_data_dir();
                path.push("shares");
                path
            })
    }
}

/// Authentication and security configuration
pub mod security {
    use super::*;

    /// Get BearDog API key
    pub fn beardog_api_key() -> String {
        env::var("BEARDOG_API_KEY").unwrap_or_else(|_| {
            tracing::warn!("BEARDOG_API_KEY not set, using default (not secure for production)");
            "default_beardog_key".to_string()
        })
    }

    /// Get Songbird API key
    pub fn songbird_api_key() -> String {
        env::var("SONGBIRD_API_KEY").unwrap_or_else(|_| {
            tracing::warn!("SONGBIRD_API_KEY not set, using default (not secure for production)");
            "default_songbird_key".to_string()
        })
    }

    /// Get Squirrel AI API key
    pub fn squirrel_api_key() -> String {
        env::var("SQUIRREL_API_KEY").unwrap_or_else(|_| {
            tracing::warn!("SQUIRREL_API_KEY not set, using default (not secure for production)");
            "default_squirrel_key".to_string()
        })
    }

    /// Get Toadstool compute API key
    pub fn toadstool_api_key() -> String {
        env::var("TOADSTOOL_API_KEY").unwrap_or_else(|_| {
            tracing::warn!("TOADSTOOL_API_KEY not set, using default (not secure for production)");
            "default_toadstool_key".to_string()
        })
    }

    /// Get HuggingFace API token
    pub fn huggingface_api_token() -> Option<String> {
        env::var("HUGGINGFACE_API_TOKEN").ok()
    }

    /// Get NCBI API key
    pub fn ncbi_api_key() -> Option<String> {
        env::var("NCBI_API_KEY").ok()
    }

    /// Get default BearDog validation token (for testing only)
    pub fn default_beardog_validation_token() -> String {
        env::var("BEARDOG_VALIDATION_TOKEN").unwrap_or_else(|_| {
            if is_production() {
                panic!("BEARDOG_VALIDATION_TOKEN must be set in production");
            }
            tracing::warn!("Using default BearDog validation token (testing only)");
            "test_token".to_string()
        })
    }

    /// Get JWT secret key
    pub fn jwt_secret() -> String {
        env::var("NESTGATE_JWT_SECRET").unwrap_or_else(|_| {
            if is_production() {
                panic!("NESTGATE_JWT_SECRET must be set in production");
            }
            tracing::warn!("Using default JWT secret (not secure for production)");
            "default_jwt_secret_change_in_production".to_string()
        })
    }

    /// Get encryption key
    pub fn encryption_key() -> String {
        env::var("NESTGATE_ENCRYPTION_KEY").unwrap_or_else(|_| {
            if is_production() {
                panic!("NESTGATE_ENCRYPTION_KEY must be set in production");
            }
            tracing::warn!("Using default encryption key (not secure for production)");
            "default_encryption_key_change_in_production".to_string()
        })
    }
}

/// Environment detection
pub mod environment {
    use super::*;

    /// Check if running in production mode
    pub fn is_production() -> bool {
        env::var("NESTGATE_ENV")
            .map(|env| env.to_lowercase() == "production")
            .unwrap_or(false)
    }

    /// Check if running in development mode
    pub fn is_development() -> bool {
        env::var("NESTGATE_ENV")
            .map(|env| env.to_lowercase() == "development")
            .unwrap_or(true)
    }

    /// Check if running in test mode
    pub fn is_test() -> bool {
        env::var("NESTGATE_ENV")
            .map(|env| env.to_lowercase() == "test")
            .unwrap_or(false)
    }

    /// Get the current environment
    pub fn current_environment() -> String {
        env::var("NESTGATE_ENV").unwrap_or_else(|_| "development".to_string())
    }
}

/// Feature flags configuration
pub mod features {
    use super::*;

    /// Check if AI features are enabled
    pub fn ai_features_enabled() -> bool {
        env::var("NESTGATE_AI_FEATURES")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true)
    }

    /// Check if ZFS features are enabled
    pub fn zfs_features_enabled() -> bool {
        env::var("NESTGATE_ZFS_FEATURES")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true)
    }

    /// Check if networking features are enabled
    pub fn network_features_enabled() -> bool {
        env::var("NESTGATE_NETWORK_FEATURES")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true)
    }

    /// Check if web interface is enabled
    pub fn web_interface_enabled() -> bool {
        env::var("NESTGATE_WEB_INTERFACE")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true)
    }

    /// Check if metrics collection is enabled
    pub fn metrics_enabled() -> bool {
        env::var("NESTGATE_METRICS")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true)
    }

    /// Check if debug logging is enabled
    pub fn debug_logging_enabled() -> bool {
        env::var("NESTGATE_DEBUG_LOGGING")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false)
    }
}

/// Database configuration
pub mod database {
    use super::*;

    /// Get database URL
    pub fn database_url() -> String {
        env::var("NESTGATE_DATABASE_URL").unwrap_or_else(|_| {
            let data_dir = super::paths::default_data_dir();
            format!("sqlite://{}/nestgate.db", data_dir.display())
        })
    }

    /// Get database connection pool size
    pub fn database_pool_size() -> u32 {
        env::var("NESTGATE_DATABASE_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10)
    }
}

/// Performance tuning configuration
pub mod performance {
    use super::*;

    /// Get the number of worker threads
    pub fn worker_threads() -> usize {
        env::var("NESTGATE_WORKER_THREADS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(num_cpus::get)
    }

    /// Get the request timeout in seconds
    pub fn request_timeout_seconds() -> u64 {
        env::var("NESTGATE_REQUEST_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30)
    }

    /// Get the connection timeout in seconds
    pub fn connection_timeout_seconds() -> u64 {
        env::var("NESTGATE_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5)
    }

    /// Get the maximum concurrent connections
    pub fn max_concurrent_connections() -> usize {
        env::var("NESTGATE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000)
    }
}

/// Utility functions
pub use environment::is_production;

/// Centralized configuration structure
#[derive(Debug, Clone)]
pub struct NestGateConfig {
    pub bind_address: String,
    pub api_address: String,
    pub streaming_rpc_address: String,
    pub websocket_address: String,
    pub hostname: String,
    pub external_hostname: String,
    pub api_port: u16,
    pub streaming_rpc_port: u16,
    pub websocket_port: u16,
    pub web_port: u16,
    pub cache_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub log_dir: PathBuf,
    pub beardog_api_key: String,
    pub songbird_api_key: String,
    pub squirrel_api_key: String,
    pub toadstool_api_key: String,
    pub huggingface_api_token: Option<String>,
    pub ncbi_api_key: Option<String>,
    pub jwt_secret: String,
    pub encryption_key: String,
    pub environment: String,
    pub ai_features_enabled: bool,
    pub zfs_features_enabled: bool,
    pub network_features_enabled: bool,
    pub web_interface_enabled: bool,
    pub metrics_enabled: bool,
    pub debug_logging_enabled: bool,
    pub database_url: String,
    pub database_pool_size: u32,
    pub worker_threads: usize,
    pub request_timeout_seconds: u64,
    pub connection_timeout_seconds: u64,
    pub max_concurrent_connections: usize,
}

impl Default for NestGateConfig {
    fn default() -> Self {
        Self {
            bind_address: network::default_bind_address(),
            api_address: network::default_api_address(),
            streaming_rpc_address: network::default_streaming_rpc_address(),
            websocket_address: network::default_websocket_address(),
            hostname: network::default_hostname(),
            external_hostname: network::default_external_hostname(),
            api_port: network::default_api_port(),
            streaming_rpc_port: network::default_streaming_rpc_port(),
            websocket_port: network::default_websocket_port(),
            web_port: network::default_web_port(),
            cache_dir: paths::default_cache_dir(),
            temp_dir: paths::default_temp_dir(),
            data_dir: paths::default_data_dir(),
            config_dir: paths::default_config_dir(),
            log_dir: paths::default_log_dir(),
            beardog_api_key: security::beardog_api_key(),
            songbird_api_key: security::songbird_api_key(),
            squirrel_api_key: security::squirrel_api_key(),
            toadstool_api_key: security::toadstool_api_key(),
            huggingface_api_token: security::huggingface_api_token(),
            ncbi_api_key: security::ncbi_api_key(),
            jwt_secret: security::jwt_secret(),
            encryption_key: security::encryption_key(),
            environment: environment::current_environment(),
            ai_features_enabled: features::ai_features_enabled(),
            zfs_features_enabled: features::zfs_features_enabled(),
            network_features_enabled: features::network_features_enabled(),
            web_interface_enabled: features::web_interface_enabled(),
            metrics_enabled: features::metrics_enabled(),
            debug_logging_enabled: features::debug_logging_enabled(),
            database_url: database::database_url(),
            database_pool_size: database::database_pool_size(),
            worker_threads: performance::worker_threads(),
            request_timeout_seconds: performance::request_timeout_seconds(),
            connection_timeout_seconds: performance::connection_timeout_seconds(),
            max_concurrent_connections: performance::max_concurrent_connections(),
        }
    }
}

impl NestGateConfig {
    /// Create a new configuration with all defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Validate the configuration
    pub fn validate(&self) -> std::result::Result<(), String> {
        // Validate network addresses
        if let Err(e) = SocketAddr::from_str(&self.bind_address) {
            return Err(format!("Invalid bind address: {}", e));
        }

        if let Err(e) = SocketAddr::from_str(&self.api_address) {
            return Err(format!("Invalid API address: {}", e));
        }

        if let Err(e) = SocketAddr::from_str(&self.streaming_rpc_address) {
            return Err(format!("Invalid streaming RPC address: {}", e));
        }

        if let Err(e) = SocketAddr::from_str(&self.websocket_address) {
            return Err(format!("Invalid WebSocket address: {}", e));
        }

        // Validate ports
        if self.api_port == 0
            || self.streaming_rpc_port == 0
            || self.websocket_port == 0
            || self.web_port == 0
        {
            return Err("Ports must be non-zero".to_string());
        }

        // Validate directories exist or can be created
        for dir in [
            &self.cache_dir,
            &self.temp_dir,
            &self.data_dir,
            &self.config_dir,
            &self.log_dir,
        ] {
            if let Some(parent) = dir.parent() {
                if !parent.exists() {
                    return Err(format!(
                        "Parent directory does not exist: {}",
                        parent.display()
                    ));
                }
            }
        }

        // Validate security settings in production
        if is_production() {
            if self.jwt_secret.starts_with("default_") {
                return Err("JWT secret must be set in production".to_string());
            }
            if self.encryption_key.starts_with("default_") {
                return Err("Encryption key must be set in production".to_string());
            }
            if self.beardog_api_key.starts_with("default_") {
                return Err("BearDog API key must be set in production".to_string());
            }
        }

        Ok(())
    }
}
