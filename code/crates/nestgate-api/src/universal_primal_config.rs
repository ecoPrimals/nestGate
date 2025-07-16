//! Universal Primal Configuration for NestGate
//!
//! This module provides configuration management for NestGate's integration
//! with the universal primal ecosystem (beardog, squirrel, songbird, toadstool).

use crate::universal_primal::StorageCapability;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Universal NestGate Primal Configuration
/// Follows the same pattern as other primal configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalNestGateConfig {
    /// Core NestGate configuration
    pub nestgate: NestGateCoreConfig,

    /// Primal discovery and integration settings
    pub primal_ecosystem: PrimalEcosystemConfig,

    /// Integration settings for specific primals
    pub primal_integrations: HashMap<String, PrimalIntegrationConfig>,

    /// Auto-discovery settings
    pub discovery: PrimalDiscoveryConfig,

    /// Network and communication settings
    pub networking: NetworkingConfig,
}

/// Core NestGate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateCoreConfig {
    /// Server binding configuration
    pub server: ServerConfig,

    /// ZFS storage configuration
    pub storage: StorageConfig,

    /// Performance and optimization settings
    pub performance: PerformanceConfig,

    /// Security settings
    pub security: SecurityConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<u32>,
    pub max_connections: Option<u32>,
    pub request_timeout_ms: Option<u64>,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub zfs_enabled: bool,
    pub default_pool: Option<String>,
    pub auto_snapshots: bool,
    pub compression_enabled: bool,
    pub deduplication_enabled: bool,
    pub encryption_enabled: bool,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub enable_caching: bool,
    pub cache_size_mb: Option<u64>,
    pub io_threads: Option<u32>,
    pub async_queue_size: Option<u32>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_tls: bool,
    pub require_authentication: bool,
    pub audit_logging: bool,
    pub rate_limiting: bool,
}

/// Primal ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEcosystemConfig {
    /// Whether to participate in the primal ecosystem
    pub enabled: bool,

    /// Primal identifier (always "nestgate")
    pub primal_id: String,

    /// Capabilities to advertise
    pub advertised_capabilities: Vec<String>,

    /// Required dependencies from other primals
    pub required_dependencies: Vec<PrimalDependencySpec>,

    /// Optional dependencies from other primals
    pub optional_dependencies: Vec<PrimalDependencySpec>,

    /// Health check configuration
    pub health_check: HealthCheckConfig,

    /// Metrics collection configuration
    pub metrics: MetricsConfig,
}

/// Primal dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalDependencySpec {
    pub primal_type: String, // "security", "ai", "network", "compute"
    pub capabilities: Vec<String>,
    pub minimum_version: Option<String>,
    pub timeout_ms: Option<u64>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub timeout_ms: u64,
    pub failure_threshold: u32,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub collection_interval_seconds: u64,
    pub retention_hours: u64,
    pub export_endpoint: Option<String>,
}

/// Integration configuration for specific primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIntegrationConfig {
    /// Whether this integration is enabled
    pub enabled: bool,

    /// Primal endpoint (auto-discovered if None)
    pub endpoint: Option<String>,

    /// Authentication configuration
    pub auth: Option<PrimalAuthConfig>,

    /// Capabilities to request from this primal
    pub requested_capabilities: Vec<String>,

    /// Custom configuration for this primal
    pub custom_config: HashMap<String, serde_json::Value>,

    /// Integration-specific settings
    pub settings: PrimalIntegrationSettings,
}

/// Authentication configuration for primal integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalAuthConfig {
    pub auth_type: String, // "api_key", "oauth2", "mutual_tls", "bearer_token"
    pub credentials: HashMap<String, String>,
    pub token_refresh_enabled: bool,
}

/// Integration-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIntegrationSettings {
    /// Connection settings
    pub connection_timeout_ms: u64,
    pub request_timeout_ms: u64,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,

    /// Performance settings
    pub connection_pool_size: Option<u32>,
    pub keep_alive_enabled: bool,
    pub compression_enabled: bool,

    /// Security settings
    pub tls_enabled: bool,
    pub certificate_validation: bool,
    pub mutual_tls: bool,
}

/// Primal discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalDiscoveryConfig {
    /// Enable automatic discovery
    pub enabled: bool,

    /// Discovery methods to use
    pub methods: Vec<String>, // "network_scan", "environment", "config", "service_registry"

    /// Network discovery settings
    pub network_discovery: NetworkDiscoveryConfig,

    /// Service registry settings
    pub service_registry: Option<ServiceRegistryConfig>,

    /// Discovery intervals
    pub discovery_interval_seconds: u64,
    pub retry_interval_seconds: u64,
}

/// Network discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscoveryConfig {
    /// Network interfaces to scan
    pub interfaces: Vec<String>,

    /// Port ranges to scan
    pub port_ranges: Vec<PortRange>,

    /// Discovery protocols
    pub protocols: Vec<String>, // "mdns", "upnp", "broadcast"

    /// Scan timeout
    pub scan_timeout_ms: u64,
}

/// Port range for network discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

/// Service registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    pub endpoint: String,
    pub auth: Option<PrimalAuthConfig>,
    pub registration_ttl_seconds: u64,
    pub refresh_interval_seconds: u64,
}

/// Networking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingConfig {
    /// API endpoints configuration
    pub api_endpoints: ApiEndpointsConfig,

    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,

    /// SSL/TLS configuration
    pub tls: TlsConfig,

    /// CORS configuration
    pub cors: CorsConfig,
}

/// API endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointsConfig {
    pub rest_api_enabled: bool,
    pub graphql_api_enabled: bool,
    pub streaming_api_enabled: bool,
    pub websocket_enabled: bool,
    pub sse_enabled: bool,
    pub custom_endpoints: HashMap<String, String>,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub enabled: bool,
    pub algorithm: String, // "round_robin", "least_connections", "weighted"
    pub health_check_enabled: bool,
    pub session_affinity: bool,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub ca_file: Option<String>,
    pub verify_client: bool,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age_seconds: u64,
}

impl Default for UniversalNestGateConfig {
    fn default() -> Self {
        Self {
            nestgate: NestGateCoreConfig {
                server: ServerConfig {
                    host: "0.0.0.0".to_string(),
                    port: 8080,
                    workers: Some(4),
                    max_connections: Some(1000),
                    request_timeout_ms: Some(30000),
                },
                storage: StorageConfig {
                    zfs_enabled: true,
                    default_pool: None,
                    auto_snapshots: true,
                    compression_enabled: true,
                    deduplication_enabled: true,
                    encryption_enabled: false,
                },
                performance: PerformanceConfig {
                    enable_caching: true,
                    cache_size_mb: Some(512),
                    io_threads: Some(8),
                    async_queue_size: Some(1000),
                },
                security: SecurityConfig {
                    enable_tls: false,
                    require_authentication: false,
                    audit_logging: true,
                    rate_limiting: true,
                },
            },
            primal_ecosystem: PrimalEcosystemConfig {
                enabled: true,
                primal_id: "nestgate".to_string(),
                advertised_capabilities: vec![
                    "zfs_storage".to_string(),
                    "high_performance".to_string(),
                    "data_protection".to_string(),
                    "api_endpoints".to_string(),
                    "byob_integration".to_string(),
                ],
                required_dependencies: vec![],
                optional_dependencies: vec![
                    PrimalDependencySpec {
                        primal_type: "security".to_string(),
                        capabilities: vec!["encryption".to_string(), "access_control".to_string()],
                        minimum_version: Some("1.0.0".to_string()),
                        timeout_ms: Some(5000),
                    },
                    PrimalDependencySpec {
                        primal_type: "network".to_string(),
                        capabilities: vec!["service_discovery".to_string()],
                        minimum_version: None,
                        timeout_ms: Some(3000),
                    },
                ],
                health_check: HealthCheckConfig {
                    enabled: true,
                    interval_seconds: 30,
                    timeout_ms: 5000,
                    failure_threshold: 3,
                },
                metrics: MetricsConfig {
                    enabled: true,
                    collection_interval_seconds: 15,
                    retention_hours: 24,
                    export_endpoint: Some("/metrics".to_string()),
                },
            },
            primal_integrations: create_default_primal_integrations(),
            discovery: PrimalDiscoveryConfig {
                enabled: true,
                methods: vec![
                    "environment".to_string(),
                    "network_scan".to_string(),
                    "config".to_string(),
                ],
                network_discovery: NetworkDiscoveryConfig {
                    interfaces: vec!["eth0".to_string(), "wlan0".to_string()],
                    port_ranges: vec![
                        PortRange {
                            start: 8080,
                            end: 8090,
                        },
                        PortRange {
                            start: 3000,
                            end: 3010,
                        },
                    ],
                    protocols: vec!["mdns".to_string()],
                    scan_timeout_ms: 5000,
                },
                service_registry: None,
                discovery_interval_seconds: 60,
                retry_interval_seconds: 30,
            },
            networking: NetworkingConfig {
                api_endpoints: ApiEndpointsConfig {
                    rest_api_enabled: true,
                    graphql_api_enabled: false,
                    streaming_api_enabled: true,
                    websocket_enabled: true,
                    sse_enabled: true,
                    custom_endpoints: HashMap::new(),
                },
                load_balancing: LoadBalancingConfig {
                    enabled: false,
                    algorithm: "round_robin".to_string(),
                    health_check_enabled: true,
                    session_affinity: false,
                },
                tls: TlsConfig {
                    enabled: false,
                    cert_file: None,
                    key_file: None,
                    ca_file: None,
                    verify_client: false,
                },
                cors: CorsConfig {
                    enabled: true,
                    allowed_origins: vec!["*".to_string()],
                    allowed_methods: vec![
                        "GET".to_string(),
                        "POST".to_string(),
                        "PUT".to_string(),
                        "DELETE".to_string(),
                        "OPTIONS".to_string(),
                    ],
                    allowed_headers: vec!["*".to_string()],
                    max_age_seconds: 3600,
                },
            },
        }
    }
}

/// Create default primal integration configurations
fn create_default_primal_integrations() -> HashMap<String, PrimalIntegrationConfig> {
    let mut integrations = HashMap::new();

    // BearDog (Security) integration
    integrations.insert(
        "beardog".to_string(),
        PrimalIntegrationConfig {
            enabled: true,
            endpoint: None, // Auto-discover
            auth: None,     // Use mutual discovery authentication
            requested_capabilities: vec![
                "encryption".to_string(),
                "access_control".to_string(),
                "audit_logging".to_string(),
            ],
            custom_config: HashMap::new(),
            settings: PrimalIntegrationSettings {
                connection_timeout_ms: 5000,
                request_timeout_ms: 10000,
                retry_attempts: 3,
                retry_delay_ms: 1000,
                connection_pool_size: Some(10),
                keep_alive_enabled: true,
                compression_enabled: false, // BearDog handles its own compression
                tls_enabled: true,
                certificate_validation: true,
                mutual_tls: true,
            },
        },
    );

    // Squirrel (AI) integration
    integrations.insert(
        "squirrel".to_string(),
        PrimalIntegrationConfig {
            enabled: true,
            endpoint: None, // Auto-discover
            auth: None,
            requested_capabilities: vec![
                "model_inference".to_string(),
                "agent_framework".to_string(),
                "data_processing".to_string(),
            ],
            custom_config: HashMap::new(),
            settings: PrimalIntegrationSettings {
                connection_timeout_ms: 3000,
                request_timeout_ms: 30000, // AI operations can take longer
                retry_attempts: 2,
                retry_delay_ms: 2000,
                connection_pool_size: Some(5),
                keep_alive_enabled: true,
                compression_enabled: true,
                tls_enabled: false, // Often internal network
                certificate_validation: false,
                mutual_tls: false,
            },
        },
    );

    // Songbird (Distribution) integration
    integrations.insert(
        "songbird".to_string(),
        PrimalIntegrationConfig {
            enabled: true,
            endpoint: None, // Auto-discover
            auth: None,
            requested_capabilities: vec![
                "service_discovery".to_string(),
                "load_balancing".to_string(),
                "network_routing".to_string(),
            ],
            custom_config: HashMap::new(),
            settings: PrimalIntegrationSettings {
                connection_timeout_ms: 2000,
                request_timeout_ms: 5000,
                retry_attempts: 5,
                retry_delay_ms: 500,
                connection_pool_size: Some(20),
                keep_alive_enabled: true,
                compression_enabled: true,
                tls_enabled: false,
                certificate_validation: false,
                mutual_tls: false,
            },
        },
    );

    // Toadstool (Compute) integration
    integrations.insert(
        "toadstool".to_string(),
        PrimalIntegrationConfig {
            enabled: true,
            endpoint: None, // Auto-discover
            auth: None,
            requested_capabilities: vec![
                "container_runtime".to_string(),
                "serverless_execution".to_string(),
                "resource_management".to_string(),
            ],
            custom_config: HashMap::new(),
            settings: PrimalIntegrationSettings {
                connection_timeout_ms: 3000,
                request_timeout_ms: 15000,
                retry_attempts: 3,
                retry_delay_ms: 1000,
                connection_pool_size: Some(15),
                keep_alive_enabled: true,
                compression_enabled: true,
                tls_enabled: false,
                certificate_validation: false,
                mutual_tls: false,
            },
        },
    );

    integrations
}

impl UniversalNestGateConfig {
    /// Load configuration from file with environment variable overrides
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let mut config: Self = toml::from_str(&content)?;

        // Apply environment variable overrides
        config.apply_environment_overrides();

        Ok(config)
    }

    /// Save configuration to file
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    /// Load configuration with secure defaults
    pub fn load_with_secure_defaults() -> Self {
        let mut config = Self::default();

        // Apply secure defaults
        config.nestgate.security.enable_tls = true;
        config.nestgate.security.require_authentication = true;
        config.nestgate.storage.encryption_enabled = true;

        // Enable all security integrations
        if let Some(beardog_config) = config.primal_integrations.get_mut("beardog") {
            beardog_config.settings.tls_enabled = true;
            beardog_config.settings.certificate_validation = true;
            beardog_config.settings.mutual_tls = true;
        }

        config
    }

    /// Apply environment variable overrides
    fn apply_environment_overrides(&mut self) {
        // Server configuration
        if let Ok(host) = std::env::var("NESTGATE_HOST") {
            self.nestgate.server.host = host;
        }
        if let Ok(port) = std::env::var("NESTGATE_PORT") {
            if let Ok(port) = port.parse::<u16>() {
                self.nestgate.server.port = port;
            }
        }

        // Primal ecosystem
        if let Ok(primal_enabled) = std::env::var("NESTGATE_PRIMAL_ECOSYSTEM_ENABLED") {
            self.primal_ecosystem.enabled = primal_enabled.parse().unwrap_or(true);
        }

        // Discovery
        if let Ok(discovery_enabled) = std::env::var("NESTGATE_DISCOVERY_ENABLED") {
            self.discovery.enabled = discovery_enabled.parse().unwrap_or(true);
        }

        // Specific primal endpoints
        for (primal_name, integration) in &mut self.primal_integrations {
            let env_var = format!("NESTGATE_{}_ENDPOINT", primal_name.to_uppercase());
            if let Ok(endpoint) = std::env::var(env_var) {
                integration.endpoint = Some(endpoint);
            }

            let enabled_var = format!("NESTGATE_{}_ENABLED", primal_name.to_uppercase());
            if let Ok(enabled) = std::env::var(enabled_var) {
                integration.enabled = enabled.parse().unwrap_or(true);
            }
        }
    }

    /// Get advertised storage capabilities
    pub fn get_storage_capabilities(&self) -> Vec<StorageCapability> {
        // Convert from configuration to actual capabilities
        // This would be implemented based on actual storage configuration
        vec![]
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate server configuration
        if self.nestgate.server.port == 0 {
            return Err(anyhow::anyhow!("Server port cannot be 0"));
        }

        // Validate primal ecosystem configuration
        if self.primal_ecosystem.enabled && self.primal_ecosystem.primal_id.is_empty() {
            return Err(anyhow::anyhow!(
                "Primal ID cannot be empty when ecosystem is enabled"
            ));
        }

        // Validate discovery configuration
        if self.discovery.enabled && self.discovery.methods.is_empty() {
            return Err(anyhow::anyhow!(
                "At least one discovery method must be specified"
            ));
        }

        // Validate primal integrations
        for (name, integration) in &self.primal_integrations {
            if integration.enabled && integration.settings.connection_timeout_ms == 0 {
                return Err(anyhow::anyhow!(
                    "Connection timeout for {} cannot be 0",
                    name
                ));
            }
        }

        Ok(())
    }
}

/// Example configuration files for different deployment scenarios
/// Generate a basic configuration
pub fn generate_basic_config() -> UniversalNestGateConfig {
    UniversalNestGateConfig::default()
}

/// Generate a production configuration with security hardening
pub fn generate_production_config() -> UniversalNestGateConfig {
    UniversalNestGateConfig::load_with_secure_defaults()
}

/// Generate a development configuration with all features enabled
pub fn generate_development_config() -> UniversalNestGateConfig {
    let mut config = UniversalNestGateConfig::default();

    // Enable all features for development
    config.networking.api_endpoints.graphql_api_enabled = true;
    config.primal_ecosystem.metrics.collection_interval_seconds = 5; // More frequent metrics
    config.discovery.discovery_interval_seconds = 30; // More frequent discovery

    // Relaxed security for development
    config.nestgate.security.require_authentication = false;
    config.networking.cors.allowed_origins = vec!["*".to_string()];

    config
}
