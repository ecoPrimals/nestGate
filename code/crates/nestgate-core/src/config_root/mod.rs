//! Config Root module

use crate::error::{NetworkError};
// Configuration module for Universal Primal Architecture
///
// Re-exports configuration types and provides implementation modules.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::Result;
use crate::error::{ NestGateError as ConfigurationError};

// Re-export config provider
pub use crate::traits::config::ConfigProvider;

pub mod validation;
pub mod providers;

// Main orchestrator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Orchestrator
pub struct OrchestratorConfig<T = DefaultServiceConfig>
where
    T: Clone + Send + Sync + serde::de::DeserializeOwned
{
    /// Core orchestrator configuration
    pub orchestrator: CoreOrchestratorConfig,
    /// Network configuration
    pub network: NetworkConfig,

    /// Security configuration
    pub security: SecurityConfig,

    /// Monitoring configuration
    pub monitoring: MonitoringConfig,

    /// Service discovery configuration
    pub discovery: DiscoveryConfig,

    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,

    /// Health monitoring configuration
    pub health: HealthConfig,

    /// Service-specific configuration
    pub service: T,
}

impl<T> Default for OrchestratorConfig<T>
where
    T: Default
{
    /// Returns the default instance
    fn default() -> Self {
        Self {
            orchestrator: CoreOrchestratorConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            discovery: DiscoveryConfig::default(),
            load_balancing: LoadBalancingConfig::default(),
            health: HealthConfig::default(),
            service: T::default(),
        }
    }
}

// Core orchestrator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::CoreOrchestratorConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::CoreOrchestratorConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for CoreOrchestrator
pub struct CoreOrchestratorConfig {
    /// Port
    pub port: u16,
    /// Max Services
    pub max_services: usize,
    /// Health Check Interval
    pub health_check_interval: Duration,
    /// Service Start Timeout
    pub service_start_timeout: Duration,
    /// Service Stop Timeout
    pub service_stop_timeout: Duration,
    /// Request Timeout
    pub request_timeout: Duration,
}
impl Default for CoreOrchestratorConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            port: 8080,
            max_services: 100,
            health_check_interval: Duration::from_secs(30),
            service_start_timeout: Duration::from_secs(30),
            service_stop_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(30),
        }
    }
}

// Network configuration
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_primary::domains::network`
#[deprecated(since = "0.9.0", note = "Use canonical_primary::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Network
pub struct NetworkConfig {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Max Connections
    pub max_connections: usize,
    /// Connection Timeout
    pub connection_timeout: Duration,
    /// Keep Alive
    pub keep_alive: bool,
    /// Compression
    pub compression: bool,
}
impl Default for NetworkConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            keep_alive: true,
            compression: true,
        }
    }
}

// Security configuration
/// **⚠️ DEPRECATED**: Use `CanonicalSecurityConfig` from `canonical_primary::domains::security_canonical`
#[deprecated(since = "0.11.2", note = "Use canonical_primary::domains::security_canonical::CanonicalSecurityConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::SecurityConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::SecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Security
pub struct SecurityConfig {
    /// Enable Tls
    pub enable_tls: bool,
    /// Require Auth
    pub require_auth: bool,
    /// Auth Method
    pub auth_method: String,
    /// Allowed Origins
    pub allowed_origins: Vec<String>,
}
impl Default for SecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_tls: false,
            require_auth: false,
            auth_method: "none".to_string(),
            allowed_origins: vec!["*".to_string()],
        }
    }
}

// Monitoring configuration
/// **⚠️ DEPRECATED**: Use `MonitoringConfig` from `canonical_primary::supporting_types`
#[deprecated(since = "0.11.2", note = "Use canonical_primary::supporting_types::MonitoringConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Monitoring
pub struct MonitoringConfig {
    /// Enable Metrics
    pub enable_metrics: bool,
    /// Metrics Port
    pub metrics_port: u16,
    /// Enable Tracing
    pub enable_tracing: bool,
    /// Log Level
    pub log_level: String,
    /// Log Format
    pub log_format: String,
}
impl Default for MonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_port: 9090,
            enable_tracing: true,
            log_level: "info".to_string(),
            log_format: "json".to_string(),
        }
    }
}

// Service discovery configuration
/// **⚠️ DEPRECATED**: Use Infant Discovery / capability-based discovery instead
#[deprecated(since = "0.11.2", note = "Use Infant Discovery system for dynamic service discovery instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Discovery
pub struct DiscoveryConfig {
    /// Provider
    pub provider: String,
// DEPRECATED: Consul service discovery - migrate to capability-based discovery
// Capability-based discovery implemented
    pub consul_url: Option<String>,
// DEPRECATED: Kubernetes orchestration - migrate to capability-based orchestration
// Capability-based discovery implemented
    pub kubernetes_namespace: Option<String>,
    /// Static Services
    pub static_services: Vec<StaticServiceConfig>,
}
impl Default for DiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            provider: "static".to_string(),
// DEPRECATED: Consul service discovery - migrate to capability-based discovery
// Capability-based discovery implemented
            consul_url: None,
// DEPRECATED: Kubernetes orchestration - migrate to capability-based orchestration
// Capability-based discovery implemented
            kubernetes_namespace: None,
            static_services: Vec::new(),
        }
    }
}

// Load balancing configuration
/// **⚠️ DEPRECATED**: Load balancing should be handled by networking layer (capability-based)
#[deprecated(since = "0.11.2", note = "NestGate is a storage system. Use orchestration/networking capability for load balancing")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LoadBalancing
pub struct LoadBalancingConfig {
    /// Algorithm
    pub algorithm: String,
    /// Health Check Enabled
    pub health_check_enabled: bool,
    /// Failure Threshold
    pub failure_threshold: u32,
    /// Recovery Threshold
    pub recovery_threshold: u32,
}
impl Default for LoadBalancingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            algorithm: "round_robin".to_string(),
            health_check_enabled: true,
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

// Health monitoring configuration
/// **⚠️ DEPRECATED**: Use canonical health monitoring from `canonical_primary::supporting_types`
#[deprecated(since = "0.11.2", note = "Use canonical health monitoring configuration instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Health
pub struct HealthConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Check Interval
    pub check_interval: Duration,
    /// Timeout
    pub timeout: Duration,
    /// Failure Threshold
    pub failure_threshold: u32,
    /// Success Threshold
    pub success_threshold: u32,
}
impl Default for HealthConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            failure_threshold: 3,
            success_threshold: 2,
        }
    }
}

// Static service configuration for discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::StaticServiceConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::StaticServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for StaticService
pub struct StaticServiceConfig {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Tags
    pub tags: Vec<String>,
}
// Default service configuration placeholder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DefaultServiceConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DefaultServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for DefaultService
pub struct DefaultServiceConfig {
    /// Placeholder
    pub placeholder: bool,
}
// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Staticserviceconfigcanonical
pub type StaticServiceConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StaticServiceConfig (the deprecated struct) for now.
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
/// Type alias for Defaultserviceconfigcanonical
pub type DefaultServiceConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DefaultServiceConfig (the deprecated struct) for now.
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
/// Type alias for Securityconfigcanonical
pub type SecurityConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityConfig (the deprecated struct) for now.
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
/// Type alias for Coreorchestratorconfigcanonical
pub type CoreOrchestratorConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CoreOrchestratorConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

