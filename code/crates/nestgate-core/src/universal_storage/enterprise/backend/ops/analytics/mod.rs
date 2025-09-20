//! Enterprise Storage Analytics Operations
//!
//! This module provides comprehensive analytics functionality for enterprise storage,
//! including performance monitoring, usage analysis, trend prediction, and optimization insights.
//!
//! The module is organized into focused submodules for better maintainability:
//! - `config`: Analytics configuration types
//! - `metrics`: Performance, usage, I/O, and health metrics
//! - `reports`: Analytics reports and recommendations  
//! - `manager`: Main analytics manager implementation

pub mod config;
pub mod metrics;
pub mod reports;
pub mod manager;

// Re-export commonly used types for convenience
pub use config::{AnalyticsConfig, PerformanceThresholds};
pub use metrics::{
    StorageAnalyticsPoint, PerformanceMetrics, UsageMetrics, 
    IoMetrics, HealthMetrics, ScrubStatus
};
pub use reports::{
    AnalyticsReport, PerformanceSummary, UsageTrends, HealthAssessment,
    Trend, OptimizationRecommendation, RecommendationCategory, Priority, 
    Difficulty, PredictiveInsights, ThresholdAlert, AlertSeverity
};
pub use manager::EnterpriseAnalyticsManager; 