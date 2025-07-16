//! NestGate ZFS Storage Management
//!
//! Advanced ZFS pool, dataset, and tier management with AI-driven optimization

// Core modules
pub mod advanced_features;
pub mod automation;
pub mod byob;
pub mod command;
pub mod config;
pub mod dataset;
pub mod error;
pub mod failover;
pub mod health;
pub mod manager;
pub mod mcp_integration;
pub mod metrics;
pub mod migration;
pub mod orchestrator_integration;
pub mod performance;
pub mod performance_engine;
pub mod pool;
pub mod pool_setup;
pub mod snapshot;
pub mod tier;
pub mod types; // New failover module for high availability

pub use advanced_features::{
    AdvancedSnapshotManager, IntelligentReplicationManager, PredictiveAnalyticsEngine,
};
pub use automation::{AutomationPolicy, DatasetAutomation, DatasetAutomationConfig};
pub use byob::{
    create_zfs_storage_provider, ByobStorageProvider, ByobStorageRequest, ByobStorageResponse,
    ZfsStorageProvider,
};
pub use command::{
    CommandResult, PoolStatus, ZfsCommand, ZfsDataset, ZfsOperations, ZfsPool, ZfsSnapshot,
};
#[cfg(feature = "orchestrator")]
// Type exports
pub use config::{TierConfig, TierConfigurations, ZfsConfig};
pub use dataset::{DatasetInfo, ZfsDatasetManager};
pub use error::{ZfsError, ZfsResult as Result};
pub use health::ZfsHealthMonitor;
pub use manager::ZfsManager;
pub use mcp_integration::{ZfsMcpConfig, ZfsMcpStorageProvider};
pub use migration::MigrationEngine as ZfsMigrationEngine;
pub use orchestrator_integration::*;
pub use performance::ZfsPerformanceMonitor;
pub use performance_engine::{AccessPattern, PerformanceOptimizationEngine};
pub use pool::{PoolInfo, ZfsPoolManager};
pub use pool_setup::{
    setup_production_zfs,
    CacheThresholds,
    DeviceDetectionConfig,
    DeviceType,
    IoThresholds,
    L2ArcLimits,
    MemoryLimits,
    MigrationThresholds,
    OptimizationIntervals,
    PerformanceConfig,
    PoolPropertyConfig,
    PoolSetupConfig,
    // New configuration types
    PoolSetupConfiguration,
    // Enhanced error handling
    PoolSetupError,
    PoolSetupResult,
    SafetyConfig,
    SpeedClass,
    StorageDevice,
    SystemReport,
    TierLimits,
    TierProperties,
    TierSetupConfig,
    ValidationResult,
    ZfsPoolSetup,
};
pub use snapshot::{SnapshotInfo, ZfsSnapshotManager};
pub use tier::TierManager as ZfsTierManager;
pub use types::*;

// Re-export Songbird integration
#[cfg(feature = "orchestrator")]
pub use orchestrator_integration::{NestGateZfsService, ZfsHealthStatus, ZfsServiceConfig};

// Re-export main types for convenience
pub use nestgate_core::NestGateError;

/// Utility function to check if ZFS is available on the system
pub async fn is_zfs_available() -> bool {
    #[cfg(feature = "zfs")]
    {
        // Use our command framework to check ZFS availability
        command::ZfsCommand::check_zfs_available()
            .await
            .unwrap_or(false)
    }

    #[cfg(not(feature = "zfs"))]
    {
        false
    }
}

/// Get ZFS system information
pub async fn get_system_info() -> Result<ZfsSystemInfo> {
    Ok(ZfsSystemInfo {
        zfs_available: is_zfs_available().await,
        version: env!("CARGO_PKG_VERSION").to_string(),
        features: features(),
        supported_tiers: vec!["hot".to_string(), "warm".to_string(), "cold".to_string()],
    })
}

/// System information structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZfsSystemInfo {
    pub zfs_available: bool,
    pub version: String,
    pub features: Vec<String>,
    pub supported_tiers: Vec<String>,
}

/// Get list of enabled features
pub fn features() -> Vec<String> {
    let mut features = vec!["core".to_string()];
    // Add conditional features based on compilation flags
    #[cfg(feature = "advanced")]
    features.push("advanced".to_string());
    #[cfg(feature = "performance")]
    features.push("performance".to_string());

    // Always include ZFS
    features.push("zfs".to_string());

    features
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ZfsConfig;

    #[test]
    fn test_zfs_config_creation() {
        let config = ZfsConfig::default();

        // Test default tier configurations
        let hot_tier = config.get_tier_config(&nestgate_core::StorageTier::Hot);
        assert_eq!(hot_tier.name, "hot");
        assert_eq!(
            hot_tier
                .properties
                .get("compression")
                .expect("Hot tier should have compression property"),
            "lz4"
        );

        let warm_tier = config.get_tier_config(&nestgate_core::StorageTier::Warm);
        assert_eq!(warm_tier.name, "warm");
        assert_eq!(
            warm_tier
                .properties
                .get("compression")
                .expect("Warm tier should have compression property"),
            "zstd"
        );

        let cold_tier = config.get_tier_config(&nestgate_core::StorageTier::Cold);
        assert_eq!(cold_tier.name, "cold");
        assert_eq!(
            cold_tier
                .properties
                .get("compression")
                .expect("Cold tier should have compression property"),
            "gzip-9"
        );
    }

    #[tokio::test]
    async fn test_enhanced_zfs_manager_creation() {
        let config = ZfsConfig::default();

        // This test might fail in environments without ZFS
        // but it validates the manager creation logic
        match ZfsManager::new(config).await {
            Ok(manager) => {
                // Note: AI integration has been sunset - NestGate focuses on data management
                // while Toadstool handles AI/GPU compute workloads

                // Verify performance monitoring is available
                let monitor = manager.performance_monitor.read().await;
                let current_metrics = monitor.get_current_metrics().await;
                assert!(current_metrics.timestamp > std::time::SystemTime::UNIX_EPOCH);
            }
            Err(e) => {
                // Expected in test environments without ZFS
                println!("ZFS Manager creation failed (expected in test env): {e}");
            }
        }
    }

    #[test]
    fn test_data_management_focus() {
        // NestGate now focuses on data management capabilities
        // AI/ML tier prediction is handled by external systems via API
        let config = ZfsConfig::default();

        // Verify all three tiers are properly configured
        assert!(!config.tiers.hot.name.is_empty());
        assert!(!config.tiers.warm.name.is_empty());
        assert!(!config.tiers.cold.name.is_empty());

        // Verify tier names match expected values
        assert_eq!(config.tiers.hot.name, "hot");
        assert_eq!(config.tiers.warm.name, "warm");
        assert_eq!(config.tiers.cold.name, "cold");
    }

    #[test]
    fn test_performance_config_defaults() {
        let perf_config = crate::performance::PerformanceConfig::default();

        assert_eq!(perf_config.collection_interval, 30);
        assert_eq!(perf_config.analysis_interval, 300);
        assert_eq!(perf_config.alert_interval, 60);
        assert_eq!(perf_config.history_retention_hours, 24);
        assert_eq!(perf_config.max_history_entries, 2880);
        assert!(perf_config.enable_alerting);
        assert!(perf_config.enable_trend_analysis);
    }

    #[test]
    fn test_tier_metrics_performance_hierarchy() {
        let hot_metrics =
            crate::performance::TierMetrics::default_for_tier(nestgate_core::StorageTier::Hot);
        let warm_metrics =
            crate::performance::TierMetrics::default_for_tier(nestgate_core::StorageTier::Warm);
        let cold_metrics =
            crate::performance::TierMetrics::default_for_tier(nestgate_core::StorageTier::Cold);

        // Verify performance hierarchy: Hot > Warm > Cold
        assert!(hot_metrics.read_iops > warm_metrics.read_iops);
        assert!(warm_metrics.read_iops > cold_metrics.read_iops);

        assert!(hot_metrics.read_throughput_mbs > warm_metrics.read_throughput_mbs);
        assert!(warm_metrics.read_throughput_mbs > cold_metrics.read_throughput_mbs);

        // Latency should be inverse: Hot < Warm < Cold
        assert!(hot_metrics.avg_read_latency_ms < warm_metrics.avg_read_latency_ms);
        assert!(warm_metrics.avg_read_latency_ms < cold_metrics.avg_read_latency_ms);
    }
}
