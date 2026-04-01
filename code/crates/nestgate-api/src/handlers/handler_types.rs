// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Concrete handler and manager wrapper types for the API layer.

use crate::handlers::hardware_tuning::HardwareTuningConfig;

#[cfg(feature = "dev-stubs")]
use crate::handlers::zfs::basic::ZfsHandlerImpl;
#[cfg(not(feature = "dev-stubs"))]
use crate::handlers::zfs::production_placeholders::ZfsHandlerImpl;
use axum::Router;

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
            router: crate::handlers::ai_first_example::create_handler(),
        }
    }
}

/// Compliance handler for regulatory compliance
#[derive(Debug, Clone)]
/// Handler for Compliance requests
pub struct ComplianceHandler {
    /// Compliance state manager for regulatory tracking
    pub manager: crate::handlers::compliance::ComplianceState,
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
                crate::handlers::compliance::ComplianceManager::default(),
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
    pub config: crate::handlers::load_testing::config::LoadTestConfig,
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
            config: crate::handlers::load_testing::config::LoadTestConfig::default(),
        }
    }
}

/// Metrics collector for system metrics
#[derive(Debug, Clone)]
/// Metricscollector
pub struct MetricsCollector {
    /// Metrics collector state and data aggregation _engine
    pub collector: crate::handlers::metrics_collector::MetricsCollectorState,
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
            collector: crate::handlers::metrics_collector::MetricsCollectorState::default(),
        }
    }
}

/// Performance analyzer for system analysis
#[derive(Debug, Clone)]
/// Performanceanalyzer
pub struct PerformanceAnalyzer {
    /// Performance analyzer state and trend detection _engine
    pub analyzer: crate::handlers::performance_analyzer::PerformanceAnalyzerState,
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
            analyzer: crate::handlers::performance_analyzer::PerformanceAnalyzerState::default(),
        }
    }
}

/// Storage handler for storage operations
#[derive(Debug, Clone)]
/// Handler for Storage requests
pub struct StorageHandler {
    /// Storage management _engine for configuration and operations
    pub manager: crate::handlers::storage::StorageManager,
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
            manager: crate::handlers::storage::StorageManager::new(),
        }
    }
}

/// Workspace manager for workspace operations
#[derive(Debug, Clone)]
/// Manager for Workspace operations
pub struct WorkspaceManager {
    /// Workspace management implementation
    pub manager: crate::handlers::workspace_management::WorkspaceManager,
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
            manager: crate::handlers::workspace_management::WorkspaceManager::new(),
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
    pub manager: crate::handlers::compliance::ComplianceState,
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
            manager: crate::handlers::compliance::ComplianceState::default(),
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
    pub config: crate::handlers::load_testing::config::LoadTestConfig,
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
            config: crate::handlers::load_testing::config::LoadTestConfig::default(),
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
    pub analyzer: crate::handlers::performance_analytics::PerformanceAnalyzerState,
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
            analyzer: crate::handlers::performance_analytics::PerformanceAnalyzerState::default(),
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
    pub manager: crate::handlers::workspace_management::WorkspaceManager,
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
            manager: crate::handlers::workspace_management::WorkspaceManager::new(),
        }
    }
}

/// **ZFS MANAGER**
///
/// Manager for ZFS operations and pool management.
#[derive(Debug, Clone)]
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
