// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **OPTIMIZED API HANDLERS MODULE**
//!
//! Comprehensive collection of HTTP request handlers for the NestGate REST API.
//!
//! # Organization
//!
//! This module provides organized, explicit imports instead of wildcard imports
//! for better maintainability and clearer module boundaries.
//!
//! Handlers are organized by domain:
//! - **AI First**: AI-powered examples and demonstrations
//! - **Compliance**: Regulatory compliance and auditing
//! - **Dashboard**: Dashboard data and visualizations
//! - **Hardware Tuning**: Performance tuning and optimization
//! - **Health**: System health checks and status
//! - **Load Testing**: Performance testing infrastructure
//! - **Metrics**: System metrics collection and reporting
//! - **Performance Analytics**: Performance analysis and recommendations
//! - **RPC**: JSON-RPC and protocol discovery for inter-primal communication
//! - **Storage**: ZFS pool, dataset, and snapshot operations
//! - **Workspace Management**: Multi-tenant workspace isolation
//! - **ZFS**: Low-level ZFS operations
//!
//! # Handler Types
//!
//! All handlers follow the Axum handler signature:
//! ```rust,ignore
//! async fn handler(
//!     State(state): State<Arc<AppState>>,
//!     Json(payload): Json<RequestType>
//! ) -> Result<Json<ResponseType>, StatusCode>
//! ```
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use nestgate_api::handlers::storage::get_storage_pools;
//! use axum::{Router, routing::get};
//!
//! let app = Router::new()
//!     .route("/api/v1/storage/pools", get(get_storage_pools));
//! ```
//!
//! # Feature Flags
//!
//! - `dev-stubs`: Use stub implementations for testing
//! - Production builds use real ZFS implementations
//!
//! # Architecture Decisions
//!
//! - **Explicit Imports**: No wildcard imports for clarity
//! - **Domain Grouping**: Related handlers grouped in submodules
//! - **Async First**: All handlers are async for non-blocking I/O
//! - **Type Safety**: Strong typing with validated request/response types
//! - **Error Handling**: Consistent error responses with proper HTTP status codes

// ==================== CORE HANDLER MODULES ====================

/// **AUTH MODULE**
///
/// Authentication handlers delegate to `nestgate-security` (`HybridAuthenticationManager`).
pub mod auth;

/// **PRODUCTION AUTH HANDLERS**
///
/// Production authentication using `nestgate-core` security (`AuthManager`).
pub mod auth_production;
#[cfg(test)]
mod auth_production_tests;

/// **AI-FIRST EXAMPLE MODULE**
///
/// Demo/example handlers — gated behind `dev-stubs` to keep production binary lean.
#[cfg(any(test, feature = "dev-stubs"))]
pub mod ai_first_example;
#[cfg(test)]
mod ai_first_example_tests;
#[cfg(test)]
mod ai_first_handler_tests;

/// **COMPLIANCE MODULE**
///
/// Compliance and regulatory management for storage systems.
pub mod compliance;

/// **DASHBOARD TYPES MODULE**
///
/// Type definitions and structures for dashboard functionality.
pub mod dashboard_types;
#[cfg(test)]
mod dashboard_types_tests;

/// **HARDWARE TUNING MODULE**
///
/// Hardware performance tuning and optimization.
pub mod hardware_tuning;

/// **HEALTH MODULE**
///
/// System health monitoring and status reporting.
pub mod health;

/// **LOAD TESTING MODULE**
///
/// Load testing framework and scenarios.
pub mod load_testing;

/// **METRICS COLLECTOR MODULE**
///
/// Real-time metrics collection and aggregation system.
pub mod metrics_collector;
#[cfg(test)]
mod metrics_collector_critical_tests;
#[cfg(test)]
mod metrics_collector_expanded_tests;
#[cfg(test)]
mod metrics_collector_unit_tests;

/// **PERFORMANCE ANALYTICS MODULE**
///
/// Advanced performance analysis and optimization recommendations.
pub mod performance_analytics;
#[cfg(test)]
mod performance_analytics_comprehensive_tests;
#[cfg(test)]
mod performance_analytics_expanded_tests;

/// **PERFORMANCE ANALYZER MODULE**
///
/// Core performance analysis engine.
pub mod performance_analyzer;

/// **PERFORMANCE DASHBOARD MODULE**
///
/// Comprehensive performance dashboard with real-time monitoring.
pub mod performance_dashboard;

/// **RPC HANDLERS MODULE**
///
/// JSON-RPC and protocol discovery for inter-primal communication.
pub mod rpc_handlers;

/// **STATUS MODULE**
///
/// System status reporting and uptime tracking.
pub mod status;
#[cfg(test)]
mod status_additional_comprehensive_tests;

/// **STORAGE MODULE**
///
/// Core storage management and operations.
pub mod storage;
#[cfg(test)]
mod storage_comprehensive_tests;
#[cfg(test)]
mod storage_critical_tests;
#[cfg(test)]
mod storage_unit_tests;

/// **WORKSPACE MANAGEMENT MODULE**
///
/// Workspace creation, management, and collaboration features.
pub mod workspace_management;

#[cfg(test)]
mod api_error_path_tests; // Dec 10, 2025 - Comprehensive API error path tests

#[cfg(test)]
mod mod_tests;

/// **ZERO-COST API HANDLERS MODULE**
///
/// High-performance zero-cost abstraction API handlers.
pub mod zero_cost_api_handlers;
#[cfg(test)]
mod zero_cost_api_handlers_tests;

/// **ZFS HANDLERS MODULE**
///
/// ZFS-specific storage handlers (production implementations).
pub mod zfs;

/// Handler structs, manager wrappers, and [`ApiRouter`].
pub mod handler_types;

/// [`HandlerCollection`], [`HandlerRegistry`], and handler discovery helpers.
pub mod registry;

// ==================== EXPLICIT RE-EXPORTS ====================

pub use handler_types::*;
pub use registry::{
    HandlerCollection, HandlerRegistry, available_handlers, create_handler_by_name,
    initialize_handlers,
};

#[cfg(test)]
mod auth_tests;
#[cfg(test)]
mod compliance_types_tests;
#[cfg(test)]
mod health_tests;
#[cfg(test)]
mod metrics_collector_enhanced_tests;
#[cfg(test)]
mod performance_analytics_tests;
#[cfg(test)]
mod status_comprehensive_tests;
#[cfg(test)]
mod storage_tests;
#[cfg(test)]
mod zero_cost_api_handlers_additional_tests;
#[cfg(test)]
mod zero_cost_tests;
