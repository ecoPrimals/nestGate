// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **PERFORMANCE INSIGHTS**
//!
//! Types related to performance insights, recommendations, and severity levels.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **PERFORMANCE INSIGHT**
///
/// Represents a single performance insight with severity and recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceinsight
pub struct PerformanceInsight {
    /// Unique identifier for this insight
    pub id: String,
    /// Human-readable title of the insight
    pub title: String,
    /// Detailed description of the performance issue or observation
    pub description: String,
    /// Severity level of this insight
    pub severity: InsightSeverity,
    /// Type of insight
    pub insight_type: InsightType,
    /// Category of performance this insight relates to
    pub category: String,
    /// Confidence score (0.0 to 1.0) in this insight's accuracy
    pub confidence: f64,
    /// List of recommended actions to address this insight
    pub recommendations: Vec<String>,
    /// Primary recommendation (for compatibility)
    pub recommendation: String,
    /// Timestamp when this insight was generated
    pub generated_at: SystemTime,
    /// Expected impact if recommendations are followed
    pub potential_impact: String,
    /// Estimated impact as a numeric value (percentage improvement)
    pub estimated_impact: f64,
    /// Additional metadata for this insight
    pub metadata: HashMap<String, String>,
}

/// **INSIGHT SEVERITY**
///
/// Severity levels for performance insights and recommendations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Insightseverity
pub enum InsightSeverity {
    /// Informational severity level
    Info,
    /// Low priority, informational insights
    Low,
    /// Medium priority, should be addressed soon
    Medium,
    /// Warning level requiring attention
    Warning,
    /// High priority, requires immediate attention
    High,
    /// Critical priority, system performance at risk
    Critical,
}

/// **INSIGHT TYPE**
///
/// Categories of performance insights that can be generated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of Insight
pub enum InsightType {
    /// Optimization opportunity
    Optimization,
    /// Performance degradation detected
    Degradation,
    /// Resource utilization insight
    ResourceUtilization,
    /// Capacity planning insight
    CapacityPlanning,
    /// Anomaly detection result
    Anomaly,
    /// Trend analysis result
    Trend,
    /// Predictive insight
    Predictive,
    /// Configuration recommendation
    Configuration,
}
