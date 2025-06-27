//! ZFS Automation Integration
//!
//! This module provides integration between ZFS storage management and the
//! NestGate automation system. It re-exports the main automation functionality
//! from the dedicated nestgate-automation crate.

// Re-export main automation types and functionality
pub use nestgate_automation::{
    IntelligentDatasetManager,
    FileAnalyzer,
    AccessPatternAnalyzer,
    DatasetAnalyzer,
    DatasetAnalysis,
    TierPredictor,
    DatasetLifecycleManager,
    AutomationConfig,
    TierPrediction,
    FileAnalysis,
    AccessPatterns,
    OptimizationResult,
    TierPerformanceStats,
    AiPredictionResult,
    Result as AutomationResult,
};

// Backward compatibility aliases
pub use nestgate_automation::AiPredictionResult as PredictionResult;

// Re-export ecosystem integration types (when network integration is enabled)
#[cfg(feature = "network-integration")]
pub use nestgate_automation::{
    EcosystemDiscovery,
    ServiceConnectionPool,
    SquirrelConnection,
    ServicePlan,
    EcosystemService,
    ServiceRegistration,
    TierDiscoveryRequest,
    TierDiscoveryResponse,
    DatasetCreatedNotification,
};

// Legacy compatibility types that some existing code might still reference
pub use nestgate_automation::{
    FileType,
    FileCharacteristics,
    AccessEvent,
    AccessType,
    TaskPriority,
    ServiceHealth,
    TierStats,
    TrainingExample,
    StorageContext,
    DatasetContext,
};

/// Initialize automation for ZFS with default configuration
pub async fn initialize_zfs_automation() -> AutomationResult<IntelligentDatasetManager> {
    let zfs_config = nestgate_core::config::Config::default();
    nestgate_automation::initialize_automation(zfs_config).await
}

/// Initialize automation for ZFS with custom configuration
pub async fn initialize_zfs_automation_with_config(
    automation_config: AutomationConfig,
) -> AutomationResult<IntelligentDatasetManager> {
    let zfs_config = nestgate_core::config::Config::default();
    nestgate_automation::initialize_automation_with_config(zfs_config, automation_config).await
}

/// Check if ecosystem services are available for ZFS automation
#[cfg(feature = "network-integration")]
pub async fn check_zfs_ecosystem_availability() -> bool {
    nestgate_automation::check_ecosystem_availability().await
}

#[cfg(not(feature = "network-integration"))]
pub async fn check_zfs_ecosystem_availability() -> bool {
    false
} 