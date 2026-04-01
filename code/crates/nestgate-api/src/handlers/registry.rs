// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Handler collection, registry trait, and discovery helpers.

use crate::handlers::{
    AIFirstHandler, ComplianceHandler, HardwareTuningHandler, HealthHandler, LoadTestHandler,
    MetricsCollector, PerformanceAnalyzer, StorageHandler, WorkspaceManager, ZfsHandler,
    ai_first_example,
};

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
