//! NestGate ZFS Storage Management
//! 
//! Advanced ZFS pool, dataset, and tier management with AI-driven optimization

// Core modules
pub mod config;
pub mod command;
pub mod dataset;
pub mod error;
pub mod manager;
pub mod pool;
pub mod snapshot;
pub mod health;
pub mod tier;
pub mod migration;
pub mod ai_integration;
pub mod performance;
pub mod mcp_integration;
pub mod advanced_features;
pub mod pool_setup;
pub mod automation;
pub mod types;
pub mod performance_engine;
pub mod orchestrator_integration;
pub mod metrics;

#[cfg(feature = "orchestrator")]
pub mod orchestrator;

// Type exports
pub use config::{ZfsConfig, TierConfig, TierConfigurations};
pub use command::{ZfsCommand, ZfsOperations, CommandResult, ZfsPool, ZfsDataset, ZfsSnapshot, PoolStatus};
pub use dataset::{ZfsDatasetManager, DatasetInfo};
pub use error::{ZfsError, ZfsResult as Result};
pub use manager::ZfsManager;
pub use pool::{ZfsPoolManager, PoolInfo};
pub use snapshot::{ZfsSnapshotManager, SnapshotInfo};
pub use health::ZfsHealthMonitor;
pub use tier::TierManager as ZfsTierManager;
pub use migration::MigrationEngine as ZfsMigrationEngine;
pub use ai_integration::ZfsAiIntegration;
pub use performance::ZfsPerformanceMonitor;
pub use mcp_integration::{ZfsMcpStorageProvider, ZfsMcpConfig};
pub use pool_setup::{
    ZfsPoolSetup, 
    StorageDevice, 
    DeviceType, 
    SpeedClass, 
    PoolSetupConfig, 
    PoolSetupResult, 
    SystemReport, 
    setup_production_zfs,
    // New configuration types
    PoolSetupConfiguration,
    PoolPropertyConfig,
    DeviceDetectionConfig,
    SafetyConfig,
    PerformanceConfig,
    CacheThresholds,
    IoThresholds,
    MemoryLimits,
    OptimizationIntervals,
    TierSetupConfig,
    TierProperties,
    MigrationThresholds,
    TierLimits,
    L2ArcLimits,
    // Enhanced error handling
    PoolSetupError,
    ValidationResult,
};
pub use automation::IntelligentDatasetManager;
pub use advanced_features::{
    PredictiveAnalyticsEngine,
    IntelligentReplicationManager, 
    AdvancedSnapshotManager,
};
pub use performance_engine::{PerformanceOptimizationEngine, AccessPattern};
pub use types::*;
pub use orchestrator_integration::*;

// Re-export Songbird integration
#[cfg(feature = "orchestrator")]
pub use orchestrator_integration::{
    NestGateZfsService, ZfsServiceConfig, ZfsHealthStatus
};

// Re-export main types for convenience
pub use nestgate_core::NestGateError;

/// Utility function to check if ZFS is available on the system
pub async fn is_zfs_available() -> bool {
    #[cfg(feature = "zfs")]
    {
        // Use our command framework to check ZFS availability
        command::ZfsCommand::check_zfs_available().await.unwrap_or(false)
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
        features: get_enabled_features(),
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
fn get_enabled_features() -> Vec<String> {
    let mut features = vec!["core".to_string()];
    
    #[cfg(feature = "zfs")]
    features.push("zfs".to_string());
    
    #[cfg(feature = "orchestrator")]
    features.push("orchestrator".to_string());
    
    features
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zfs_config_creation() {
        let config = ZfsConfig::default();
        
        // Test default tier configurations
        let hot_tier = config.get_tier_config(&nestgate_core::StorageTier::Hot);
        assert_eq!(hot_tier.name, "hot");
        assert_eq!(hot_tier.properties.get("compression").expect("Hot tier should have compression property"), "lz4");
        
        let warm_tier = config.get_tier_config(&nestgate_core::StorageTier::Warm);
        assert_eq!(warm_tier.name, "warm");
        assert_eq!(warm_tier.properties.get("compression").expect("Warm tier should have compression property"), "zstd");
        
        let cold_tier = config.get_tier_config(&nestgate_core::StorageTier::Cold);
        assert_eq!(cold_tier.name, "cold");
        assert_eq!(cold_tier.properties.get("compression").expect("Cold tier should have compression property"), "gzip-9");
    }
    
    #[tokio::test]
    async fn test_enhanced_zfs_manager_creation() {
        let config = ZfsConfig::default();
        
        // This test might fail in environments without ZFS
        // but it validates the manager creation logic
        match ZfsManager::new(config).await {
            Ok(manager) => {
                // Verify AI integration status
                if manager.ai_integration.is_some() {
                    println!("AI integration enabled");
                } else {
                    println!("AI integration disabled or failed to initialize");
                }
                
                // Verify performance monitoring is available
                let monitor = manager.performance_monitor.read().await;
                let current_metrics = monitor.get_current_metrics().await;
                assert!(current_metrics.timestamp > std::time::SystemTime::UNIX_EPOCH);
            }
            Err(e) => {
                // Expected in test environments without ZFS
                println!("ZFS Manager creation failed (expected in test env): {}", e);
            }
        }
    }
    
    #[test]
    fn test_ai_config_defaults() {
        let ai_config = crate::ai_integration::ZfsAiConfig::default();
        
        assert!(ai_config.enable_tier_optimization);
        assert!(ai_config.enable_predictive_analytics);
        assert!(ai_config.enable_anomaly_detection);
        assert_eq!(ai_config.optimization_interval, 3600);
        assert_eq!(ai_config.analytics_interval, 300);
        assert_eq!(ai_config.min_confidence_threshold, 0.7);
        assert_eq!(ai_config.max_concurrent_models, 3);
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
        let hot_metrics = crate::performance::TierMetrics::default_for_tier(nestgate_core::StorageTier::Hot);
        let warm_metrics = crate::performance::TierMetrics::default_for_tier(nestgate_core::StorageTier::Warm);
        let cold_metrics = crate::performance::TierMetrics::default_for_tier(nestgate_core::StorageTier::Cold);
        
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