// **MIGRATED**: Using config module's unified_types instead of deprecated root unified_types
use crate::config::canonical_master::{
    MonitoringConfig as UnifiedMonitoringConfig, 
    NetworkConfig as UnifiedNetworkConfig,
};

// **FALLBACK**: Define missing config types locally until they are added to unified_types
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedServiceConfig {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    // Additional fields needed by the codebase
    pub service_name: String,
    pub description: String,
    pub service_type: crate::unified_enums::UnifiedServiceType,
    pub auto_start: bool,
    pub priority: u8,
    pub max_instances: usize,
    pub health_check_enabled: bool,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub timeouts: UnifiedTimeoutConfig,
    pub retry: UnifiedRetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTimeoutConfig {
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub idle_timeout: Duration,
    pub default_timeout: Duration,
}

impl Default for UnifiedTimeoutConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            idle_timeout: Duration::from_secs(300),
            default_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl UnifiedRetryConfig {
    pub fn critical_operations() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }

    pub fn high_frequency() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(50),
            max_delay: Duration::from_secs(2),
            backoff_multiplier: 1.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSecurityConfig {
    pub enable_tls: bool,
    pub verify_certificates: bool,
    pub require_auth: bool,
}

impl Default for UnifiedSecurityConfig {
    fn default() -> Self {
        Self {
            enable_tls: true,
            verify_certificates: true,
            require_auth: true,
        }
    }
}

// Remove duplicate struct definitions - use the imported ones from unified_types

/// Universal Adapter Configuration
/// Configuration structures and settings for the NestGate Universal Adapter.
use serde::{Deserialize, Serialize};
/// **UNIFIED** Universal adapter configuration - consolidated pattern
/// Eliminates duplicate config patterns and uses unified base configurations
#[derive(Debug, Clone)]
pub struct UnifiedAdapterConfig {
    /// Base service configuration (standardized)
    pub service: UnifiedServiceConfig,
    /// Network configuration (standardized)
    pub network: UnifiedNetworkConfig,
    /// Security configuration (standardized)  
    pub security: UnifiedSecurityConfig,
    /// Monitoring configuration (standardized)
    pub monitoring: UnifiedMonitoringConfig,
    /// Adapter-specific extensions
    pub adapter: AdapterExtensions,
}
/// Adapter-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterExtensions {
    /// Service discovery endpoint
    pub discovery_endpoint: String,
    /// Our service registration info
    pub service_registration: ServiceRegistration,
    /// Performance monitoring settings
    pub monitoring_enabled: bool,
    /// Proxy settings if needed
    pub proxy_settings: Option<ProxyConfig>,
}
impl Default for UnifiedAdapterConfig {
    fn default() -> Self {
        let service = UnifiedServiceConfig {
            name: "universal-adapter".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            service_name: "universal-adapter".to_string(),
            description: "Universal Primal Adapter Service".to_string(),
            service_type: crate::unified_enums::UnifiedServiceType::Adapter,
            auto_start: true,
            priority: 8,
            max_instances: 5,
            health_check_enabled: true,
            capabilities: vec![
                "universal-integration".to_string(),
                "primal-discovery".to_string(),
            ],
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
            timeouts: UnifiedTimeoutConfig::default(),
            retry: UnifiedRetryConfig::critical_operations(),
        };
        Self {
            service,
            network: UnifiedNetworkConfig::default(),
            security: UnifiedSecurityConfig::default(),
            monitoring: UnifiedMonitoringConfig::default(),
            adapter: AdapterExtensions {
                discovery_endpoint: "http://localhost:8080/discover".to_string(),
                service_registration: ServiceRegistration::default(),
                monitoring_enabled: true,
                proxy_settings: None,
            },
        }
    }
}

/// Service registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Service version
    pub version: String,
    /// Service maintainer information
    pub maintainer: String,
    /// Service endpoint URL
    pub endpoint: String,
    /// Service health check endpoint
    pub health_endpoint: String,
    /// Service metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Service tags for discovery
    pub tags: Vec<String>,
    /// Service capabilities summary
    pub capabilities_summary: String,
}
impl Default for ServiceRegistration {
    fn default() -> Self {
        Self {
            // SOVEREIGNTY FIX: Use environment-based service identification
            name: std::env::var("NESTGATE_ADAPTER_ID")
                .unwrap_or_else(|_| format!("universal-adapter-{uuid::Uuid::new_v4(}").simple())),
            description: "NestGate Universal Primal Adapter".to_string(),
            version: "2.0.0".to_string(),
            maintainer: "NestGate Team".to_string(),
            // SOVEREIGNTY FIX: Use environment-based endpoint discovery
            endpoint: std::env::var("NESTGATE_ADAPTER_ENDPOINT")
                .unwrap_or_else(|_| "dynamic://capability-discovery".to_string()),
            health_endpoint: std::env::var("NESTGATE_ADAPTER_HEALTH_ENDPOINT")
                .unwrap_or_else(|_| "dynamic://capability-discovery/health".to_string()),
            metadata: std::collections::HashMap::new(),
            tags: vec!["adapter".to_string(), "universal".to_string()],
            capabilities_summary: "Universal primal ecosystem integration".to_string(),
        }
    }
}

/// Proxy configuration for adapter communications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy server address
    pub proxy_endpoint: String,
    /// Proxy authentication if required
    pub proxy_auth: Option<ProxyAuth>,
    /// Bypass proxy for these hosts
    pub no_proxy: Vec<String>,
}
/// Proxy authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyAuth {
    /// Basic username/password auth
    Basic { username: String, password: String },
    /// Bearer token auth
    Bearer { token: String },
}
/// Certificate validation modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateValidation {
    /// No certificate validation
    None,
    /// Basic certificate validation
    Basic,
    /// Strict certificate validation
    Strict,
}
// Type alias for backward compatibility
pub type AdapterConfig = UnifiedAdapterConfig;

impl AdapterConfig {
    /// Create adapter config from environment variables
    pub fn from_environment() -> Self {
        // Return default for now - environment integration can be enhanced later
        Self::default()
    }
}

impl UnifiedAdapterConfig {
    /// Create adapter config optimized for high-availability environments
    #[must_use]
    pub fn high_availability() -> Self {
        let mut config = Self::default();
        config.service.retry = UnifiedRetryConfig::critical_operations();
        config.service.max_instances = 10;
        config.monitoring.enabled = true;
        config.adapter.monitoring_enabled = true;
        config
    }

    /// Create adapter config optimized for development environments
    #[must_use]
    pub fn development() -> Self {
        let mut config = Self::default();
        config.service.retry = UnifiedRetryConfig::high_frequency();
        config.service.max_instances = 2;
        config.security.require_auth = false;
        config.security.enable_tls = false;
        config
    }
}
