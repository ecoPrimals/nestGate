/// **UNIFIED DYNAMIC DISCOVERY CONFIGURATION**
/// Consolidates all fragmented Dynamic*Config structs into a single, comprehensive
/// dynamic discovery system using the universal adapter pattern.
///
/// **ELIMINATES**:
/// - DynamicTimeoutConfig (timeout_migration.rs)
/// - DynamicNetworkConfig (network_migration.rs)
/// - DynamicSecurityConfig (security_migration.rs)
/// - DynamicEnvironmentConfig (env_migration.rs)
///
/// **PROVIDES**:
/// - Single source of truth for all dynamic discovery configuration
/// - Unified adapter integration across all discovery types
/// - Consistent configuration patterns and caching strategies
/// - Extensible architecture for new discovery types
// Using universal adapter from proper module path
use crate::ecosystem_integration::universal_adapter::UniversalAdapter;
use crate::error::Result;
use crate::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Duration;

/// **UNIFIED DYNAMIC DISCOVERY EXTENSIONS**
/// Consolidates all dynamic discovery configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDynamicDiscoveryExtensions {
    /// Timeout discovery settings
    pub timeout: TimeoutDiscoverySettings,
    /// Network discovery settings  
    pub network: NetworkDiscoverySettings,
    /// Security discovery settings
    pub security: SecurityDiscoverySettings,
    /// Environment discovery settings
    pub environment: EnvironmentDiscoverySettings,
    /// Storage discovery settings
    pub storage: StorageDiscoverySettings,
    /// Cache discovery settings
    pub cache: CacheDiscoverySettings,
}

/// Timeout discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutDiscoverySettings {
    /// Enable dynamic timeout discovery
    pub enable_dynamic_timeouts: bool,
    /// Timeout cache TTL
    pub cache_ttl: Duration,
    /// Fallback timeout for unknown services
    pub default_timeout: Duration,
    /// Timeout multiplier for high-load scenarios
    pub load_multiplier: f64,
    /// Enable adaptive timeout adjustment
    pub enable_adaptive_timeouts: bool,
}

/// Network discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscoverySettings {
    /// Enable dynamic network discovery
    pub enable_dynamic_networks: bool,
    /// Network cache TTL
    pub cache_ttl: Duration,
    /// Default bind address for new services
    pub default_bind_address: IpAddr,
    /// Enable IPv6 discovery
    pub enable_ipv6: bool,
    /// Network health check interval
    pub health_check_interval: Duration,
}

/// Security discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDiscoverySettings {
    /// Enable dynamic security discovery
    pub enable_dynamic_security: bool,
    /// Security cache TTL
    pub cache_ttl: Duration,
    /// Enable automatic credential rotation
    pub enable_credential_rotation: bool,
    /// Default security level for new services
    pub default_security_level: String,
    /// Certificate discovery settings
    pub certificate_discovery: CertificateDiscoverySettings,
}

/// Environment discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentDiscoverySettings {
    /// Enable dynamic environment discovery
    pub enable_dynamic_environment: bool,
    /// Environment cache TTL
    pub cache_ttl: Duration,
    /// Environment variable prefixes to discover
    pub discovery_prefixes: Vec<String>,
    /// Enable environment change detection
    pub enable_change_detection: bool,
    /// Default environment for new services
    pub default_environment: String,
}

/// Storage discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDiscoverySettings {
    /// Enable dynamic storage discovery
    pub enable_dynamic_storage: bool,
    /// Storage cache TTL
    pub cache_ttl: Duration,
    /// Default storage tier for new resources
    pub default_tier: String,
    /// Enable storage health monitoring
    pub enable_health_monitoring: bool,
    /// Storage capacity thresholds
    pub capacity_thresholds: StorageCapacityThresholds,
}

/// Cache discovery configuration settings  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDiscoverySettings {
    /// Enable dynamic cache discovery
    pub enable_dynamic_cache: bool,
    /// Cache discovery TTL
    pub cache_ttl: Duration,
    /// Default cache size for new caches
    pub default_cache_size: usize,
    /// Enable cache performance monitoring
    pub enable_performance_monitoring: bool,
    /// Cache eviction policies
    pub eviction_policies: Vec<String>,
}

/// Certificate discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateDiscoverySettings {
    /// Enable automatic certificate discovery
    pub enable_cert_discovery: bool,
    /// Certificate cache TTL
    pub cache_ttl: Duration,
    /// Certificate renewal threshold (days before expiry)
    pub renewal_threshold_days: u32,
    /// Enable automatic certificate renewal
    pub enable_auto_renewal: bool,
}

/// Storage capacity thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapacityThresholds {
    /// Warning threshold (percentage)
    pub warning_threshold: f64,
    /// Critical threshold (percentage)  
    pub critical_threshold: f64,
    /// Emergency threshold (percentage)
    pub emergency_threshold: f64,
}

impl Default for UnifiedDynamicDiscoveryExtensions {
    fn default() -> Self {
        Self {
            timeout: TimeoutDiscoverySettings {
                enable_dynamic_timeouts: true,
                cache_ttl: Duration::from_secs(300), // 5 minutes
                default_timeout: Duration::from_secs(30),
                load_multiplier: 1.5,
                enable_adaptive_timeouts: true,
            },
            network: NetworkDiscoverySettings {
                enable_dynamic_networks: true,
                cache_ttl: Duration::from_secs(300),
                default_bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
                enable_ipv6: true,
                health_check_interval: Duration::from_secs(60),
            },
            security: SecurityDiscoverySettings {
                enable_dynamic_security: true,
                cache_ttl: Duration::from_secs(180), // 3 minutes for security
                enable_credential_rotation: true,
                default_security_level: "standard".to_string(),
                certificate_discovery: CertificateDiscoverySettings {
                    enable_cert_discovery: true,
                    cache_ttl: Duration::from_secs(3600), // 1 hour
                    renewal_threshold_days: 30,
                    enable_auto_renewal: true,
                },
            },
            environment: EnvironmentDiscoverySettings {
                enable_dynamic_environment: true,
                cache_ttl: Duration::from_secs(600), // 10 minutes
                discovery_prefixes: vec![
                    "NESTGATE_".to_string(),
                    "BIOMEOS_".to_string(),
                    "ECOPRIMAL_".to_string(),
                ],
                enable_change_detection: true,
                default_environment: "development".to_string(),
            },
            storage: StorageDiscoverySettings {
                enable_dynamic_storage: true,
                cache_ttl: Duration::from_secs(300),
                default_tier: "standard".to_string(),
                enable_health_monitoring: true,
                capacity_thresholds: StorageCapacityThresholds {
                    warning_threshold: 75.0,
                    critical_threshold: 85.0,
                    emergency_threshold: 95.0,
                },
            },
            cache: CacheDiscoverySettings {
                enable_dynamic_cache: true,
                cache_ttl: Duration::from_secs(300),
                default_cache_size: 1000,
                enable_performance_monitoring: true,
                eviction_policies: vec!["lru".to_string(), "ttl".to_string()],
            },
        }
    }
}

/// **UNIFIED DYNAMIC DISCOVERY CONFIGURATION**
/// Single configuration type that replaces all Dynamic*Config structs
pub type UnifiedDynamicDiscoveryConfig = StandardDomainConfig<UnifiedDynamicDiscoveryExtensions>;

/// **UNIFIED DYNAMIC DISCOVERY MANAGER**
/// Consolidates all dynamic discovery operations into a single manager
pub struct UnifiedDynamicDiscoveryManager {
    config: UnifiedDynamicDiscoveryConfig,
    adapter: Arc<UniversalAdapter>,

    // Unified caches for all discovery types
    timeout_cache: tokio::sync::RwLock<HashMap<String, Duration>>,
    network_cache: tokio::sync::RwLock<HashMap<String, NetworkEndpoint>>,
    security_cache: tokio::sync::RwLock<HashMap<String, SecurityConfiguration>>,
    environment_cache: tokio::sync::RwLock<HashMap<String, EnvironmentConfiguration>>,
    #[allow(dead_code)]
    storage_cache: tokio::sync::RwLock<HashMap<String, StorageConfiguration>>,
    #[allow(dead_code)]
    cache_cache: tokio::sync::RwLock<HashMap<String, CacheConfiguration>>,
}

impl UnifiedDynamicDiscoveryManager {
    /// Create new unified dynamic discovery manager
    pub fn new(config: UnifiedDynamicDiscoveryConfig, adapter: Arc<UniversalAdapter>) -> Self {
        Self {
            config,
            adapter,
            timeout_cache: tokio::sync::RwLock::new(HashMap::new()),
            network_cache: tokio::sync::RwLock::new(HashMap::new()),
            security_cache: tokio::sync::RwLock::new(HashMap::new()),
            environment_cache: tokio::sync::RwLock::new(HashMap::new()),
            storage_cache: tokio::sync::RwLock::new(HashMap::new()),
            cache_cache: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// **UNIFIED TIMEOUT DISCOVERY**
    /// Replaces DynamicTimeoutConfig::discover_timeout_config()
    pub async fn discover_timeout_config(
        &self,
        service_type: &str,
    ) -> Result<TimeoutConfiguration> {
        if !self.config.extensions.timeout.enable_dynamic_timeouts {
            return Ok(TimeoutConfiguration::default_for_service(service_type));
        }

        // Check cache first
        let cache_key = format!("timeout:{service_type}");
        if let Some(cached_timeout) = self.get_cached_timeout(&cache_key).await {
            return Ok(TimeoutConfiguration::from_duration(cached_timeout));
        }

        // Dynamic discovery via universal adapter
        let timeout_caps = self
            .adapter
            .query_capabilities(
                crate::ecosystem_integration::universal_adapter::types::CapabilityQuery::Search(
                    format!("{service_type}-timeout"),
                ),
            )
            .await?;

        let config = if timeout_caps.is_empty() {
            // Use adaptive default based on service type
            TimeoutConfiguration::adaptive_default(
                service_type,
                self.config.extensions.timeout.load_multiplier,
            )
        } else {
            // Extract from discovered capabilities
            TimeoutConfiguration::from_capabilities(&timeout_caps[..])?
        };

        // Cache the result
        self.cache_timeout(&cache_key, config.request_timeout).await;

        Ok(config)
    }

    /// **UNIFIED NETWORK DISCOVERY**
    /// Replaces DynamicNetworkConfig::discover_network_config()
    pub async fn discover_network_config(
        &self,
        service_name: &str,
    ) -> Result<NetworkConfiguration> {
        if !self.config.extensions.network.enable_dynamic_networks {
            return Ok(NetworkConfiguration::default_for_service(service_name));
        }

        // Check cache first
        let cache_key = format!("network:{service_name}");
        if let Some(cached_config) = self.get_cached_network(&cache_key).await {
            return Ok(cached_config);
        }

        // Dynamic discovery via universal adapter
        let network_caps = self
            .adapter
            .query_capabilities(
                crate::ecosystem_integration::universal_adapter::types::CapabilityQuery::Search(
                    format!("{service_name}-network"),
                ),
            )
            .await?;

        let config = if network_caps.is_empty() {
            NetworkConfiguration::default_for_service(service_name)
        } else {
            NetworkConfiguration::from_capabilities(&network_caps[..])?
        };

        // Cache the result
        self.cache_network(&cache_key, config.clone()).await;

        Ok(config)
    }

    /// **UNIFIED SECURITY DISCOVERY**
    /// Replaces DynamicSecurityConfig::discover_security_config()
    pub async fn discover_security_config(
        &self,
        service_name: &str,
    ) -> Result<SecurityConfiguration> {
        if !self.config.extensions.security.enable_dynamic_security {
            return Ok(SecurityConfiguration::default_for_service(service_name));
        }

        // Check cache first
        let cache_key = format!("security:{service_name}");
        if let Some(cached_config) = self.get_cached_security(&cache_key).await {
            return Ok(cached_config);
        }

        // Dynamic discovery via universal adapter
        let security_caps = self
            .adapter
            .query_capabilities(
                crate::ecosystem_integration::universal_adapter::types::CapabilityQuery::Search(
                    format!("{service_name}-security"),
                ),
            )
            .await?;

        let config = if security_caps.is_empty() {
            SecurityConfiguration::default_for_service(service_name)
        } else {
            SecurityConfiguration::from_capabilities(&security_caps[..])?
        };

        // Cache the result
        self.cache_security(&cache_key, config.clone()).await;

        Ok(config)
    }

    /// **UNIFIED ENVIRONMENT DISCOVERY**
    /// Replaces DynamicEnvironmentConfig::discover_environment_config()
    pub async fn discover_environment_config(
        &self,
        service_name: &str,
    ) -> Result<EnvironmentConfiguration> {
        if !self
            .config
            .extensions
            .environment
            .enable_dynamic_environment
        {
            return Ok(EnvironmentConfiguration::default_for_service(service_name));
        }

        // Check cache first
        let cache_key = format!("environment:{service_name}");
        if let Some(cached_config) = self.get_cached_environment(&cache_key).await {
            return Ok(cached_config);
        }

        // Dynamic discovery via universal adapter
        let env_caps = self
            .adapter
            .query_capabilities(
                crate::ecosystem_integration::universal_adapter::types::CapabilityQuery::Search(
                    format!("{service_name}-environment"),
                ),
            )
            .await?;

        let config = if env_caps.is_empty() {
            EnvironmentConfiguration::default_for_service(service_name)
        } else {
            EnvironmentConfiguration::from_capabilities(&env_caps[..])?
        };

        // Cache the result
        self.cache_environment(&cache_key, config.clone()).await;

        Ok(config)
    }

    // Private cache management methods
    async fn get_cached_timeout(&self, key: &str) -> Option<Duration> {
        self.timeout_cache.read().await.get(key).copied()
    }

    async fn cache_timeout(&self, key: &str, timeout: Duration) {
        self.timeout_cache
            .write()
            .await
            .insert(key.to_string(), timeout);
    }

    async fn get_cached_network(&self, key: &str) -> Option<NetworkConfiguration> {
        self.network_cache
            .read()
            .await
            .get(key)
            .map(NetworkConfiguration::from_endpoint)
    }

    async fn cache_network(&self, key: &str, config: NetworkConfiguration) {
        let endpoint = NetworkEndpoint::from_configuration(&config);
        self.network_cache
            .write()
            .await
            .insert(key.to_string(), endpoint);
    }

    async fn get_cached_security(&self, key: &str) -> Option<SecurityConfiguration> {
        self.security_cache.read().await.get(key).cloned()
    }

    async fn cache_security(&self, key: &str, config: SecurityConfiguration) {
        self.security_cache
            .write()
            .await
            .insert(key.to_string(), config);
    }

    async fn get_cached_environment(&self, key: &str) -> Option<EnvironmentConfiguration> {
        self.environment_cache.read().await.get(key).cloned()
    }

    async fn cache_environment(&self, key: &str, config: EnvironmentConfiguration) {
        self.environment_cache
            .write()
            .await
            .insert(key.to_string(), config);
    }
}

// Supporting types that were previously scattered across modules

/// Network endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEndpoint {
    pub address: IpAddr,
    pub port: u16,
    pub protocol: String,
    pub health_endpoint: Option<String>,
}

/// Comprehensive timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfiguration {
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub request_timeout: Duration,
    pub idle_timeout: Duration,
    pub keepalive_timeout: Duration,
    pub shutdown_timeout: Duration,
}

impl TimeoutConfiguration {
    pub fn default_for_service(service_type: &str) -> Self {
        match service_type {
            "database" => Self::database_defaults(),
            "api" | "web" => Self::web_defaults(),
            "cache" => Self::cache_defaults(),
            "storage" => Self::storage_defaults(),
            _ => Self::generic_defaults(),
        }
    }

    pub fn adaptive_default(service_type: &str, load_multiplier: f64) -> Self {
        let mut config = Self::default_for_service(service_type);

        // Apply load multiplier to all timeouts
        config.connect_timeout = Duration::from_millis(
            (config.connect_timeout.as_millis() as f64 * load_multiplier) as u64,
        );
        config.read_timeout = Duration::from_millis(
            (config.read_timeout.as_millis() as f64 * load_multiplier) as u64,
        );
        config.write_timeout = Duration::from_millis(
            (config.write_timeout.as_millis() as f64 * load_multiplier) as u64,
        );
        config.request_timeout = Duration::from_millis(
            (config.request_timeout.as_millis() as f64 * load_multiplier) as u64,
        );

        config
    }

    pub fn from_duration(duration: Duration) -> Self {
        Self {
            connect_timeout: duration,
            read_timeout: duration * 2,
            write_timeout: duration * 2,
            request_timeout: duration,
            idle_timeout: duration * 10,
            keepalive_timeout: duration * 5,
            shutdown_timeout: duration / 2,
        }
    }

    pub fn from_capabilities(
        capabilities: &[crate::ecosystem_integration::universal_adapter::types::Capability],
    ) -> Result<Self> {
        // Extract timeout values from capability metadata
        let mut config = Self::generic_defaults();

        for cap in capabilities {
            let metadata = &cap.metadata;
            if let Some(connect_timeout) = metadata.get("connect_timeout") {
                if let Ok(timeout_secs) = connect_timeout.parse::<u64>() {
                    config.connect_timeout = Duration::from_secs(timeout_secs);
                }
            }
            if let Some(request_timeout) = metadata.get("request_timeout") {
                if let Ok(timeout_secs) = request_timeout.parse::<u64>() {
                    config.request_timeout = Duration::from_secs(timeout_secs);
                }
            }
            // ... extract other timeouts from metadata
        }

        Ok(config)
    }

    fn database_defaults() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            idle_timeout: Duration::from_secs(300),
            keepalive_timeout: Duration::from_secs(60),
            shutdown_timeout: Duration::from_secs(5),
        }
    }

    fn web_defaults() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(60),
            keepalive_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(2),
        }
    }

    fn cache_defaults() -> Self {
        Self {
            connect_timeout: Duration::from_secs(2),
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(120),
            keepalive_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(1),
        }
    }

    fn storage_defaults() -> Self {
        Self {
            connect_timeout: Duration::from_secs(15),
            read_timeout: Duration::from_secs(120),
            write_timeout: Duration::from_secs(120),
            request_timeout: Duration::from_secs(300),
            idle_timeout: Duration::from_secs(600),
            keepalive_timeout: Duration::from_secs(120),
            shutdown_timeout: Duration::from_secs(10),
        }
    }

    fn generic_defaults() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            keepalive_timeout: Duration::from_secs(60),
            shutdown_timeout: Duration::from_secs(5),
        }
    }
}

/// Comprehensive network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    pub bind_address: IpAddr,
    pub port: u16,
    pub enable_tls: bool,
    pub max_connections: usize,
    pub buffer_size: usize,
    pub health_check_path: String,
}

impl NetworkConfiguration {
    pub fn default_for_service(service_name: &str) -> Self {
        let port = crate::universal_primal_discovery::stubs::get_fallback_port(service_name);

        Self {
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            enable_tls: false,
            max_connections: 1000,
            buffer_size: 8192,
            health_check_path: "/health".to_string(),
        }
    }

    pub fn from_endpoint(endpoint: &NetworkEndpoint) -> Self {
        Self {
            bind_address: endpoint.address,
            port: endpoint.port,
            enable_tls: endpoint.protocol == "https",
            max_connections: 1000,
            buffer_size: 8192,
            health_check_path: endpoint
                .health_endpoint
                .clone()
                .unwrap_or("/health".to_string()),
        }
    }

    pub fn from_capabilities(
        capabilities: &[crate::ecosystem_integration::universal_adapter::types::Capability],
    ) -> Result<Self> {
        let mut config = Self {
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 8080,
            enable_tls: false,
            max_connections: 1000,
            buffer_size: 8192,
            health_check_path: "/health".to_string(),
        };

        for cap in capabilities {
            let metadata = &cap.metadata;
            if let Some(port_str) = metadata.get("port") {
                if let Ok(port) = port_str.parse::<u16>() {
                    config.port = port;
                }
            }
            if let Some(timeout_str) = metadata.get("timeout") {
                if let Ok(_timeout_secs) = timeout_str.parse::<u64>() {
                    // Note: NetworkConfiguration doesn't have timeout field, skipping
                }
            }
            // ... extract other network configs from metadata
        }

        Ok(config)
    }
}

impl NetworkEndpoint {
    pub fn from_configuration(config: &NetworkConfiguration) -> Self {
        Self {
            address: config.bind_address,
            port: config.port,
            protocol: if config.enable_tls {
                "https".to_string()
            } else {
                "http".to_string()
            },
            health_endpoint: Some(config.health_check_path.clone()),
        }
    }
}

/// Comprehensive security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    pub enable_auth: bool,
    pub auth_provider: String,
    pub enable_tls: bool,
    pub certificate_path: Option<String>,
    pub allowed_origins: Vec<String>,
    pub rate_limit: Option<u32>,
}

impl SecurityConfiguration {
    pub fn default_for_service(service_name: &str) -> Self {
        match service_name {
            "api" | "web" => Self::web_security_defaults(),
            "admin" => Self::admin_security_defaults(),
            "internal" => Self::internal_security_defaults(),
            _ => Self::standard_security_defaults(),
        }
    }

    pub fn from_capabilities(
        capabilities: &[crate::ecosystem_integration::universal_adapter::types::Capability],
    ) -> Result<Self> {
        let mut config = Self::standard_security_defaults();

        for cap in capabilities {
            let metadata = &cap.metadata;
            if let Some(auth_str) = metadata.get("enable_auth") {
                config.enable_auth = auth_str == "true";
            }
            if let Some(provider) = metadata.get("auth_provider") {
                config.auth_provider = provider.clone();
            }
            if let Some(tls_str) = metadata.get("enable_tls") {
                config.enable_tls = tls_str == "true";
            }
            // ... extract other security settings from metadata
        }

        Ok(config)
    }

    fn web_security_defaults() -> Self {
        Self {
            enable_auth: true,
            auth_provider: "oauth2".to_string(),
            enable_tls: true,
            certificate_path: None,
            allowed_origins: vec!["*".to_string()],
            rate_limit: Some(100),
        }
    }

    fn admin_security_defaults() -> Self {
        Self {
            enable_auth: true,
            auth_provider: "local".to_string(),
            enable_tls: true,
            certificate_path: None,
            allowed_origins: vec!["127.0.0.1".to_string(), "localhost".to_string()],
            rate_limit: Some(50),
        }
    }

    fn internal_security_defaults() -> Self {
        Self {
            enable_auth: false,
            auth_provider: "none".to_string(),
            enable_tls: false,
            certificate_path: None,
            allowed_origins: vec!["127.0.0.1".to_string()],
            rate_limit: None,
        }
    }

    fn standard_security_defaults() -> Self {
        Self {
            enable_auth: true,
            auth_provider: "local".to_string(),
            enable_tls: false,
            certificate_path: None,
            allowed_origins: vec!["*".to_string()],
            rate_limit: Some(100),
        }
    }
}

/// Comprehensive environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfiguration {
    pub environment: String,
    pub variables: HashMap<String, String>,
    pub config_sources: Vec<String>,
    pub enable_hot_reload: bool,
}

impl EnvironmentConfiguration {
    pub fn default_for_service(service_name: &str) -> Self {
        Self {
            environment: "development".to_string(),
            variables: HashMap::new(),
            config_sources: vec!["env".to_string(), "file".to_string()],
            enable_hot_reload: service_name != "production",
        }
    }

    pub fn from_capabilities(
        capabilities: &[crate::ecosystem_integration::universal_adapter::types::Capability],
    ) -> Result<Self> {
        let mut config = Self::default_for_service("generic");

        for cap in capabilities {
            let metadata = &cap.metadata;
            if let Some(env_str) = metadata.get("environment") {
                config.environment = env_str.clone();
            }
            // ... extract other environment settings from metadata
        }

        Ok(config)
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfiguration {
    pub storage_type: String,
    pub capacity_gb: u64,
    pub replication_factor: u32,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

/// Cache configuration  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfiguration {
    pub cache_type: String,
    pub max_size: usize,
    pub ttl: Duration,
    pub eviction_policy: String,
}
