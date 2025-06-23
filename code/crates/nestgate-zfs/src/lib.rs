//! NestGate ZFS Integration
//! 
//! Enhanced ZFS integration with orchestrator support, AI optimization,
//! and comprehensive performance monitoring for tiered storage

pub mod manager;
pub mod config;
pub mod error;
pub mod pool;
pub mod dataset;
pub mod snapshot;
pub mod tier;
pub mod health;
pub mod metrics;
pub mod automation;
pub mod migration;
pub mod ai_integration;
pub mod performance;
pub mod types;
pub mod orchestrator_integration;
pub mod mcp_integration;

#[cfg(feature = "orchestrator")]
pub mod orchestrator;

pub use automation::{
    DatasetAnalyzer, FileCharacteristics, TierRecommendation, 
    AccessPattern, TierThresholds, PerformanceExpectation,
    AutomatedDatasetCreator, TierStatistics
};

pub use migration::{
    MigrationEngine, MigrationJob, MigrationStatus, MigrationPriority,
    MigrationConfig, MigrationStatistics
};

pub use ai_integration::{
    ZfsAiIntegration, ZfsAiConfig, TierPrediction, OptimizationOpportunity,
    PerformanceSnapshot as AiPerformanceSnapshot, TierAnalytics,
    OptimizationType, OptimizationComplexity
};

pub use performance::{
    ZfsPerformanceMonitor, PerformanceConfig, CurrentPerformanceMetrics,
    TierMetrics, AlertCondition, ActiveAlert, Alert, AlertSeverity,
    PerformanceSnapshot, TierPerformanceData
};

pub use manager::{
    ZfsManager, EnhancedServiceStatus, AiIntegrationStatus,
    PerformanceAnalytics, OptimizationResult
};

pub use config::ZfsConfig;
pub use error::{ZfsError, PoolError, DatasetError, SnapshotError, MigrationError};
pub use types::{StorageTier, CompressionAlgorithm, DatasetProperty};

// Re-export core types for convenience
pub use nestgate_core::{Result, NestGateError, StorageTier as CoreStorageTier};

// Re-export main types
pub use config::*;
pub use manager::*;
pub use orchestrator_integration::*;
pub use types::*;
pub use error::*;

/// Initialize ZFS system with default configuration
pub async fn initialize_zfs() -> Result<ZfsManager> {
    let config = ZfsConfig::default();
    ZfsManager::new(config).await
}

/// Initialize ZFS system with custom configuration
pub async fn initialize_zfs_with_config(config: ZfsConfig) -> Result<ZfsManager> {
    // Validate configuration before initialization
    config.validate()?;
    ZfsManager::new(config).await
}

/// Load ZFS configuration from file
pub async fn load_config_from_file(path: &std::path::Path) -> Result<ZfsConfig> {
    ZfsConfig::load_from_file(path).await
}

/// Create a default ZFS configuration with tier setup
pub fn create_default_config() -> ZfsConfig {
    ZfsConfig::default()
}

/// Utility function to check if ZFS is available on the system
pub async fn is_zfs_available() -> bool {
    #[cfg(feature = "zfs")]
    {
        // TODO: Implement actual ZFS availability check
        // For now, assume ZFS is available if the feature is enabled
        true
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
    use tempfile::tempdir;
    
    #[test]
    fn test_zfs_config_creation() {
        let config = ZfsConfig::default();
        
        // Test default tier configurations
        let hot_tier = config.get_tier_config(&nestgate_core::StorageTier::Hot);
        assert_eq!(hot_tier.name, "hot");
        assert_eq!(hot_tier.properties.get("compression").unwrap(), "lz4");
        
        let warm_tier = config.get_tier_config(&nestgate_core::StorageTier::Warm);
        assert_eq!(warm_tier.name, "warm");
        assert_eq!(warm_tier.properties.get("compression").unwrap(), "zstd");
        
        let cold_tier = config.get_tier_config(&nestgate_core::StorageTier::Cold);
        assert_eq!(cold_tier.name, "cold");
        assert_eq!(cold_tier.properties.get("compression").unwrap(), "gzip-9");
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
                assert!(manager.performance_monitor.get_current_metrics().await.timestamp > std::time::SystemTime::UNIX_EPOCH);
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
    
    #[test]
    fn test_optimization_opportunity_sorting() {
        let mut opportunities = vec![
            crate::ai_integration::OptimizationOpportunity {
                optimization_type: crate::ai_integration::OptimizationType::TierMigration,
                description: "Low impact optimization".to_string(),
                expected_impact: 5.0,
                confidence: 0.9,
                complexity: crate::ai_integration::OptimizationComplexity::Low,
                implementation_time: std::time::Duration::from_secs(60),
            },
            crate::ai_integration::OptimizationOpportunity {
                optimization_type: crate::ai_integration::OptimizationType::CompressionOptimization,
                description: "High impact optimization".to_string(),
                expected_impact: 25.0,
                confidence: 0.8,
                complexity: crate::ai_integration::OptimizationComplexity::Medium,
                implementation_time: std::time::Duration::from_secs(300),
            },
        ];
        
        // Sort by expected impact (descending)
        opportunities.sort_by(|a, b| b.expected_impact.partial_cmp(&a.expected_impact).unwrap_or(std::cmp::Ordering::Equal));
        
        assert_eq!(opportunities[0].expected_impact, 25.0);
        assert_eq!(opportunities[1].expected_impact, 5.0);
    }
} 