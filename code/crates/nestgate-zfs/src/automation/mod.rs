//! ZFS Automation Integration
//!
//! This module provides integration between ZFS storage management and the
//! NestGate automation system. It offers intelligent dataset lifecycle management,
//! automated tier optimization, and policy-driven automation.
//!
//! The automation system includes:
//! - Intelligent tier evaluation and recommendation
//! - Automated dataset lifecycle management
//! - Policy-driven automation with customizable rules
//! - Integration with the NestGate ecosystem
//! - Performance optimization and migration coordination

pub mod actions;
pub mod engine;
pub mod integration;
pub mod lifecycle;
pub mod policies;
pub mod tests;
pub mod tier_evaluation;
pub mod types;

// Re-export main automation types and functionality from nestgate-automation
pub use nestgate_automation::{
    AccessPatterns, AutomationConfig, DatasetAnalysis, DatasetAnalyzer, DatasetLifecycleManager,
    FileAnalysis, FileAnalyzer, IntelligentDatasetManager, OptimizationResult,
    Result as AutomationResult, TierPerformanceStats, TierPrediction, TierPredictor,
};

// Re-export ecosystem integration types (when network integration is enabled)
#[cfg(feature = "network-integration")]
pub use nestgate_automation::{
    DatasetCreatedNotification, EcosystemDiscovery, EcosystemService, ServiceConnectionPool,
    ServicePlan, ServiceRegistration, SquirrelConnection, TierDiscoveryRequest,
    TierDiscoveryResponse,
};

// Legacy compatibility types that some existing code might still reference
pub use nestgate_automation::{
    AccessEvent, AccessType, DatasetContext, FileCharacteristics, FileType, ServiceHealth,
    StorageContext, TaskPriority, TierStats, TrainingExample,
};

// Re-export core types from our modules
pub use types::{
    AutomationEvent, AutomationEventType, AutomationPolicy, AutomationStatus, BandwidthLimits,
    DatasetLifecycle, DatasetMetadata, LifecycleRule, LifecycleStage, MigrationRule,
    PolicyConditions, PolicyPriority, TierRule,
};

// Re-export main engine
pub use engine::DatasetAutomation;

// Re-export integration functions
pub use integration::{
    check_zfs_ecosystem_availability, initialize_zfs_automation,
    initialize_zfs_automation_with_config,
};

// Re-export policy management
pub use policies::{
    AccessPatternRules, LifecycleRules, MigrationPerformanceLimits, MigrationRules,
    MigrationSchedule, PerformanceRequirement, PerformanceThresholds, TierAssignmentRules,
    TierSizeThresholds,
};

// Re-export configuration types
pub use crate::config::{AiAutomationSettings, DatasetAutomationConfig};
