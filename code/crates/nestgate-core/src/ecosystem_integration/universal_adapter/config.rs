// **MIGRATED**: Using config module's unified_types instead of deprecated root unified_types
//! Config module

use crate::config::canonical_primary::{
    MonitoringConfig as UnifiedMonitoringConfig, 
    NetworkConfig as UnifiedNetworkConfig,
};

// **FALLBACK**: Define missing config types locally until they are added to unified_types
use std::time::Duration;

// Use canonical configs - removed 3 duplicate definitions  
// Service: (17 fields) → canonical ServiceConfig with 7 sub-configs
// Timeout: (4 fields) → canonical TimeoutConfig with 8 timeout types
// Retry: (4 fields) → canonical RetryConfig with comprehensive strategies
use crate::config::canonical_primary::service::UnifiedServiceConfig;
use crate::config::canonical_primary::timeout::UnifiedTimeoutConfig;
use crate::config::canonical_primary::retry::UnifiedRetryConfig;

// Use canonical security config - removed duplicate definition
// (was: simple struct with enable_tls, verify_certificates, require_auth)
// Now using: canonical CanonicalSecurityConfig with full security features
use crate::config::canonical_primary::domains::security_canonical::UnifiedSecurityConfig;

/// Universal Adapter Configuration
/// Configuration structures and settings for the NestGate Universal Adapter.
use serde::{Deserialize, Serialize};
/// **UNIFIED** Universal adapter configuration - consolidated pattern
/// Eliminates duplicate config patterns and uses unified base configurations
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UnifiedAdapterConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UnifiedAdapterConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for UnifiedAdapter
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
/// Adapterextensions
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
    /// Returns the default instance
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
        
        use crate::config::environment::EnvironmentConfig;
        let env_config = EnvironmentConfig::from_env()
            .unwrap_or_else(|_| EnvironmentConfig::default());
        
        let discovery_endpoint = std::env::var("NESTGATE_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| {
                format!(
                    "http://{}:{}/discover",
                    env_config.network.host,
                    env_config.network.port.get()
                )
            });
        
        Self {
            service,
            network: UnifiedNetworkConfig::default(),
            security: UnifiedSecurityConfig::default(),
            monitoring: UnifiedMonitoringConfig::default(),
            adapter: AdapterExtensions {
                discovery_endpoint,
                service_registration: ServiceRegistration::default(),
                monitoring_enabled: true,
                proxy_settings: None,
            },
        }
    }
}

/// Service registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceregistration
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
    /// Returns the default instance
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
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::ProxyConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::ProxyConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Proxy
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
/// Proxyauth
pub enum ProxyAuth {
    /// Basic username/password auth
    Basic { username: String, password: String },
    /// Bearer token auth
    Bearer { token: String },
}
/// Certificate validation modes
#[derive(Debug, Clone, PartialEq, Eq)]
/// Certificatevalidation
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

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Unifiedadapterconfigcanonical
pub type UnifiedAdapterConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UnifiedAdapterConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Proxyconfigcanonical
pub type ProxyConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ProxyConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

