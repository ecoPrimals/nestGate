//! Performance Dashboard Types
//! 
//! This module contains all the data structures and enums used by the performance dashboard.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub update_interval_seconds: u64,
    pub retention_days: u32,
    pub enable_predictive_analytics: bool,
    pub max_historical_points: usize,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            update_interval_seconds: 10,
            retention_days: 30,
            enable_predictive_analytics: true,
            max_historical_points: 10000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOverview {
    pub timestamp: SystemTime,
    pub time_range: TimeRange,
    pub current_metrics: RealTimeMetrics,
    pub performance_analysis: PerformanceAnalysisResult,
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    pub insights: Vec<PerformanceInsight>,
    pub health_score: HealthScore,
    pub capacity_forecast: CapacityForecast,
    pub alert_summary: AlertSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: SystemTime,
    pub end: SystemTime,
}

impl TimeRange {
    pub fn last_hour() -> Self {
        let now = SystemTime::now();
        Self {
            start: now - Duration::from_secs(3600),
            end: now,
        }
    }

    pub fn last_day() -> Self {
        let now = SystemTime::now();
        Self {
            start: now - Duration::from_secs(86400),
            end: now,
        }
    }

    pub fn last_week() -> Self {
        let now = SystemTime::now();
        Self {
            start: now - Duration::from_secs(604800),
            end: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    pub timestamp: SystemTime,
    pub pool_metrics: Vec<PoolMetrics>,
    pub system_metrics: SystemMetrics,
    pub arc_hit_ratio: f64,
    pub l2arc_hit_ratio: f64,
    pub compression_ratio: f64,
    pub total_throughput: f64,
    pub average_read_latency: f64,
    pub average_write_latency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub name: String,
    pub health_status: String,
    pub utilization_percentage: f64,
    pub total_capacity: u64,
    pub used_space: u64,
    pub available_space: u64,
    pub read_iops: u64,
    pub write_iops: u64,
    pub read_throughput: f64,
    pub write_throughput: f64,
    pub fragmentation_level: f64,
    pub error_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub memory_total: u64,
    pub memory_available: u64,
    pub network_io: NetworkIOMetrics,
    pub disk_io: DiskIOMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIOMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIOMetrics {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_operations: u64,
    pub write_operations: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsight {
    pub insight_type: InsightType,
    pub severity: InsightSeverity,
    pub title: String,
    pub description: String,
    pub recommendation: String,
    pub estimated_impact: f64, // Percentage improvement
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InsightType {
    Performance,
    Capacity,
    Caching,
    Maintenance,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum InsightSeverity {
    Info = 1,
    Medium = 2,
    Warning = 3,
    High = 4,
    Critical = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    pub overall_score: f64,
    pub health_status: HealthStatus,
    pub score_components: HashMap<String, f64>,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSummary {
    pub critical_alerts: usize,
    pub warning_alerts: usize,
    pub info_alerts: usize,
    pub recent_alerts: Vec<DashboardAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAlert {
    pub id: String,
    pub alert_type: AlertType,
    pub severity: InsightSeverity,
    pub title: String,
    pub description: String,
    pub timestamp: SystemTime,
    pub acknowledged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    Performance,
    Capacity,
    Health,
    Security,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardEvent {
    pub event_type: DashboardEventType,
    pub timestamp: SystemTime,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DashboardEventType {
    MetricsUpdate,
    AlertTriggered,
    OptimizationCompleted,
    HealthScoreUpdated,
    CapacityThresholdReached,
}

#[derive(Debug)]
pub struct DashboardState {
    pub critical_alerts: Vec<DashboardAlert>,
    pub warning_alerts: Vec<DashboardAlert>,
    pub info_alerts: Vec<DashboardAlert>,
    pub recent_alerts: Vec<DashboardAlert>,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            critical_alerts: Vec::new(),
            warning_alerts: Vec::new(),
            info_alerts: Vec::new(),
            recent_alerts: Vec::new(),
        }
    }
}

impl Default for DashboardState {
    fn default() -> Self {
        Self::new()
    }
}

// Additional types from later in the original file

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysisResult {
    pub pool_trends: Vec<PoolPerformanceTrends>,
    pub system_resources: SystemResourceMetrics,
    pub capacity_analysis: CapacityAnalysis,
    pub io_performance: IOPerformanceAnalysis,
    pub cache_performance: CachePerformanceAnalysis,
    pub forecast: PerformanceForecast,
    pub risk_assessment: RiskAssessment,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub impact_level: InsightSeverity,
    pub estimated_improvement: f64,
    pub implementation_complexity: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolPerformanceTrends {
    pub pool_name: String,
    pub iops_trend: Vec<DataPoint>,
    pub throughput_trend: Vec<DataPoint>,
    pub latency_trend: Vec<DataPoint>,
    pub utilization_trend: Vec<DataPoint>,
    pub health_trend: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: SystemTime,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResourceMetrics {
    pub cpu_usage_history: Vec<DataPoint>,
    pub memory_usage_history: Vec<DataPoint>,
    pub network_throughput_history: Vec<DataPoint>,
    pub disk_usage_history: Vec<DataPoint>,
    pub network_interfaces: Vec<NetworkInterfaceMetrics>,
    pub load_average: [f64; 3], // 1, 5, 15 minute averages
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceMetrics {
    pub interface_name: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub speed_mbps: u64,
    pub utilization_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityAnalysis {
    pub total_capacity: u64,
    pub used_capacity: u64,
    pub available_capacity: u64,
    pub growth_rate_per_day: f64,
    pub days_until_full: Option<u32>,
    pub pool_details: Vec<PoolCapacityDetail>,
    pub recommendations: Vec<CapacityRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacityDetail {
    pub pool_name: String,
    pub total_size: u64,
    pub used_size: u64,
    pub available_size: u64,
    pub growth_trend: Vec<DataPoint>,
    pub fragmentation_level: f64,
    pub compression_ratio: f64,
    pub deduplication_ratio: f64,
    pub projected_full_date: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOPerformanceAnalysis {
    pub average_read_latency: f64,
    pub average_write_latency: f64,
    pub peak_read_latency: f64,
    pub peak_write_latency: f64,
    pub latency_percentiles: LatencyPercentiles,
    pub throughput_analysis: ThroughputAnalysis,
    pub queue_depth_average: f64,
    pub io_size_distribution: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyPercentiles {
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub p99_9: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputAnalysis {
    pub peak_read_throughput: f64,
    pub peak_write_throughput: f64,
    pub average_read_throughput: f64,
    pub average_write_throughput: f64,
    pub throughput_patterns: Vec<ThroughputPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputPattern {
    pub pattern_type: String,
    pub frequency: String,
    pub impact_on_performance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceAnalysis {
    pub arc_hit_ratio: f64,
    pub l2arc_hit_ratio: f64,
    pub arc_size_current: u64,
    pub arc_size_target: u64,
    pub arc_components: HashMap<String, CacheComponentAnalysis>,
    pub optimization_opportunities: Vec<CacheOptimizationOpportunity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheComponentAnalysis {
    pub component_name: String,
    pub hit_ratio: f64,
    pub size_bytes: u64,
    pub efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOptimizationOpportunity {
    pub opportunity_type: String,
    pub description: String,
    pub potential_improvement: f64,
    pub implementation_effort: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceForecast {
    pub forecast_horizon_days: u32,
    pub predicted_metrics: Vec<PredictedMetrics>,
    pub confidence_intervals: Vec<ConfidenceInterval>,
    pub risk_factors: Vec<String>,
    pub capacity_forecast: CapacityForecast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedMetrics {
    pub timestamp: SystemTime,
    pub predicted_iops: f64,
    pub predicted_throughput: f64,
    pub predicted_latency: f64,
    pub predicted_capacity_usage: f64,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub metric_name: String,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_level: InsightSeverity,
    pub risk_factors: Vec<String>,
    pub mitigation_recommendations: Vec<String>,
    pub predicted_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityForecast {
    pub current_usage_percentage: f64,
    pub projected_usage_in_30_days: f64,
    pub projected_usage_in_90_days: f64,
    pub growth_points: Vec<CapacityGrowthPoint>,
    pub recommendations: Vec<CapacityRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityGrowthPoint {
    pub timestamp: SystemTime,
    pub projected_usage_percentage: f64,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityRecommendation {
    pub recommendation_type: String,
    pub description: String,
    pub urgency: InsightSeverity,
    pub estimated_cost: Option<f64>,
    pub timeline_days: u32,
}

// Metric point types for time series data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOMetricsPoint {
    pub timestamp: SystemTime,
    pub read_iops: f64,
    pub write_iops: f64,
    pub read_throughput: f64,
    pub write_throughput: f64,
    pub read_latency: f64,
    pub write_latency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetricsPoint {
    pub timestamp: SystemTime,
    pub arc_hit_ratio: f64,
    pub l2arc_hit_ratio: f64,
    pub arc_size: u64,
    pub l2arc_size: u64,
    pub metadata_hit_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveMetricsPoint {
    pub timestamp: SystemTime,
    pub io_metrics: IOMetricsPoint,
    pub cache_metrics: CacheMetricsPoint,
    pub capacity_metrics: CapacityMetricsPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityMetricsPoint {
    pub timestamp: SystemTime,
    pub total_capacity: u64,
    pub used_capacity: u64,
    pub compression_ratio: f64,
    pub deduplication_ratio: f64,
} 