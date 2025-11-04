//! **NESTGATE ZFS CRATE**
//!
//! This crate provides ZFS storage management functionality for `NestGate`,
//! with canonical configuration integration and zero-cost abstractions.

// Core modules
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
pub mod canonical_zfs_config;
pub mod health;
#[cfg(test)]
mod health_tests;
pub mod metrics;
pub mod production_readiness;

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

// Configuration
pub mod config;

// Development and testing
pub mod byob;
pub mod dev_environment;

// Performance engine
pub mod performance_engine;

// Native implementations
pub mod native;

// Constants
pub mod constants;

// Additional test modules
#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod pool_types_tests;

// Re-exports for common usage
pub use command::ZfsCommand;
pub use dataset::ZfsDatasetManager;
pub use error::ZfsError;
pub use pool::ZfsPoolManager;
pub use types::*;
pub use zero_cost_zfs_operations::ProductionZfsManager;
// pub use canonical_zfs_config::{ZfsConfig, ZfsExtensions}; // Module not yet implemented
// #[allow(deprecated)]
// pub use canonical_zfs_config::CanonicalZfsConfig; // Module not yet implemented
pub use canonical_zfs_config::ZfsConfig;
pub use pool_setup::ZfsPoolSetup;
pub use production_readiness::ProductionReadinessValidator;
