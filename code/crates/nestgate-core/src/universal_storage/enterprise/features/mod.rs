// **ADVANCED STORAGE FEATURES - MODULAR**
//! Module definitions and exports.
// Advanced storage management capabilities organized into focused modules
//! for better maintainability and to keep individual files under 2000 lines.

pub mod optimization;
pub mod forecasting;
pub mod policies;
pub mod anomaly_detection;
pub mod disaster_recovery;

// Re-export core types for convenience
pub use optimization::{
    AdvancedStorageManagement, IntelligentOptimizationReport, 
    IntelligentRecommendation, PredictedImprovements
};

pub use forecasting::{
    StorageForecast, CapacityProjection, PerformanceProjection, CostProjection
};

pub use policies::{
    StoragePolicy, PolicyType, PolicyCondition, PolicyReport
};

pub use anomaly_detection::{
    StorageAnomaly, AnomalyType, AnomalySeverity
};

pub use disaster_recovery::{
    DisasterRecoveryPlan, RecoveryStrategy, BackupConfiguration
}; 