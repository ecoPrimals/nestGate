//! ZFS Manager - Main orchestrator for ZFS operations
//!
//! Enhanced ZFS manager that integrates advanced patterns with v2 orchestrator architecture,
//! AI integration, and comprehensive performance monitoring. This module coordinates all
//! ZFS operations including pool management, dataset operations, snapshots, tier management,
//! and automated optimization.

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    automation::DatasetAutomation, config::ZfsConfig, dataset::ZfsDatasetManager,
    health::ZfsHealthMonitor, metrics::ZfsMetrics, migration::MigrationEngine,
    performance::ZfsPerformanceMonitor, pool::ZfsPoolManager, snapshot::ZfsSnapshotManager,
    tier::TierManager,
};
use nestgate_automation::DatasetAnalyzer;

#[cfg(feature = "orchestrator")]
use nestgate_mcp::OrchestratorClient;

// Module declarations
pub mod ai_tier_optimization;
pub mod dataset_operations;
pub mod health;
pub mod initialization;
pub mod performance;
pub mod pool_operations;
pub mod types;
pub mod utilities;

// Re-export types for convenience
pub use types::*;

/// Enhanced ZFS Manager integrating AI and performance monitoring
pub struct ZfsManager {
    /// Pool management operations
    pub pool_manager: Arc<ZfsPoolManager>,
    /// Dataset management operations
    pub dataset_manager: Arc<ZfsDatasetManager>,
    /// Snapshot management operations
    pub snapshot_manager: Arc<ZfsSnapshotManager>,
    /// Migration engine for tier optimization
    pub migration_engine: Arc<RwLock<MigrationEngine>>,
    /// Dataset analysis and automation
    pub dataset_analyzer: Arc<DatasetAnalyzer>,
    /// Performance monitoring
    pub performance_monitor: Arc<RwLock<ZfsPerformanceMonitor>>,
    /// Tiered storage management
    pub tier_manager: Arc<TierManager>,
    /// Health monitoring system
    pub health_monitor: Option<Arc<RwLock<ZfsHealthMonitor>>>,
    /// Performance metrics collection
    pub metrics: Arc<ZfsMetrics>,
    /// Automation for dataset lifecycle management
    pub automation: Option<Arc<DatasetAutomation>>,
    /// Configuration management
    pub config: ZfsConfig,
    /// Optional orchestrator client
    #[cfg(feature = "orchestrator")]
    orchestrator_client: Option<Arc<dyn OrchestratorClient>>,
}

impl std::fmt::Debug for ZfsManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZfsManager")
            .field("pool_manager", &self.pool_manager)
            .field("dataset_manager", &self.dataset_manager)
            .field("snapshot_manager", &self.snapshot_manager)
            .field("migration_engine", &self.migration_engine)
            .field("dataset_analyzer", &self.dataset_analyzer)
            .field("performance_monitor", &self.performance_monitor)
            .field("tier_manager", &self.tier_manager)
            .field("health_monitor", &self.health_monitor)
            .field("metrics", &self.metrics)
            .field("automation", &self.automation)
            .field("config", &self.config)
            .field("orchestrator_client", &"<trait object>")
            .finish()
    }
}

// All implementations are provided by the individual modules through impl blocks
// - initialization.rs: new(), start(), stop(), register_with_orchestrator()
// - pool_operations.rs: create_pool(), destroy_pool(), get_pool_status(), scrub_pool()
// - dataset_operations.rs: create_dataset(), destroy_dataset(), list_snapshots()
// - health.rs: get_service_status(), get_zfs_health(), get_real_health_state()
// - ai_tier_optimization.rs: get_ai_tier_recommendation(), heuristic recommendations
// - performance.rs: get_performance_analytics(), trigger_optimization(), shutdown()
// - utilities.rs: _calculate_system_utilization(), parsing methods
