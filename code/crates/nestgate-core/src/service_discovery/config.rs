/// Service Discovery Configuration
/// Configuration types and structures for service discovery functionality.
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **ECOSYSTEM UNIFICATION**: Import unified types from canonical locations
use crate::config::canonical_primary::NestGateCanonicalConfig as UnifiedConfig;
use crate::config::canonical_primary::{
    NetworkConfig as UnifiedNetworkConfig, 
    MonitoringConfig as UnifiedMonitoringConfig,
};
// **MIGRATED**: Use canonical configs instead of local duplicates
use crate::config::canonical_primary::domains::security_canonical::UnifiedSecurityConfig;
use crate::config::canonical_primary::service::UnifiedServiceConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::ServiceDiscoveryConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ServiceDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct ServiceDiscoveryConfig {
// DEPRECATED: etcd key-value store - migrate to capability-based storage
// Capability-based discovery implemented
    /// Discovery method (HTTP, DNS, Consul, etcd)
    pub discovery_method: DiscoveryMethod,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Service timeout
    pub service_timeout: Duration,
    /// Retry attempts for failed services
    pub max_retry_attempts: u32,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Auto-registration settings
    pub auto_registration: Option<AutoRegistrationConfig>,
    }


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ServiceDiscoveryConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ServiceDiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

