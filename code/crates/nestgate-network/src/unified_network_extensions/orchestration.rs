//
// Service discovery, orchestration, and service registration configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Network orchestration and service discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(deprecated)] // Using OrchestrationRetryConfig during transition period
pub struct NetworkOrchestrationSettings {
    /// Enable universal orchestration discovery
    pub enable_orchestration: bool,
    /// Orchestration discovery timeout
    pub orchestration_timeout: Duration,
    /// Service discovery interval
    pub discovery_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Maximum concurrent orchestration connections
    pub max_orchestration_connections: u32,
    /// Orchestration retry configuration
    /// Intentional: Transition period until v0.12.0 (May 2026)
    /// Migration in progress to CanonicalNetworkConfig
    pub retry_config: OrchestrationRetryConfig,
    /// Service registration settings
    pub service_registration: ServiceRegistrationSettings,
}
/// Orchestration retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::OrchestrationRetryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::OrchestrationRetryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct OrchestrationRetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Retry multiplier
    pub multiplier: f64,
    /// Enable exponential backoff
    pub exponential_backoff: bool,
}
/// Service registration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationSettings {
    /// Auto-register services
    pub auto_register: bool,
    /// Service TTL
    pub service_ttl: Duration,
    /// Registration retry attempts
    pub registration_retries: u32,
    /// Service metadata
    pub service_metadata: HashMap<String, String>,
}
impl Default for NetworkOrchestrationSettings {
    fn default() -> Self {
        Self {
            enable_orchestration: true,
            orchestration_timeout: Duration::from_secs(30),
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            max_orchestration_connections: 100,
            #[allow(deprecated)] // Using deprecated during migration period
            retry_config: OrchestrationRetryConfig::default(),
            service_registration: ServiceRegistrationSettings::default(),
        }
    }
}

#[allow(deprecated)] // Deprecated struct with migration path documented
impl Default for OrchestrationRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
            exponential_backoff: true,
        }
    }
}

impl Default for ServiceRegistrationSettings {
    fn default() -> Self {
        Self {
            auto_register: true,
            service_ttl: Duration::from_secs(300),
            registration_retries: 3,
            service_metadata: HashMap::new(),
        }
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
pub type OrchestrationRetryConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using OrchestrationRetryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
