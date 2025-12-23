//
// This module provides enterprise-grade storage capabilities for NestGate,
// organized into focused submodules for better maintainability.
//
// ## Architecture
//
// - **snapshots**: Snapshot creation, management, and restoration
// - **replication**: Multi-target replication and backup systems  
// - **analytics**: Performance metrics and optimization analysis
// - **tiering**: Intelligent storage tiering and migration
// - **traits**: Core enterprise storage trait definitions
// - **backend**: Enterprise filesystem backend implementation
// - **advanced_features**: ML-driven optimization and predictive analytics

//! Enterprise module

pub mod features;
pub mod analytics;
pub mod backend;
pub mod replication;
pub mod snapshots;
pub mod types;
pub mod tiering;
pub mod traits;

// Re-export main types for convenience
pub use advanced_features::{
    AdvancedStorageManagement, AdvancedStorageManager, DisasterRecoveryPlan,
    IntelligentOptimizationReport, PolicyReport, StorageAnomaly, StorageForecast, StoragePolicy,
};
pub use analytics::{
    DeduplicationReport, DetailedMetrics, DuplicateGroup, EstimatedSavings, ImplementationEffort,
    OptimizationCategory, OptimizationRecommendation, OptimizationReport, PerformanceInsight,
    Priority,
};
pub use backend::EnterpriseStorageBackend;
pub use replication::{
    BackupFileEntry, BackupManifest, BackupType, ReplicationJob, ReplicationStatus, StorageTarget,
    TargetType,
};
pub use snapshots::SnapshotInfo;
pub use tiering::{AccessPattern, TierDistribution, TierMigration, TieringReport};
pub use traits::EnterpriseStorageCapabilities;
