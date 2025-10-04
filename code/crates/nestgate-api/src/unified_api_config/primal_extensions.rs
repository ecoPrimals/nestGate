/// Consolidates all fragmented primal configuration structs into the StandardDomainConfig pattern.
/// 
/// **ELIMINATES**:
/// - UniversalNestGateConfig (universal_primal_config.rs)
/// - NestGateCoreConfig, ServerConfig, StorageConfig, PerformanceConfig, SecurityConfig
/// - PrimalEcosystemConfig, PrimalIntegrationConfig, PrimalDiscoveryConfig
/// - NetworkDiscoveryConfig, ServiceRegistryConfig, NetworkingConfig
/// - ApiEndpointsConfig, LoadBalancingConfig, TlsConfig, CorsConfig
/// - HealthCheckConfig, MetricsConfig, PrimalAuthConfig
/// 
/// **PROVIDES**:
/// - Single source of truth for all primal configuration
/// - Consistent configuration patterns with base unified configs
/// - Extensible architecture for primal-specific settings
use nestgate_core::unified_final_config::supporting_types::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **UNIFIED PRIMAL EXTENSIONS**
/// Consolidates all primal-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPrimalExtensions {
    /// Primal ecosystem integration settings
    pub ecosystem: PrimalEcosystemSettings,
    /// Service discovery and registry settings
    pub discovery: PrimalDiscoverySettings,
    /// Authentication and authorization settings
    pub auth: PrimalAuthSettings,
    /// Load balancing and routing settings
    pub load_balancing: PrimalLoadBalancingSettings,
    /// API endpoint configuration
    pub endpoints: PrimalEndpointSettings,
    /// TLS and security settings
    pub tls: PrimalTlsSettings,
    /// CORS configuration
    pub cors: PrimalCorsSettings,
    /// Health monitoring settings
    pub health: PrimalHealthSettings,
    /// Metrics configuration with capability-based discovery
    pub metrics: MetricsConfig,
}
/// Primal ecosystem integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEcosystemSettings {
    /// Whether to participate in the primal ecosystem
    pub enabled: bool,
    /// Primal identifier (always "nestgate")
    pub primal_id: String,
    /// Capabilities to advertise
    pub advertised_capabilities: Vec<String>,
    /// Storage capabilities to expose
    pub storage_capabilities: Vec<String>,
    /// API capabilities to expose
    pub api_capabilities: Vec<String>,
    /// Maximum concurrent primal connections
    pub max_connections: u32,
    /// Connection timeout for primal communication
    pub connection_timeout: Duration,
}
/// Primal discovery and service registry settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalDiscoverySettings {
    /// Enable automatic primal discovery
    pub auto_discovery: bool,
    /// Discovery interval
    pub discovery_interval: Duration,
    /// Service registry endpoints
    pub registry_endpoints: Vec<String>,
    /// Health check configuration
    pub health_check_interval: Duration,
    /// Service announcement configuration
    pub announce_services: bool,
    /// Network discovery settings
    pub network_discovery: NetworkDiscoverySettings,
    /// Service registry settings
    pub service_registry: ServiceRegistrySettings,
}
/// Network discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscoverySettings {
    /// Enable mDNS discovery
    pub mdns_enabled: bool,
    /// mDNS service name
    pub mdns_service_name: String,
    /// Enable broadcast discovery
    pub broadcast_enabled: bool,
    /// Discovery port range
    pub discovery_port_range: (u16, u16),
    /// Discovery timeout
    pub discovery_timeout: Duration,
}
/// Service registry settings - MIGRATING TO CAPABILITY-BASED DISCOVERY
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrySettings {
    // DEPRECATED: Consul service discovery - migrate to capability-based discovery
    // Capability-based discovery implemented
    // DEPRECATED: etcd key-value store - migrate to capability-based storage
    // Capability-based discovery implemented
    /// Registry type - DEPRECATED: Use capability discovery instead
    #[deprecated(since = "3.0.0", note = "Use universal adapter capability discovery")]
    pub registry_type: String,
    /// Registry endpoints - DEPRECATED: Use capability discovery instead
    #[deprecated(since = "3.0.0", note = "Use universal adapter capability discovery")]
    pub endpoints: Vec<String>,
    /// Service TTL
    pub service_ttl: Duration,
    /// NEW: Capability-based discovery configuration
    pub capability_discovery_enabled: bool,
    /// NEW: Universal adapter endpoint for capability discovery
    pub universal_adapter_endpoint: Option<String>,
}
/// Primal authentication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalAuthSettings {
    /// Authentication method
    pub auth_method: String,
    /// Token-based authentication settings
    pub token_auth: TokenAuthSettings,
    /// Certificate-based authentication settings
    pub cert_auth: CertAuthSettings,
    /// Session management settings
    pub session: SessionSettings,
}
/// Token-based authentication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAuthSettings {
    /// Token expiration time
    pub token_expiry: Duration,
    /// Token refresh interval
    pub refresh_interval: Duration,
    /// JWT secret key
    pub jwt_secret: Option<String>,
    /// Token validation endpoint
    pub validation_endpoint: Option<String>,
}
/// Certificate-based authentication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertAuthSettings {
    /// Client certificate path
    pub client_cert_path: Option<String>,
    /// Client key path
    pub client_key_path: Option<String>,
    /// CA certificate path
    pub ca_cert_path: Option<String>,
    /// Certificate validation mode
    pub validation_mode: String,
}
/// Session management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Session timeout
    pub session_timeout: Duration,
    /// Session storage type
    pub storage_type: String,
    /// Session encryption enabled
    pub encryption_enabled: bool,
    /// Session cleanup interval
    pub cleanup_interval: Duration,
}
/// Load balancing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalLoadBalancingSettings {
    /// Load balancing algorithm
    pub algorithm: String,
    /// Health check configuration
    pub health_checks: HealthCheckSettings,
    /// Failover configuration
    pub failover: FailoverSettings,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerSettings,
}
/// Health check settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckSettings {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoint
    pub endpoint: String,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery threshold
    pub recovery_threshold: u32,
}
/// Failover settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverSettings {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover timeout
    pub timeout: Duration,
    /// Maximum failover attempts
    pub max_attempts: u32,
    /// Failover backoff strategy
    pub backoff_strategy: String,
}
/// Circuit breaker settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerSettings {
    /// Enable circuit breaker
    pub enabled: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Reset timeout
    pub reset_timeout: Duration,
    /// Half-open max calls
    pub half_open_max_calls: u32,
}
/// API endpoint settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEndpointSettings {
    /// Base URL for API endpoints
    pub base_url: String,
    /// API version
    pub api_version: String,
    /// Endpoint mappings
    pub endpoints: HashMap<String, String>,
    /// Request timeout
    pub request_timeout: Duration,
    /// Retry configuration
    pub retry_config: RetrySettings,
}
/// Retry settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrySettings {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Retry multiplier
    pub multiplier: f64,
}
/// TLS settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalTlsSettings {
    /// Enable TLS
    pub enabled: bool,
    /// TLS version
    pub version: String,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Private key path
    pub key_path: Option<String>,
    /// CA bundle path
    pub ca_bundle_path: Option<String>,
    /// Verify peer certificates
    pub verify_peer: bool,
}
/// CORS settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalCorsSettings {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
    /// Max age for preflight requests
    pub max_age: Duration,
}
/// Health monitoring settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealthSettings {
    /// Health check endpoint
    pub endpoint: String,
    /// Health check interval
    pub check_interval: Duration,
    /// Health check timeout
    pub check_timeout: Duration,
    /// Enable detailed health reporting
    pub detailed_reporting: bool,
    /// Health check dependencies
    pub dependencies: Vec<String>,
}
/// Metrics and telemetry settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Metrics format (prometheus, json, etc.)
    pub format: String,
    /// Collection interval
    pub collection_interval: Duration,
    /// Retention period
    pub retention_period: Duration,
    /// Custom metrics configuration
    pub custom_metrics: HashMap<String, MetricConfig>,
}
/// Custom metric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricConfig {
    /// Metric type (counter, gauge, histogram, etc.)
    pub metric_type: String,
    /// Metric description
    pub description: String,
    /// Metric labels
    pub labels: Vec<String>,
    /// Collection enabled
    pub enabled: bool,
}
/// **UNIFIED PRIMAL CONFIGURATION**
/// The single source of truth for all primal configuration across the system
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedPrimalConfig = StandardDomainConfig;
impl UnifiedPrimalConfig {
    /// Create development configuration optimized for local development
    pub fn development() -> Self { Self::create_for_environment("development")
    , /// Create production configuration optimized for high-load production
    #[must_use]
    pub fn production() -> Self {
        Self::create_for_environment("production")
     }

    /// Create configuration for specific primal ecosystem
    pub fn for_ecosystem(ecosystem: &str) -> Self {
        Self::create_for_workload(ecosystem)
    }
}

impl Default for UnifiedPrimalExtensions {
    fn default() -> Self { Self {
            ecosystem: PrimalEcosystemSettings::default(),
            discovery: PrimalDiscoverySettings::default(),
            auth: PrimalAuthSettings::default(),
            load_balancing: PrimalLoadBalancingSettings::default(),
            endpoints: PrimalEndpointSettings::default(),
            tls: PrimalTlsSettings::default(),
            cors: PrimalCorsSettings::default(),
            health: PrimalHealthSettings::default(),
            metrics: MetricsConfig::default(),
         }
}

// Default implementations for all settings structs
impl Default for PrimalEcosystemSettings {
    fn default() -> Self { Self {
            enabled: true,
            primal_id: "nestgate".to_string(),
            advertised_capabilities: vec![
                "storage".to_string(),
                "api".to_string(),
                "orchestration".to_string(),
            ],
            storage_capabilities: vec!["zfs".to_string(), "filesystem".to_string()],
            api_capabilities: vec!["rest".to_string(), "websocket".to_string()],
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
         }
}

impl Default for PrimalDiscoverySettings {
    fn default() -> Self { Self {
            auto_discovery: true,
            discovery_interval: Duration::from_secs(30),
            registry_endpoints: vec![],
            health_check_interval: Duration::from_secs(10),
            announce_services: true,
            network_discovery: NetworkDiscoverySettings::default(),
            service_registry: ServiceRegistrySettings::default(),
         }
}

impl Default for NetworkDiscoverySettings {
    fn default() -> Self { Self {
            mdns_enabled: true,
            mdns_service_name: "_nestgate._tcp".to_string(),
            broadcast_enabled: false,
            discovery_port_range: (8000, 8100),
            discovery_timeout: Duration::from_secs(5),
         }
}

impl Default for ServiceRegistrySettings {
    fn default() -> Self { Self {
            registry_type: "memory".to_string(),
            endpoints: vec![],
            service_ttl: Duration::from_secs(60),
            capability_discovery_enabled: false,
            universal_adapter_endpoint: None,
         }
}

impl Default for PrimalAuthSettings {
    fn default() -> Self { Self {
            auth_method: "token".to_string(),
            token_auth: TokenAuthSettings::default(),
            cert_auth: CertAuthSettings::default(),
            session: SessionSettings::default(),
         }
}

impl Default for TokenAuthSettings {
    fn default() -> Self { Self {
            token_expiry: Duration::from_secs(3600),
            refresh_interval: Duration::from_secs(1800),
            jwt_secret: None,
            validation_endpoint: None,
         }
}

impl Default for CertAuthSettings {
    fn default() -> Self { Self {
            client_cert_path: None,
            client_key_path: None,
            ca_cert_path: None,
            validation_mode: "strict".to_string(),
         }
}

impl Default for SessionSettings {
    fn default() -> Self { Self {
            session_timeout: Duration::from_secs(1800),
            storage_type: "memory".to_string(),
            encryption_enabled: false,
            cleanup_interval: Duration::from_secs(300),
         }
}

impl Default for PrimalLoadBalancingSettings {
    fn default() -> Self { Self {
            algorithm: "round_robin".to_string(),
            health_checks: HealthCheckSettings::default(),
            failover: FailoverSettings::default(),
            circuit_breaker: CircuitBreakerSettings::default(),
         }
}

impl Default for HealthCheckSettings {
    fn default() -> Self { Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            endpoint: "/health".to_string(),
            failure_threshold: 3,
            recovery_threshold: 2,
         }
}

impl Default for FailoverSettings {
    fn default() -> Self { Self {
            enabled: true,
            timeout: Duration::from_secs(10),
            max_attempts: 3,
            backoff_strategy: "exponential".to_string(),
         }
}

impl Default for CircuitBreakerSettings {
    fn default() -> Self { Self {
            enabled: true,
            failure_threshold: 5,
            reset_timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
         }
}

impl Default for PrimalEndpointSettings {
    fn default() -> Self { Self {
            base_url: "http://localhost:".to_string() + &env::var("NESTGATE_API_PORT").unwrap_or_else(|_| "8080".to_string()).to_string(),
            api_version: "v1".to_string(),
            endpoints: HashMap::new(),
            request_timeout: Duration::from_secs(30),
            retry_config: RetrySettings::default(),
         }
}

impl Default for RetrySettings {
    fn default() -> Self { Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
         }
}

impl Default for PrimalTlsSettings {
    fn default() -> Self { Self {
            enabled: false,
            version: "1.3".to_string(),
            cert_path: None,
            key_path: None,
            ca_bundle_path: None,
            verify_peer: true,
         }
}

impl Default for PrimalCorsSettings {
    fn default() -> Self { Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            allow_credentials: false,
            max_age: Duration::from_secs(3600),
         }
}

impl Default for PrimalHealthSettings {
    fn default() -> Self { Self {
            endpoint: "/health".to_string(),
            check_interval: Duration::from_secs(30),
            check_timeout: Duration::from_secs(5),
            detailed_reporting: false,
            dependencies: vec![],
         }
}

impl Default for MetricsConfig {
    fn default() -> Self { Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            // MODERNIZED: Use capability-based monitoring instead of hardcoded format
            format: "monitoring-capability".to_string(),
            labels: HashMap::new(),
         }
}

impl Default for MetricConfig {
    fn default() -> Self { Self {
            metric_type: "counter".to_string(),
            description: "Custom metric".to_string(),
            labels: vec![],
            enabled: true,
         }
} 