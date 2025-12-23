// **STORAGE FORECASTING FEATURES**
//! Forecasting functionality and utilities.
// Storage capacity and performance forecasting capabilities.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Storage capacity and performance forecasting
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageforecast
pub struct StorageForecast {
    /// Forecast identifier
    pub forecast_id: String,
    /// Forecast Period Days
    pub forecast_period_days: u32,
    /// Capacity Projection
    pub capacity_projection: CapacityProjection,
    /// Performance Projection
    pub performance_projection: PerformanceProjection,
    /// Cost Projection
    pub cost_projection: CostProjection,
    /// Confidence Interval
    pub confidence_interval: (f64, f64),
}
/// Capacity growth projection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capacityprojection
pub struct CapacityProjection {
    /// Current Usage in gigabytes
    pub current_usage_gb: f64,
    /// Projected Usage in gigabytes
    pub projected_usage_gb: f64,
    /// Growth Rate Percent
    pub growth_rate_percent: f64,
    /// Days To Full
    pub days_to_full: Option<u32>,
}
/// Performance trend projection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceprojection
pub struct PerformanceProjection {
    /// Current Iops
    pub current_iops: f64,
    /// Projected Iops
    pub projected_iops: f64,
    /// Current Latency Ms
    pub current_latency_ms: f64,
    /// Projected Latency Ms
    pub projected_latency_ms: f64,
}
/// Cost projection analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Costprojection
pub struct CostProjection {
    /// Current Monthly Cost
    pub current_monthly_cost: f64,
    /// Projected Monthly Cost
    pub projected_monthly_cost: f64,
    /// Cost Drivers
    pub cost_drivers: Vec<String>,
    /// Optimization Opportunities
    pub optimization_opportunities: Vec<String>,
} 