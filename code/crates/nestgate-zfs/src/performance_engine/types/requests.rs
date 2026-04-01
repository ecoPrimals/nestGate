// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::alerts::{AlertSeverity, PerformanceAlert};
use super::context::{
    SystemCapabilities, WorkloadPattern, ZfsConfigurationContext, ZfsExpertiseContext,
};
use super::metrics::ZfsPerformanceMetrics;

/// Performance optimization request
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceOptimizationRequest {
    /// Name of the ZFS pool to optimize
    pub pool_name: String,
    /// Optional dataset name for specific dataset optimization
    pub dataset_name: Option<String>,
    /// Current optimization strategy being used
    pub optimization_strategy: EcosystemOptimizationStrategy,
    /// Current performance metrics
    pub current_metrics: ZfsPerformanceMetrics,
    /// ZFS configuration context
    pub configuration_context: ZfsConfigurationContext,
}
/// Ecosystem optimization strategy
#[derive(Debug, Serialize, Deserialize)]
pub enum EcosystemOptimizationStrategy {
    /// Optimize for minimum latency
    LatencyOptimization,
    /// Optimize for maximum throughput
    ThroughputOptimization,
    /// Balance latency and throughput
    BalancedOptimization,
    /// Custom optimization strategy with specified name
    CustomStrategy(String),
}
/// AI optimization recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct AiOptimizationRecommendation {
    /// Recommended optimization strategy
    pub strategy: EcosystemOptimizationStrategy,
    /// Confidence score for the recommendation (0.0-1.0)
    pub confidence_score: f64,
    /// Expected performance improvement percentage
    pub expected_improvement: f64,
    /// Recommended parameter values
    pub parameter_recommendations: HashMap<String, String>,
    /// Risk assessment description
    pub risk_assessment: String,
}
/// ZFS tuning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsTuningRequest {
    /// Name of the ZFS pool
    pub pool_name: String,
    /// Optional dataset name
    pub dataset_name: Option<String>,
    /// Detected workload pattern
    pub workload_pattern: WorkloadPattern,
    /// Current ZFS configuration parameters
    pub current_configuration: HashMap<String, String>,
    /// Performance goals for tuning
    pub performance_goals: Vec<String>,
    /// System hardware capabilities
    pub system_capabilities: SystemCapabilities,
    /// ZFS configuration context
    pub configuration_context: ZfsConfigurationContext,
    /// Expertise level context for tuning
    pub expertise_context: ZfsExpertiseContext,
}
/// Ecosystem tuning recommendations
#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemTuningRecommendations {
    /// List of tuning recommendations
    pub recommendations: Vec<ZfsTuningRecommendation>,
    /// Overall confidence score (0.0-1.0)
    pub overall_confidence: f64,
    /// Estimated performance improvement percentage
    pub estimated_improvement: f64,
}
/// ZFS tuning recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct ZfsTuningRecommendation {
    /// ZFS parameter to tune
    pub parameter: String,
    /// Recommended value for the parameter
    pub recommendedvalue: String,
    /// Confidence level in this recommendation (0.0-1.0)
    pub confidence: f64,
    /// Expected performance impact percentage
    pub expected_impact: f64,
}
/// Performance alert analysis request
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAlertAnalysisRequest {
    /// The performance alert being analyzed
    pub alert: PerformanceAlert,
    /// Historical performance metrics for trend analysis
    pub historical_metrics: Vec<ZfsPerformanceMetrics>,
    /// Current ZFS configuration parameters
    pub current_configuration: HashMap<String, String>,
    /// System hardware and resource capabilities
    pub system_capabilities: SystemCapabilities,
}
/// Ecosystem alert analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemAlertAnalysis {
    /// Root cause analysis description
    pub root_cause_analysis: String,
    /// Recommended remediation actions
    pub recommended_actions: Vec<String>,
    /// Confidence score for the analysis (0.0 to 1.0)
    pub confidence_score: f64,
    /// Alert urgency level classification
    pub urgency_level: AlertSeverity,
}
