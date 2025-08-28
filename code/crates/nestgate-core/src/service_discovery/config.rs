/// Service Discovery Configuration
/// Configuration types and structures for service discovery functionality.

use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **ECOSYSTEM UNIFICATION**: Import unified types from canonical locations
use crate::config::canonical_master::NestGateCanonicalConfig as UnifiedConfig;
use crate::config::canonical_master::{
    NetworkConfig as UnifiedNetworkConfig, 
    MonitoringConfig as UnifiedMonitoringConfig,
};

// **FALLBACK**: Define missing config types locally until they are added to unified_types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedSecurityConfig {
    pub enable_tls: bool,
    pub verify_certificates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedServiceConfig {
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
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

