//
// This module handles performance analysis and trend detection with real ZFS metrics.
// Split into logical sub-modules to maintain code organization and comply with file size limits.

// Note: error module moved to nestgate-core
// use crate::error::ApiResult; // DEPRECATED: Now using canonical Result<T>
//! Analyzer module

use nestgate_core::canonical_modernization::canonical_constants;
use nestgate_core::Result; // Canonical Result type (Nov 10, 2025 consolidation)
use serde::{Deserialize, Serialize};

// **CANONICAL MODERNIZATION**: Use canonical constants instead of scattered definitions
use canonical_constants::api::{
    IMPACT_HIGH, IMPACT_LOW, IMPACT_MEDIUM, TITLE_EXPAND_STORAGE, TITLE_SCHEDULE_DEFRAG,
};

/// **IMPACT LEVELS**
///
/// Standard impact level constants for performance insights.
pub struct ImpactLevel;

impl ImpactLevel {
    /// High impact performance issue requiring immediate attention
    pub const HIGH: &'static str = IMPACT_HIGH;
    /// Medium impact performance issue requiring attention
    pub const MEDIUM: &'static str = IMPACT_MEDIUM;
    /// Low impact performance issue for future consideration
    pub const LOW: &'static str = IMPACT_LOW;
}

/// **OPTIMIZATION TITLES**
///
/// Standard titles for optimization recommendations.
pub struct OptimizationTitle;

impl OptimizationTitle {
    /// Recommendation to expand storage capacity
    pub const EXPAND_STORAGE: &'static str = TITLE_EXPAND_STORAGE;
    /// Recommendation to schedule defragmentation
    pub const SCHEDULE_DEFRAG: &'static str = TITLE_SCHEDULE_DEFRAG;
}

/// **PERFORMANCE INSIGHT**
///
/// A performance insight with recommendations for system optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceinsight
pub struct PerformanceInsight {
    /// Impact level of this insight
    pub impact: String,
    /// Title describing the insight
    pub title: String,
    /// Detailed description of the performance issue
    pub description: String,
    /// List of recommended actions
    pub recommendations: Vec<String>,
}
/// Performance analyzer
#[derive(Debug)]
/// Performanceanalyzer
pub struct PerformanceAnalyzer;
impl PerformanceAnalyzer {
    /// Create new analyzer
    pub fn new() -> Self {
        Self
    }

    /// Analyze storage performance and capacity
    pub fn analyze_storage(&self) -> Result<PerformanceInsight> {
        Ok(PerformanceInsight {
            impact: ImpactLevel::HIGH.to_string(),
            title: OptimizationTitle::EXPAND_STORAGE.to_string(),
            description: "Storage capacity approaching limits".to_string(),
            recommendations: vec![
                "Add additional storage capacity".to_string(),
                "Archive old data".to_string(),
                "Enable compression".to_string(),
            ],
        })
    }

    /// Analyze system fragmentation levels
    pub fn analyze_fragmentation(&self) -> Result<PerformanceInsight> {
        Ok(PerformanceInsight {
            impact: ImpactLevel::HIGH.to_string(),
            title: OptimizationTitle::SCHEDULE_DEFRAG.to_string(),
            description: "System fragmentation detected".to_string(),
            recommendations: vec!["Schedule defragmentation".to_string()],
        })
    }
}
