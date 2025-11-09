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

