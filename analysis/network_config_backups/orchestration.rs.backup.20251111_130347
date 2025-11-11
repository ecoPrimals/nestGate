//
// Service discovery, orchestration, and service registration configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Network orchestration and service discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub retry_config: OrchestrationRetryConfig,
    /// Service registration settings
    pub service_registration: ServiceRegistrationSettings,
}
/// Orchestration retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            retry_config: OrchestrationRetryConfig::default(),
            service_registration: ServiceRegistrationSettings::default(),
        }
    }
}

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
