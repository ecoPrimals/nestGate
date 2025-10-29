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
pub struct CoreOrchestratorConfig {
    pub port: u16,
    pub max_services: usize,
    pub health_check_interval: Duration,
    pub service_start_timeout: Duration,
    pub service_stop_timeout: Duration,
    pub request_timeout: Duration,
}
impl Default for CoreOrchestratorConfig {
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
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_master::domains::network`
#[deprecated(since = "0.9.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub keep_alive: bool,
    pub compression: bool,
}
impl Default for NetworkConfig {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_tls: bool,
    pub require_auth: bool,
    pub auth_method: String,
    pub allowed_origins: Vec<String>,
}
impl Default for SecurityConfig {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub metrics_port: u16,
    pub enable_tracing: bool,
    pub log_level: String,
    pub log_format: String,
}
impl Default for MonitoringConfig {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub provider: String,
// DEPRECATED: Consul service discovery - migrate to capability-based discovery
// Capability-based discovery implemented
    pub consul_url: Option<String>,
// DEPRECATED: Kubernetes orchestration - migrate to capability-based orchestration
// Capability-based discovery implemented
    pub kubernetes_namespace: Option<String>,
    pub static_services: Vec<StaticServiceConfig>,
}
impl Default for DiscoveryConfig {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: String,
    pub health_check_enabled: bool,
    pub failure_threshold: u32,
    pub recovery_threshold: u32,
}
impl Default for LoadBalancingConfig {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub timeout: Duration,
    pub failure_threshold: u32,
    pub success_threshold: u32,
}
impl Default for HealthConfig {
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
pub struct StaticServiceConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub tags: Vec<String>,
}
// Default service configuration placeholder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultServiceConfig {
    pub placeholder: bool,
}