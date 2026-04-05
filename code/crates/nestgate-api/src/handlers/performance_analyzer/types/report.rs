// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Aggregated reports and actionable recommendations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::trends::{ComponentAnalysis, PerformanceTrends};

/// Performance analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceanalysisreport
pub struct PerformanceAnalysisReport {
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Overall system health score (0-100)
    pub overall_health_score: f64,
    /// Performance trends
    pub trends: PerformanceTrends,
    /// Component analyses
    pub component_analyses: Vec<ComponentAnalysis>,
    /// Performance recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
    /// Critical issues detected
    pub critical_issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// System uptime in seconds
    pub system_uptime_seconds: u64,
    /// Analysis period start
    pub analysis_period_start: DateTime<Utc>,
    /// Analysis period end
    pub analysis_period_end: DateTime<Utc>,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancerecommendation
pub struct PerformanceRecommendation {
    /// Recommendation category
    pub category: String,
    /// Recommendation description
    pub description: String,
    /// Priority level (1-10, 10 being highest)
    pub priority: u8,
    /// Estimated impact
    pub estimated_impact: String,
}
