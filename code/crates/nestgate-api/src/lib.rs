// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]
#![allow(
    deprecated,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::type_complexity,
    clippy::module_inception,
    clippy::too_long_first_doc_paragraph,
    clippy::used_underscore_binding,
    clippy::implicit_hasher,
    clippy::pub_underscore_fields,
    clippy::wildcard_in_or_patterns,
    clippy::manual_clamp,
    clippy::doc_markdown,
    clippy::uninlined_format_args,
    clippy::case_sensitive_file_extension_comparisons
)]
#![cfg_attr(
    test,
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unused_comparisons,
        unused_must_use,
        unused_doc_comments,
        unfulfilled_lint_expectations,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
        clippy::float_cmp,
        clippy::bool_assert_comparison,
        clippy::assertions_on_constants,
        clippy::no_effect_underscore_binding,
        clippy::field_reassign_with_default,
    )
)]

//! **NESTGATE API LIBRARY**
//!
//! Comprehensive API library for `NestGate` storage and system management platform.
//! Provides REST endpoints, handlers, models, and routing for all `NestGate` functionality.

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

/// **TRANSPORT MODULE** (TRUE PRIMAL)
///
/// Unix socket + JSON-RPC 2.0 transport for TRUE PRIMAL architecture.
/// Primary transport mechanism for production deployments.
///
/// **Features**:
/// - Unix sockets (100x faster than HTTP, port-free)
/// - JSON-RPC 2.0 protocol (universal, compatible)
/// - BearDog integration (hardware-backed security)
/// - Optional HTTP fallback (debugging only)
///
/// **Evolution**: January 2026 - TRUE PRIMAL transport implementation
pub mod transport;

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
pub use routes::{AppState, create_router};

// Test modules
#[cfg(test)]
mod api_coverage_boost;
#[cfg(all(test, feature = "dev-stubs"))]
mod round6_final_coverage;
