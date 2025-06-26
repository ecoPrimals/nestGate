//! Enhanced Configuration Management for NestGate v2
//! 
//! Advanced configuration capabilities with v2 orchestrator-centric architecture

use serde::{Deserialize, Serialize};
use std::path::Path;
use config::{Config as ConfigBuilder, File, Environment as ConfigEnvironment};
use std::collections::HashMap;
use uuid;

// Re-export from existing error module
use crate::error::{Result, NestGateError};

/// Main configuration structure for the NestGate v2 system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// System-wide settings
    pub system: SystemConfig,
    
    /// Orchestrator configuration (v2 specific)
    pub orchestrator: OrchestratorConfig,
    
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

/// Orchestrator configuration (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    /// Orchestrator bind address
    pub bind_address: String,
    
    /// Orchestrator port
    pub port: u16,
    
    /// Service registry configuration
    pub service_registry: ServiceRegistryConfig,
    
    /// Load balancer configuration
    pub load_balancer: LoadBalancerConfig,
    
    /// Health monitoring configuration
    pub health_monitor: HealthMonitorConfig,
}

/// Service registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    /// Registry type (memory, redis, consul)
    pub registry_type: String,
    
    /// Registry connection string
    pub connection_string: Option<String>,
    
    /// Service timeout in seconds
    pub service_timeout: u64,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancing algorithm
    pub algorithm: String,
    
    /// Health check interval in seconds
    pub health_check_interval: u64,
    
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}

/// Health monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    /// Check interval in seconds
    pub check_interval: u64,
    
    /// Timeout for health checks in seconds
    pub timeout: u64,
    
    /// Number of retries before marking unhealthy
    pub retries: u32,
}

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
    
    /// Encryption algorithm to use
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

// Network configuration constants and utilities
pub const DEFAULT_LOCALHOST: &str = "127.0.0.1";
pub const DEFAULT_ALL_INTERFACES: &str = "0.0.0.0";
pub const DEFAULT_IPV6_LOCALHOST: &str = "::1";
pub const DEFAULT_IPV6_ALL_INTERFACES: &str = "::";

// Default ports for different services
pub mod default_ports {
    pub const ORCHESTRATOR: u16 = 8090;
    pub const API: u16 = 8080;
    pub const MCP: u16 = 8081;
    pub const WEBSOCKET: u16 = 8082;
    pub const METRICS: u16 = 8083;
    pub const HEALTH: u16 = 8084;
    pub const ZFS_API: u16 = 8085;
    pub const NETWORK_SERVICE: u16 = 8086;
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
        self.localhost_only || 
        self.bind_interface == DEFAULT_LOCALHOST || 
        self.bind_interface == DEFAULT_IPV6_LOCALHOST ||
        self.custom_host.as_ref().is_some_and(|h| h == DEFAULT_LOCALHOST || h == DEFAULT_IPV6_LOCALHOST)
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
                bind_interface: std::env::var("SONGBIRD_SERVICE_NAME")
                    .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())),
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
            return Err(NestGateError::Validation("max_concurrent_ops must be greater than 0".to_string()));
        }

        // Validate orchestrator config
        if self.orchestrator.port == 0 {
            return Err(NestGateError::Validation("orchestrator port must be greater than 0".to_string()));
        }

        // Validate storage config
        if self.storage.cache_size == 0 {
            return Err(NestGateError::Validation("cache_size must be greater than 0".to_string()));
        }

        // Validate security config
        if self.security.max_failed_attempts == 0 {
            return Err(NestGateError::Validation("max_failed_attempts must be greater than 0".to_string()));
        }

        // Validate monitoring config
        if self.monitoring.metrics_interval == 0 {
            return Err(NestGateError::Validation("metrics_interval must be greater than 0".to_string()));
        }

        // Validate MCP config if enabled
        if let Some(mcp) = &self.mcp {
            if mcp.enabled && mcp.cluster_endpoint.is_empty() {
                return Err(NestGateError::Validation("MCP cluster_endpoint cannot be empty when MCP is enabled".to_string()));
            }
        }

        // Validate federation config if enabled
        if let Some(federation) = &self.federation {
            if federation.enabled && federation.cluster_name.is_empty() {
                return Err(NestGateError::Validation("Federation cluster_name cannot be empty when federation is enabled".to_string()));
            }
        }

        Ok(())
    }

    /// Save configuration to a file
    pub async fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| NestGateError::Configuration(format!("Failed to serialize config: {}", e)))?;
        
        tokio::fs::write(path, serialized).await
            .map_err(|e| NestGateError::Configuration(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Gets the configuration for a specific environment
    pub fn for_environment(env: &str) -> Result<Self> {
        let env_config = format!("config/{}", env);
        let config_files = vec![
            "config/default",
            &env_config,
            "config/local",
        ];
        
        Self::load_with_sources(config_files)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            system: SystemConfig {
                log_level: "info".to_string(),
                data_dir: "/var/lib/nestgate".to_string(),
                temp_dir: "/tmp/nestgate".to_string(),
                max_concurrent_ops: 1000,
                node_id: "nestgate-node-1".to_string(),
                environment: "development".to_string(),
            },
            orchestrator: OrchestratorConfig {
                bind_address: "0.0.0.0".to_string(),
                port: 8090,
                service_registry: ServiceRegistryConfig {
                    registry_type: "memory".to_string(),
                    connection_string: None,
                    service_timeout: 30,
                },
                load_balancer: LoadBalancerConfig {
                    algorithm: "round_robin".to_string(),
                    health_check_interval: 30,
                    connection_timeout: 10,
                },
                health_monitor: HealthMonitorConfig {
                    check_interval: 30,
                    timeout: 5,
                    retries: 3,
                },
            },
            storage: StorageConfig {
                cache_size: 1024 * 1024 * 1024, // 1GB
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
                log_file: "/var/log/nestgate.log".to_string(),
                log_rotation_size: 1024 * 1024, // 1MB
                log_retention_days: 30,
                prometheus: Some(PrometheusConfig {
                    enabled: true,
                    port: 9090,
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
                enabled: true,
                cluster_endpoint: "localhost:8080".to_string(),
                node_id: "nestgate-node-1".to_string(),
                federation_enabled: false,
                capabilities: McpCapabilitiesConfig {
                    storage_protocols: vec!["nfs".to_string(), "smb".to_string(), "s3".to_string()],
                    storage_tiers: vec!["hot".to_string(), "warm".to_string(), "cold".to_string()],
                    max_volume_size: 1024 * 1024 * 1024 * 1024, // 1TB
                    max_volumes: 1000,
                },
            }),
            federation: Some(FederationConfig {
                enabled: false,
                cluster_name: "nestgate-cluster".to_string(),
                mode: "follower".to_string(),
                peers: vec![],
                heartbeat_interval: 30,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.system.log_level, "info");
        assert_eq!(config.orchestrator.port, 8090);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // Test invalid max_concurrent_ops
        config.system.max_concurrent_ops = 0;
        assert!(config.validate().is_err());
        
        // Reset and test invalid port
        config = Config::default();
        config.orchestrator.port = 0;
        assert!(config.validate().is_err());
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
orchestrator:
  bind_address: "127.0.0.1"
  port: 9090
  service_registry:
    registry_type: "memory"
    service_timeout: 60
  load_balancer:
    algorithm: "least_connections"
    health_check_interval: 15
    connection_timeout: 5
  health_monitor:
    check_interval: 15
    timeout: 3
    retries: 2
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
  cluster_endpoint: "localhost:8080"
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
"#
        ).unwrap();

        // Copy to the new path with .yaml extension
        std::fs::copy(temp_file.path(), &temp_path).unwrap();
        
        let config = Config::load(&temp_path).unwrap();
        assert_eq!(config.system.log_level, "debug");
        assert_eq!(config.orchestrator.port, 9090);
        assert_eq!(config.system.max_concurrent_ops, 500);
        assert!(config.validate().is_ok());
        
        // Clean up
        std::fs::remove_file(&temp_path).unwrap();
    }

    #[tokio::test]
    async fn test_config_file_operations() {
        let config = Config::default();
        let temp_file = NamedTempFile::new()
            .expect("Failed to create temporary file for config test");
        
        // Test saving config
        config.save(temp_file.path()).await
            .expect("Failed to save config to temporary file");

        // Create a permanent temp path for loading test
        let temp_path = temp_file.path().with_extension("test.toml");
        std::fs::copy(temp_file.path(), &temp_path)
            .expect("Failed to copy temp file for loading test");
        
        let loaded_config = Config::load(&temp_path)
            .expect("Failed to load config from temporary file");
        
        assert_eq!(config.system.log_level, loaded_config.system.log_level);

        // Cleanup
        std::fs::remove_file(&temp_path)
            .expect("Failed to cleanup temporary config file");
    }
} 