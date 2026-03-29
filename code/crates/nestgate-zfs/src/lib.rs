// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **NESTGATE ZFS CRATE**
//!
//! This crate provides ZFS storage management functionality for `NestGate`,
//! with canonical configuration integration and zero-cost abstractions.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

// Core modules
pub mod adaptive_backend; // ✅ Adaptive ZFS: system or internal

/// Storage backends (S3, Azure, GCS, etc.)
pub mod backends;

/// Command execution module for ZFS commands
pub mod command;

pub mod dataset;

/// Error types and error handling for ZFS operations
pub mod error;

pub mod pool;
pub mod pool_helpers;

/// Saturating `f64`/`u64` helpers for size parsing and metrics.
pub mod numeric;

/// Core type definitions for ZFS operations
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

/// Metrics collection and reporting for ZFS operations
pub mod metrics;

/// Production readiness validation and operational checks
pub mod production_readiness;
#[cfg(test)]
mod production_readiness_tests;

// Real operations module (re-export for compatibility)
/// Real ZFS operations module - production implementations
///
/// This module contains actual ZFS command execution and native ZFS operations.
/// It provides the production-ready implementations of ZFS functionality including
/// pool management, dataset operations, and snapshot handling.
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
// NOTE: byob module removed (HTTP dependencies eliminated per Concentrated Gap Architecture)
// Workspace coordination now via Unix sockets through Songbird gateway
/// Development environment configuration and helpers
///
/// This module provides development-time helpers and configurations for working
/// with ZFS in development environments where actual ZFS may not be available.
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
mod round6_zfs_final_coverage;
#[cfg(test)]
mod zfs_final_coverage_boost;

// Re-exports for common usage
pub use command::ZfsCommand;
pub use dataset::ZfsDatasetManager;
pub use error::ZfsError;
pub use pool::ZfsPoolManager;
// Re-export types module items individually
pub use types::{DatasetInfo, PoolInfo, PoolStatus, SnapshotInfo};

pub use zero_cost_zfs_operations::ProductionZfsManager;
// Re-export zero-cost types for test modules
pub use zero_cost_zfs_operations::{ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo};

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
