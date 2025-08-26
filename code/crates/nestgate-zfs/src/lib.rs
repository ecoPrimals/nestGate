//
// This crate provides ZFS storage management functionality for NestGate,
// with canonical configuration integration and zero-cost abstractions.
// **CANONICAL MODERNIZATION COMPLETE** - All deprecated patterns removed.

// ==================== CANONICAL CONFIGURATION ====================
pub mod canonical_zfs_config;
pub use canonical_zfs_config::{
    CanonicalZfsConfig, CanonicalZfsConfigBuilder, ZfsDatasetConfig, ZfsMigrationConfig,
    ZfsMonitoringConfig, ZfsPerformanceConfig, ZfsPoolConfig, ZfsSnapshotConfig,
};

// ==================== CORE ZFS FUNCTIONALITY ====================
pub mod constants;
pub mod error;
pub mod native;
pub mod types;

// ==================== ZFS COMPONENT MODULES ====================
pub mod automation;
pub mod config;
pub mod dataset;
pub mod health;
pub mod metrics;
pub mod migration;
pub mod performance;
pub mod performance_engine;
pub mod pool;
pub mod pool_setup;
pub mod snapshot;
pub mod tier;

// ==================== OPERATIONS MODULES ====================
pub mod mock;
pub mod real_zfs_operations;

// ==================== CANONICAL MANAGER ====================
pub mod manager;
pub use manager::ZfsManager;

// ==================== ZERO-COST IMPLEMENTATIONS ====================
pub mod zero_cost_zfs_handler;
pub mod zero_cost_zfs_operations;

// ==================== HANDLERS & OPERATIONS ====================
pub mod command;
pub mod handlers;

// ==================== RE-EXPORTS FOR INTERNAL USE ====================
pub use types::{StorageTier, ZfsError};
pub use zero_cost_zfs_operations::{
    DevelopmentZfsManager, HighPerformanceZfsManager, ProductionZfsManager, ZeroCostZfsManager,
    ZeroCostZfsOperations,
};

// Re-export commonly used types for internal modules
pub use dataset::ZfsDatasetManager;
pub use native::is_zfs_available;
pub use pool::ZfsPoolManager;

// ==================== CANONICAL INTEGRATION ====================

/// Create a canonical ZFS configuration for development
pub fn development_config() -> nestgate_core::error::Result<CanonicalZfsConfig> {
    CanonicalZfsConfigBuilder::development()
}

/// Create a canonical ZFS configuration for production
pub fn production_config() -> nestgate_core::error::Result<CanonicalZfsConfig> {
    CanonicalZfsConfigBuilder::production()
}

/// Create a canonical ZFS configuration for high performance
pub fn high_performance_config() -> nestgate_core::error::Result<CanonicalZfsConfig> {
    CanonicalZfsConfigBuilder::high_performance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_config_creation() {
        // Test that all canonical configurations can be created
        assert!(development_config().is_ok());
        assert!(production_config().is_ok());
        assert!(high_performance_config().is_ok());
    }

    #[test]
    fn test_zero_cost_managers() {
        // Test zero-cost manager creation
        let _dev_manager = DevelopmentZfsManager::new();
        let _prod_manager = ProductionZfsManager::new();
        let _hp_manager = HighPerformanceZfsManager::new();
    }
}
