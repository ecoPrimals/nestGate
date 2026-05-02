// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! **NESTGATE ZFS CRATE**
//!
//! This crate provides ZFS storage management functionality for `NestGate`,
//! with canonical configuration integration and zero-cost abstractions.

#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::redundant_clone,
        clippy::needless_collect,
    )
)]
#![expect(
    dead_code,
    clippy::missing_errors_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::implicit_hasher,
    clippy::unreadable_literal,
    clippy::used_underscore_items
)]
#![warn(rustdoc::broken_intra_doc_links)]

// Core modules
pub mod adaptive_backend; // System ZFS vs internal implementation

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
pub mod health;
#[cfg(test)]
mod health_tests;
#[cfg(test)]
mod path_cli_test_lock;

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
#[cfg(feature = "orchestrator")]
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
// Workspace coordination now via Unix sockets through the orchestration gateway
/// Development environment configuration and helpers
///
/// This module provides development-time helpers and configurations for working
/// with ZFS in development environments where actual ZFS may not be available.
#[cfg(feature = "dev-stubs")]
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
mod additional_coverage_tests;
#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod comprehensive_coverage_tests;
#[cfg(test)]
mod extended_coverage_tests;
#[cfg(test)]
mod pool_types_tests;

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
