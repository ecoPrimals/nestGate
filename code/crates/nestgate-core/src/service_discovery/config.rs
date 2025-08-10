/// Service Discovery Configuration
/// Configuration types and structures for service discovery functionality.

use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **ECOSYSTEM UNIFICATION**: Import unified types
use crate::unified_types::{
    UnifiedConfig, UnifiedNetworkConfig, UnifiedSecurityConfig,
    UnifiedServiceConfig, UnifiedMonitoringConfig
};

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

