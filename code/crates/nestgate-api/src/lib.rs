//! **NESTGATE API LIBRARY**
//!
//! Comprehensive API library for `NestGate` storage and system management platform.
//! Provides REST endpoints, handlers, models, and routing for all `NestGate` functionality.

// Enable documentation warnings for better API documentation
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
// Temporary allow deprecated during canonical config migration
#![allow(deprecated)]
// Pedantic lints - will address incrementally during modernization phase (Phase 6)
// These don't affect functionality, only code style/documentation
// Total: ~241 pedantic warnings to be fixed during Week 11-12 modernization
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::unused_async)] // Many async functions are trait implementations
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)] // Will refactor during modernization
#![allow(clippy::cognitive_complexity)] // Will simplify during refactoring
#![allow(clippy::unused_self)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::significant_drop_tightening)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::if_not_else)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::impl_trait_in_params)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::pub_underscore_fields)]
#![allow(clippy::double_must_use)]
#![allow(clippy::manual_strip)]
#![allow(clippy::branches_sharing_code)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::to_string_trait_impl)]
#![allow(clippy::collection_is_never_read)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::significant_drop_in_scrutinee)]
#![allow(clippy::too_long_first_doc_paragraph)]

/// **TYPES MODULE**
///
/// Canonical API configuration types and backward-compatible aliases.
pub mod types;

/// **ERROR MODULE**
///
/// Error types and handling for the `NestGate` Data API.
pub mod error;

/// **HANDLERS MODULE**
///
/// Request handlers for all API endpoints including storage, performance monitoring,
/// hardware tuning, workspace management, and zero-cost operations.
pub mod handlers;

/// **MODELS MODULE**
///
/// Data models and structures used throughout the API for requests, responses,
/// and internal data representation.
pub mod models;

/// **REST MODULE**
///
/// REST API implementation including routing, middleware, and HTTP handlers.
pub mod rest;

/// **ROUTES MODULE**
///
/// URL routing configuration and endpoint definitions for the API server.
pub mod routes;

/// **NESTGATE RPC SERVICE MODULE**
///
/// tarpc and JSON-RPC interfaces for inter-primal communication.
/// Enables high-performance binary RPC (tarpc) and universal HTTP-based RPC (JSON-RPC).
pub mod nestgate_rpc_service;

/// **DEV STUBS MODULE** (Feature-gated: `dev-stubs`)
///
/// Development stub implementations for testing and local development.
/// ⚠️ **NOT FOR PRODUCTION** - Only available with `dev-stubs` feature flag.
///
/// **Consolidated**: November 10, 2025
/// - Replaces: `handlers/zfs_stub.rs`
/// - Replaces: `handlers/hardware_tuning/stub_helpers.rs`
#[cfg(feature = "dev-stubs")]
pub mod dev_stubs;

// Re-export commonly used types
// Specific handler re-exports to avoid namespace pollution
pub use handlers::{
    compliance, hardware_tuning, load_testing, performance_analyzer, performance_dashboard,
    storage, zfs,
};
pub use routes::{create_router, AppState};

// Test modules
#[cfg(test)]
mod api_coverage_boost;
