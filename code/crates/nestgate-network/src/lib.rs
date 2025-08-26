//
// Provides network services and connection management

pub mod connection_manager;
pub mod real_network_service;

// **CANONICAL MODERNIZATION**: Use canonical trait system
pub use nestgate_core::traits::{UniversalService, ServiceRegistration};
pub use connection_manager::{OrchestrationClient, ConnectionManager};

// Re-export key types for external use
// Note: orchestration_adapter module needs to be implemented
// Removed unresolved universal_orchestration import

// Re-exports for universal orchestration
// Removed unresolved universal_orchestration imports - these need to be implemented or removed

// Legacy compatibility re-exports (migrated to universal patterns)
// Removed unresolved universal_orchestration imports

// Common types and utilities
// Removed unresolved error imports - use nestgate_core::error instead
// Re-export core error types

// Re-export core types - using correct paths
pub use nestgate_core::types::ServiceInstance;

// Re-export universal provider traits
// Note: OrchestrationPrimalProvider path needs verification

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
