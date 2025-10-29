// **STORAGE OPTIMIZATION FEATURES**
//! Optimization functionality and utilities.
// Intelligent storage optimization with machine learning insights.

use crate::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// Re-exports for types from other modules
use super::forecasting::StorageForecast;
use super::policies::{StoragePolicy, PolicyReport};
use super::anomaly_detection::StorageAnomaly;
use super::disaster_recovery::DisasterRecoveryPlan;

/// Advanced storage management capabilities - ZERO-COST NATIVE ASYNC
pub trait AdvancedStorageManagement: Send + Sync {
    /// Intelligent storage optimization with machine learning insights
    fn intelligent_optimize(&self) -> impl std::future::Future<Output = Result<IntelligentOptimizationReport>> + Send;
    /// Predictive analytics for storage planning
    fn predict_storage_needs(&self, forecast_days: u32) -> impl std::future::Future<Output = Result<StorageForecast>> + Send;

    /// Automated policy enforcement
    fn enforce_storage_policies(&self, policies: &[StoragePolicy]) -> impl std::future::Future<Output = Result<PolicyReport>> + Send;

    /// Real-time anomaly detection
    fn detect_anomalies(&self) -> impl std::future::Future<Output = Result<Vec<StorageAnomaly>>> + Send;

    /// Automated disaster recovery preparation
    fn prepare_disaster_recovery(&self) -> impl std::future::Future<Output = Result<DisasterRecoveryPlan>> + Send;
}

/// Intelligent optimization report with ML-driven insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligentOptimizationReport {
    pub report_id: String,
    pub timestamp: SystemTime,
    pub optimization_score: f64, // 0.0 to 1.0
    pub recommendations: Vec<IntelligentRecommendation>,
    pub predicted_improvements: PredictedImprovements,
    pub confidence_level: f64, // 0.0 to 1.0
}
/// ML-driven storage recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligentRecommendation {
    pub category: OptimizationCategory,
    pub priority: Priority,
    pub description: String,
    pub predicted_impact: PredictedImpact,
    pub implementation_complexity: ImplementationComplexity,
}
/// Predicted performance and cost improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedImprovements {
    pub performance_gain: f64, // Percentage improvement
    pub cost_reduction: f64,   // Percentage cost reduction
    pub efficiency_improvement: f64, // Overall efficiency gain
}
/// Optimization categories for intelligent recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Performance,
    Cost,
    Reliability,
    Security,
    Compliance,
    Capacity,
}
/// Priority levels for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}
/// Predicted impact of implementing a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedImpact {
    pub performance_impact: f64,
    pub cost_impact: f64,
    pub risk_reduction: f64,
    pub implementation_time: std::time::Duration,
}
/// Implementation complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationComplexity {
    Simple,    // < 1 hour
    Medium,    // 1-8 hours
    Complex,   // 1-5 days
    Advanced,  // > 1 week
} 