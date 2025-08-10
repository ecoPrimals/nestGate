//! NestGate Network Module
//!
//! Provides network communication, service discovery, and orchestration integration
//! using universal provider patterns for maximum compatibility.

pub mod api;
pub mod connection_manager;
pub mod orchestration_adapter; // ✅ CAPABILITY-BASED: No hardcoded primal names
pub mod universal_orchestration;

// Re-export key types for external use
pub use orchestration_adapter::{OrchestrationAdapter, OrchestrationConfig, OrchestrationService};
pub use universal_orchestration::UniversalOrchestrationConfig;

// Re-exports for universal orchestration
pub use universal_orchestration::{
    create_default_orchestration_manager, create_orchestration_manager_with_config,
    NetworkServiceRegistration, OrchestrationStats, UniversalOrchestrationManager,
};

// Legacy compatibility re-exports (migrated to universal patterns)
pub use universal_orchestration::{SongbirdConfig, SongbirdConnectionManager};

// Common types and utilities
pub use connection_manager::ConnectionManager;
pub use nestgate_core::error::{NetworkErrorData, Result};
pub use nestgate_core::service_discovery::{DiscoveredService, ServiceDiscovery};
pub use nestgate_core::types::{NetworkStats, ServiceInstance, ServiceStatus};

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
