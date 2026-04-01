// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! **NESTGATE ZFS CRATE**
//!
//! This crate provides ZFS storage management functionality for `NestGate`,
//! with canonical configuration integration and zero-cost abstractions.

#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic,))]
#![allow(
    deprecated,
    dead_code,
    unused_doc_comments,
    unused_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    missing_docs
)]
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
