//! NestGate Network Module
//!
//! Provides network communication, service discovery, and orchestration integration
//! using universal provider patterns for maximum compatibility.

pub mod api;
pub mod connection_manager;
pub mod errors;
pub mod ports;
pub mod protocols;
pub mod service_discovery;
pub mod types;
pub mod universal_orchestration;

// Legacy modules for compatibility during migration
pub mod songbird;

// Re-exports for universal orchestration
pub use universal_orchestration::{
    create_default_orchestration_manager, create_orchestration_manager_with_config,
    NetworkServiceRegistration, OrchestrationStats, UniversalOrchestrationConfig,
    UniversalOrchestrationManager,
};

// Legacy compatibility re-exports
pub use universal_orchestration::{SongbirdConfig, SongbirdConnectionManager};

// Common types and utilities
pub use connection_manager::ConnectionManager;
pub use errors::{NetworkError, Result};
pub use service_discovery::{DiscoveredService, ServiceDiscovery};
pub use types::{NetworkStats, ServiceInstance, ServiceStatus};

// Re-export universal provider traits
pub use nestgate_core::universal_traits::OrchestrationPrimalProvider;

/// Default network configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfig {
    /// Enable universal orchestration discovery
    pub enable_orchestration: bool,
    /// Orchestration discovery timeout in seconds
    pub orchestration_timeout: u64,
    /// Service discovery interval in seconds
    pub discovery_interval: u64,
    /// Health check interval in seconds
    pub health_check_interval: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enable_orchestration: true,
            orchestration_timeout: 10,
            discovery_interval: 60,
            health_check_interval: 30,
        }
    }
}
