//! **PERFORMANCE DASHBOARD TYPES**
//!
//! Comprehensive type definitions for the performance dashboard system,
//! organized into logical modules for maintainability.

pub mod analysis;
pub mod capacity;
pub mod forecasting;
pub mod insights;
pub mod metrics;

// Re-export commonly used types
pub use analysis::{
    CachePerformanceAnalysis, IOPerformanceAnalysis, LatencyPercentiles, NetworkInterface,
    PerformanceAnalysisResult, PerformanceTrendAnalysis, PoolTrend, SystemResourceMetrics,
    ThroughputAnalysis,
};
pub use capacity::{CapacityAnalysis, PoolCapacityDetail};
pub use forecasting::{
    CapacityForecast, ConfidenceInterval, GrowthPoint, PerformanceForecast, PredictedMetricsPoint,
};
pub use insights::{InsightSeverity, InsightType, PerformanceInsight};
pub use metrics::{
    ComprehensiveMetricsPoint, PoolPerformanceTrends, RealTimeMetrics, SystemMetrics,
    SystemPerformanceSnapshot,
};

// Main dashboard types are defined in this module and exported directly

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// **ALERT TYPE**
///
/// Classification of alert types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// System-level alert
    System,
    /// Performance-related alert
    Performance,
    /// Security alert
    Security,
    /// Capacity alert
    Capacity,
    /// Network alert
    Network,
    /// Storage alert
    Storage,
}

/// **HEALTH SCORE**
///
/// System health score with breakdown by component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    /// Overall health score (0.0 to 100.0)
    pub overall: f64,
    /// CPU health component score
    pub cpu: f64,
    /// Memory health component score
    pub memory: f64,
    /// Storage health component score
    pub storage: f64,
    /// Network health component score
    pub network: f64,
}

/// **ALERT INFO**
///
/// Information about a system alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    /// Alert identifier
    pub id: String,
    /// Alert severity level
    pub severity: String,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: SystemTime,
    /// Alert source component
    pub source: String,
    /// Whether the alert is acknowledged
    pub acknowledged: bool,
}

/// **DASHBOARD OVERVIEW**
///
/// Complete dashboard overview containing all performance data and insights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOverview {
    /// Timestamp when this overview was generated
    pub timestamp: SystemTime,
    /// Time range for the data
    pub time_range: TimeRange,
    /// Current system metrics
    pub current_metrics: SystemPerformanceSnapshot,
    /// Performance analysis results
    pub performance_analysis: PerformanceTrendAnalysis,
    /// Optimization recommendations
    pub optimization_recommendations: Vec<String>,
    /// Performance insights
    pub insights: Vec<PerformanceInsight>,
    /// Overall health score (0.0 to 100.0)
    pub health_score: f64,
    /// Capacity forecast
    pub capacity_forecast: CapacityForecast,
    /// Alert summary
    pub alert_summary: AlertSummary,
}

/// **TIME RANGE**
///
/// Time range specification for dashboard queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time of the range
    pub start: SystemTime,
    /// End time of the range  
    pub end: SystemTime,
    /// Data granularity for aggregation
    pub granularity: std::time::Duration,
}

/// **ALERT SUMMARY**
///
/// Summary of system alerts by severity level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSummary {
    /// Critical alerts count
    pub critical_alerts: u32,
    /// Warning alerts count
    pub warning_alerts: u32,
    /// Info alerts count
    pub info_alerts: u32,
    /// Recent alerts list
    pub recent_alerts: Vec<DashboardAlert>,
}

/// **DASHBOARD ALERT**
///
/// Individual alert for dashboard display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAlert {
    /// Alert identifier
    pub id: String,
    /// Alert severity level
    pub severity: InsightSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: SystemTime,
    /// Alert source component
    pub source: String,
}

/// **COMPREHENSIVE PERFORMANCE DASHBOARD**
///
/// Main dashboard structure containing all performance analytics and insights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensivePerformanceDashboard {
    /// Timestamp when this dashboard was generated
    pub generated_at: SystemTime,
    /// Overall system health score (0.0 to 100.0)
    pub overall_health_score: f64,
    /// List of performance insights and recommendations
    pub insights: Vec<PerformanceInsight>,
    /// Current system performance metrics
    pub current_metrics: SystemPerformanceSnapshot,
    /// Historical performance trends and analysis
    pub trends: PerformanceTrendAnalysis,
    /// Detailed I/O performance analysis
    pub io_analysis: IOPerformanceAnalysis,
    /// ZFS cache performance analysis
    pub cache_analysis: CachePerformanceAnalysis,
    /// Performance forecasting and predictions
    pub forecast: PerformanceForecast,
    /// Risk assessment for system performance
    pub risk_assessment: RiskAssessment,
    /// Time series data for charts and graphs
    pub time_series_data: Vec<ComprehensiveMetricsPoint>,
    /// System capacity analysis and projections
    pub capacity_analysis: CapacityAnalysis,
}

/// **RISK ASSESSMENT**
///
/// Assessment of performance-related risks and their potential impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk level (0.0 to 1.0)
    pub overall_risk_level: f64,
    /// List of identified risks
    pub identified_risks: Vec<PerformanceRisk>,
    /// Risk mitigation recommendations
    pub mitigation_recommendations: Vec<String>,
}

/// **PERFORMANCE RISK**
///
/// Individual performance risk with severity and impact assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRisk {
    /// Risk identifier
    pub id: String,
    /// Risk description
    pub description: String,
    /// Risk severity (0.0 to 1.0)
    pub severity: f64,
    /// Probability of occurrence (0.0 to 1.0)
    pub probability: f64,
    /// Potential impact description
    pub impact: String,
    /// Recommended mitigation actions
    pub mitigation_actions: Vec<String>,
}
