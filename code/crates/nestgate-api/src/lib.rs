// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **NESTGATE API LIBRARY**
//!
//! Comprehensive API library for `NestGate` storage and system management platform.
//! Provides REST endpoints, handlers, models, and routing for all `NestGate` functionality.

// Enable documentation warnings for better API documentation
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

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
