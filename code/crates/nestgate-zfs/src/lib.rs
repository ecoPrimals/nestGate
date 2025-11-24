//! **NESTGATE ZFS CRATE**
//!
//! This crate provides ZFS storage management functionality for `NestGate`,
//! with canonical configuration integration and zero-cost abstractions.

// Temporary allow deprecated during canonical config migration
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(deprecated)]

// Core modules
pub mod adaptive_backend; // ✅ Adaptive ZFS: system or internal
pub mod command;
pub mod dataset;
pub mod error;
pub mod pool;
pub mod pool_helpers;
pub mod types;

// Performance and optimization
pub mod performance;
pub mod zero_cost_zfs_handler;
pub mod zero_cost_zfs_operations;

// Configuration and management
// ⚠️ REMOVED: canonical_zfs_config was deprecated and removed in v0.11.0 (November 2025)
// Use types::ZfsStorageConfig or types::CanonicalZfsConfig instead
// pub mod canonical_zfs_config; // REMOVED - use types::ZfsStorageConfig
pub mod health;
#[cfg(test)]
mod health_tests;
pub mod metrics;
pub mod production_readiness;
#[cfg(test)]
mod production_readiness_tests;

// Real operations module (re-export for compatibility)
pub mod real_zfs_operations {
    pub use crate::production_readiness::RealZfsOperations;
}

// Advanced features
pub mod advanced_features;
pub mod automation;
pub mod snapshot;

// Integration modules
pub mod mcp_integration;
pub mod orchestrator_integration;
pub mod pool_setup;

// Management and handlers
pub mod failover;
pub mod handlers;
pub mod manager;
pub mod tier;
#[cfg(test)]
mod tier_tests;

// Configuration
pub mod config;

// Development and testing
pub mod byob;
pub mod dev_environment;

// Performance engine
pub mod performance_engine;

// Native implementations
pub mod native;
#[cfg(test)]
mod native_command_executor_tests;

// Constants
pub mod constants;

// Additional test modules
#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod comprehensive_coverage_boost;
#[cfg(test)]
mod pool_types_tests;
#[cfg(test)]
mod zfs_final_coverage_boost;

// Re-exports for common usage
pub use command::ZfsCommand;
pub use dataset::ZfsDatasetManager;
pub use error::ZfsError;
pub use pool::ZfsPoolManager;
pub use types::*;
pub use zero_cost_zfs_operations::ProductionZfsManager;
// **CANONICAL ZFS CONFIGURATION EXPORTS**
// Re-export canonical types for easy access
pub use types::{
    CanonicalZfsConfig, ZfsDatasetConfig, ZfsMigrationConfig, ZfsMonitoringConfig,
    ZfsPerformanceConfig, ZfsPoolConfig, ZfsSnapshotConfig, ZfsStorageConfig,
};

// Backward compatibility: export as ZfsConfig
pub use types::CanonicalZfsConfig as ZfsConfig;

pub use pool_setup::ZfsPoolSetup;
pub use production_readiness::ProductionReadinessValidator;
