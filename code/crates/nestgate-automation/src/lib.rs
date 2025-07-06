//! NestGate Automation Library
//!
//! Intelligent automation and ecosystem integration for ZFS storage management.
//! This library provides:
//! - Dynamic ecosystem service discovery
//! - Heuristic tier prediction and optimization (AI compute handled by Toadstool)
//! - Automated dataset lifecycle management
//! - Performance monitoring and optimization
//! - File analysis and access pattern tracking

pub mod analysis;
pub mod connections;
pub mod discovery;
pub mod lifecycle;
pub mod manager;
pub mod prediction;
pub mod types;

// Re-export main types and interfaces
pub use analysis::*;
pub use connections::{ServiceConnectionPool, SquirrelConnection};
pub use discovery::EcosystemDiscovery;
pub use lifecycle::DatasetLifecycleManager;
pub use manager::IntelligentDatasetManager;
pub use prediction::TierPredictor;
pub use types::*;

// Result type alias for convenience
pub type Result<T> = std::result::Result<T, crate::types::AutomationError>;

/// Initialize automation system with default configuration
pub async fn initialize_automation(
    zfs_config: nestgate_core::config::Config,
) -> Result<IntelligentDatasetManager> {
    let config = AutomationConfig::default();
    IntelligentDatasetManager::new(zfs_config, config).await
}

/// Initialize automation system with custom configuration
pub async fn initialize_automation_with_config(
    zfs_config: nestgate_core::config::Config,
    automation_config: AutomationConfig,
) -> Result<IntelligentDatasetManager> {
    IntelligentDatasetManager::new(zfs_config, automation_config).await
}

/// Check if ecosystem services are available
#[cfg(feature = "network-integration")]
pub async fn check_ecosystem_availability() -> bool {
    match discovery::EcosystemDiscovery::new(&AutomationConfig::default()) {
        Ok(discovery) => discovery.discover_songbirds().await.is_ok(),
        Err(_) => false,
    }
}

#[cfg(not(feature = "network-integration"))]
pub async fn check_ecosystem_availability() -> bool {
    false
}
