// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

use crate::handlers::hardware_tuning::HardwareTuningConfig;

// ZfsHandlerImpl: dev-stubs uses real implementation, production uses placeholder
#[cfg(feature = "dev-stubs")]
use crate::handlers::zfs::basic::ZfsHandlerImpl;
#[cfg(not(feature = "dev-stubs"))]
use crate::handlers::zfs::production_placeholders::ZfsHandlerImpl;
use axum::Router;

// ==================== CORE HANDLER MODULES ====================

/// **AUTH MODULE**
///
/// Authentication handlers - returns 501 when auth not implemented.
/// Use nestgate-core security module for production auth.
pub mod auth;

/// **AI-FIRST EXAMPLE MODULE**
///
/// AI-powered example handlers and demonstrations.
pub mod ai_first_example;
#[cfg(test)]
mod ai_first_example_tests;

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
// Stub tests reference undefined types (`ApplicationMetrics`, etc.); restore when aligned with collector API.
// #[cfg(test)]
// mod metrics_collector_comprehensive_tests;
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

/// **ZFS STUB MODULE** (Development Only)
///
/// ⚠️ **ONLY AVAILABLE WITH `dev-stubs` FEATURE** ⚠️
///
/// Development stub for ZFS operations when ZFS is not installed.
/// **Never enabled in production builds.**
#[cfg(feature = "dev-stubs")]
/// Deprecated: Use `crate::dev_stubs::zfs` instead
/// This re-export will be removed in v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.2",
    note = "Use crate::dev_stubs::zfs instead - stubs organized into dev_stubs module. Migration: Replace use crate::handlers::zfs_stub with use crate::dev_stubs::zfs. Target removal: v0.12.0 (May 2026)."
)]
pub use crate::dev_stubs::zfs as zfs_stub;
// ==================== EXPLICIT RE-EXPORTS ====================

// Core handler types available through their respective modules
// Use direct module imports for clarity (e.g., use crate::handlers::storage::StorageHandler)

// ==================== HANDLER COLLECTIONS ====================

/// Collection of all core handlers for easy registration
pub struct HandlerCollection {
    /// AI-First handler for intelligent operations and recommendations
    pub ai_first: AIFirstHandler,
    /// Compliance handler for audit, governance, and regulatory features
    pub compliance: ComplianceHandler,
    /// Hardware tuning handler for system optimization and resource management
    pub hardware_tuning: HardwareTuningHandler,
    /// Health handler for system health checks and monitoring
    pub health: HealthHandler,
    /// Load testing handler for performance benchmarking and stress testing
    pub load_testing: LoadTestHandler,
    /// Metrics collector for performance data aggregation and analysis
    pub metrics_collector: MetricsCollector,
    /// Performance analyzer for trend analysis and optimization recommendations
    pub performance_analyzer: PerformanceAnalyzer,
    /// Storage handler for storage configuration and management operations
    pub storage: StorageHandler,
    /// Workspace manager for workspace lifecycle and collaboration features
    pub workspace_manager: WorkspaceManager,
    /// ZFS handler for ZFS storage operations and pool management
    pub zfs: ZfsHandler,
}
impl HandlerCollection {
    /// Create a new collection with default handlers
    #[must_use]
    pub fn new() -> Self {
        Self {
            ai_first: AIFirstHandler::new(),
            compliance: ComplianceHandler::new(),
            hardware_tuning: HardwareTuningHandler::new(),
            health: HealthHandler::new(),
            load_testing: LoadTestHandler::new(),
            metrics_collector: MetricsCollector::new(),
            performance_analyzer: PerformanceAnalyzer::new(),
            storage: StorageHandler::new(),
            workspace_manager: WorkspaceManager::new(),
            zfs: ZfsHandler::new(),
        }
    }

    /// Register all handlers with a router or service
    pub fn register_all<R>(&self, router: &mut R)
    where
        R: HandlerRegistry,
    {
        router.register_handler("ai_first", &self.ai_first);
        router.register_handler("compliance", &self.compliance);
        router.register_handler("hardware_tuning", &self.hardware_tuning);
        router.register_handler("health", &self.health);
        router.register_handler("load_testing", &self.load_testing);
        router.register_handler("metrics", &self.metrics_collector);
        router.register_handler("performance", &self.performance_analyzer);
        router.register_handler("storage", &self.storage);
        router.register_handler("workspace", &self.workspace_manager);
        router.register_handler("zfs", &self.zfs);
    }
}

impl Default for HandlerCollection {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for registering handlers with a router
pub trait HandlerRegistry {
    /// Register a new handler with the registry
    fn register_handler<H>(&mut self, name: &str, handler: &H);
}
// ==================== HANDLER UTILITIES ====================

/// Initialize all handlers with default configuration
#[must_use]
pub fn initialize_handlers() -> HandlerCollection {
    HandlerCollection::new()
}
/// Create a specific handler by name
///
/// # Arguments
///
/// * `name` - The name of the handler to create ("`ai_first`", "compliance", "`hardware_tuning`", etc.)
///
/// # Returns
///
/// Returns `Some(Box<dyn Any>)` containing the handler if the name is valid, `None` otherwise.
///
/// # Examples
///
/// ```
/// use nestgate_api::handlers::create_handler_by_name;
///
/// let handler = create_handler_by_name("storage");
/// assert!(handler.is_some());
/// ```
#[must_use]
pub fn create_handler_by_name(name: &str) -> Option<Box<dyn std::any::Any>> {
    match name {
        "ai_first" => Some(Box::new(ai_first_example::create_handler())),
        "compliance" => Some(Box::new(ComplianceHandler::new())),
        "hardware_tuning" => Some(Box::new(HardwareTuningHandler::new())),
        "health" => Some(Box::new(HealthHandler::new())),
        "load_testing" => Some(Box::new(LoadTestHandler::new())),
        "metrics" => Some(Box::new(MetricsCollector::new())),
        "performance" => Some(Box::new(PerformanceAnalyzer::new())),
        "storage" => Some(Box::new(StorageHandler::new())),
        "workspace" => Some(Box::new(WorkspaceManager::new())),
        "zfs" => Some(Box::new(ZfsHandler::new())),
        _ => None,
    }
}
/// Get list of all available handler names
///
/// Returns a vector containing the names of all registered handlers.
///
/// # Returns
///
/// A vector of static string slices containing handler names:
/// - "`ai_first`" - AI-powered operations
/// - "compliance" - Regulatory compliance
/// - "`hardware_tuning`" - Hardware performance tuning
/// - "health" - System health monitoring
/// - "`load_testing`" - Load testing framework
/// - "metrics" - Metrics collection
/// - "performance" - Performance analysis
/// - "storage" - Storage management
/// - "workspace" - Workspace management
/// - "zfs" - ZFS storage operations
///
/// # Examples
///
/// ```
/// use nestgate_api::handlers::available_handlers;
///
/// let handlers = available_handlers();
/// assert!(handlers.contains(&"storage"));
/// assert_eq!(handlers.len(), 10);
/// ```
#[must_use]
pub fn available_handlers() -> Vec<&'static str> {
    vec![
        "ai_first",
        "compliance",
        "hardware_tuning",
        "health",
        "load_testing",
        "metrics",
        "performance",
        "storage",
        "workspace",
        "zfs",
    ]
}

// ==================== HANDLER TYPE DEFINITIONS ====================

/// AI-First handler for intelligent operations
#[derive(Debug, Clone)]
/// Handler for `AIFirst` requests
pub struct AIFirstHandler {
    /// HTTP router for AI-first endpoints
    pub router: Router,
}

impl Default for AIFirstHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl AIFirstHandler {
    /// Create a new AI-First handler with default router
    #[must_use]
    pub fn new() -> Self {
        Self {
            router: ai_first_example::create_handler(),
        }
    }
}

/// Compliance handler for regulatory compliance
#[derive(Debug, Clone)]
/// Handler for Compliance requests
pub struct ComplianceHandler {
    /// Compliance state manager for regulatory tracking
    pub manager: compliance::ComplianceState,
}

impl Default for ComplianceHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ComplianceHandler {
    /// Create a new compliance handler with default manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            manager: std::sync::Arc::new(tokio::sync::RwLock::new(
                compliance::ComplianceManager::default(),
            )),
        }
    }
}

/// Hardware tuning handler for performance optimization
#[derive(Debug, Clone)]
/// Handler for `HardwareTuning` requests
pub struct HardwareTuningHandler {
    /// Hardware tuning configuration settings
    pub config: HardwareTuningConfig,
}

impl Default for HardwareTuningHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareTuningHandler {
    /// Create a new hardware tuning handler with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HardwareTuningConfig::default(),
        }
    }
}

/// Health check handler for system monitoring
#[derive(Debug, Clone)]
/// Handler for Health requests
pub struct HealthHandler;

impl Default for HealthHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl HealthHandler {
    /// Create a new health check handler
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Load testing handler for performance testing
#[derive(Debug, Clone)]
/// Handler for `LoadTest` requests
pub struct LoadTestHandler {
    /// Load testing configuration parameters and settings
    pub config: load_testing::config::LoadTestConfig,
}

impl Default for LoadTestHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadTestHandler {
    /// Create a new load testing handler with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: load_testing::config::LoadTestConfig::default(),
        }
    }
}

/// Metrics collector for system metrics
#[derive(Debug, Clone)]
/// Metricscollector
pub struct MetricsCollector {
    /// Metrics collector state and data aggregation _engine
    pub collector: metrics_collector::MetricsCollectorState,
}

impl Default for MetricsCollector {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    /// Create a new metrics collector with default state
    #[must_use]
    pub fn new() -> Self {
        Self {
            collector: metrics_collector::MetricsCollectorState::default(),
        }
    }
}

/// Performance analyzer for system analysis
#[derive(Debug, Clone)]
/// Performanceanalyzer
pub struct PerformanceAnalyzer {
    /// Performance analyzer state and trend detection _engine
    pub analyzer: performance_analyzer::PerformanceAnalyzerState,
}

impl Default for PerformanceAnalyzer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceAnalyzer {
    /// Create a new performance analyzer with default state
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyzer: performance_analyzer::PerformanceAnalyzerState::default(),
        }
    }
}

/// Storage handler for storage operations
#[derive(Debug, Clone)]
/// Handler for Storage requests
pub struct StorageHandler {
    /// Storage management _engine for configuration and operations
    pub manager: storage::StorageManager,
}

impl Default for StorageHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl StorageHandler {
    /// Create a new storage handler with default manager
    #[must_use]
    pub const fn new() -> Self {
        Self {
            manager: storage::StorageManager::new(),
        }
    }
}

/// Workspace manager for workspace operations
#[derive(Debug, Clone)]
/// Manager for Workspace operations
pub struct WorkspaceManager {
    /// Workspace management implementation
    pub manager: workspace_management::WorkspaceManager,
}

impl Default for WorkspaceManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceManager {
    /// Create a new workspace manager with default configuration
    #[must_use]
    pub const fn new() -> Self {
        Self {
            manager: workspace_management::WorkspaceManager::new(),
        }
    }
}

/// ZFS handler for ZFS operations
#[derive(Debug, Clone)]
/// Handler for Zfs requests
pub struct ZfsHandler {
    /// ZFS handler implementation for pool and dataset management
    pub handler: ZfsHandlerImpl,
}

impl Default for ZfsHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ZfsHandler {
    /// Create a new ZFS handler with default implementation
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handler: ZfsHandlerImpl::new(),
        }
    }
}

/// **COMPLIANCE MANAGER**
///
/// Manager for compliance and regulatory requirements.
#[derive(Debug, Clone)]
/// Manager for Compliance operations
pub struct ComplianceManager {
    /// Current compliance state and configuration
    pub manager: compliance::ComplianceState,
}

impl Default for ComplianceManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ComplianceManager {
    /// Create a new compliance manager instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            manager: compliance::ComplianceState::default(),
        }
    }
}

/// **HARDWARE TUNING MANAGER**
///
/// Manager for hardware tuning and optimization.
#[derive(Debug, Clone)]
/// Manager for `HardwareTuning` operations
pub struct HardwareTuningManager {
    /// Hardware tuning configuration settings
    pub config: HardwareTuningConfig,
}

impl Default for HardwareTuningManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareTuningManager {
    /// Create a new hardware tuning manager instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HardwareTuningConfig::default(),
        }
    }
}

/// **LOAD TEST MANAGER**
///
/// Manager for load testing operations and scenarios.
#[derive(Debug, Clone)]
/// Manager for `LoadTest` operations
pub struct LoadTestManager {
    /// Load testing configuration
    pub config: load_testing::config::LoadTestConfig,
}

impl Default for LoadTestManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadTestManager {
    /// Create a new load test manager instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: load_testing::config::LoadTestConfig::default(),
        }
    }
}

/// **PERFORMANCE ANALYZER MANAGER**
///
/// Manager for performance analysis operations.
#[derive(Debug, Clone)]
/// Manager for `PerformanceAnalyzer` operations
pub struct PerformanceAnalyzerManager {
    /// Performance analyzer state
    pub analyzer: performance_analytics::PerformanceAnalyzerState,
}

impl Default for PerformanceAnalyzerManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceAnalyzerManager {
    /// Create a new performance analyzer manager instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyzer: performance_analytics::PerformanceAnalyzerState::default(),
        }
    }
}

/// **WORKSPACE MANAGER WRAPPER**
///
/// Wrapper for workspace management operations.
#[derive(Debug, Clone)]
/// Workspacemanagerwrapper
pub struct WorkspaceManagerWrapper {
    /// Workspace management instance
    pub manager: workspace_management::WorkspaceManager,
}

impl Default for WorkspaceManagerWrapper {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceManagerWrapper {
    /// Create a new workspace manager wrapper instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            manager: workspace_management::WorkspaceManager::new(),
        }
    }
}

/// **ZFS MANAGER**
///
/// Manager for ZFS operations and pool management.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Handler field used for ZFS operations
/// Manager for Zfs operations
pub struct ZfsManager {
    /// ZFS handler implementation
    pub handler: ZfsHandlerImpl,
}

impl Default for ZfsManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ZfsManager {
    /// Create a new ZFS manager instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handler: ZfsHandlerImpl::new(),
        }
    }
}

/// **API ROUTER**
///
/// Main router configuration for the `NestGate` API.
#[derive(Debug, Clone)]
/// Apirouter
pub struct ApiRouter {
    /// Router instance with all configured routes
    pub router: Router,
}

impl Default for ApiRouter {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRouter {
    /// Create a new API router instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

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
