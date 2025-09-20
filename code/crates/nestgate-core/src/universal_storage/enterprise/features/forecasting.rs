// **STORAGE FORECASTING FEATURES**
//! Forecasting functionality and utilities.
// Storage capacity and performance forecasting capabilities.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Storage capacity and performance forecasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageForecast {
    pub forecast_id: String,
    pub forecast_period_days: u32,
    pub capacity_projection: CapacityProjection,
    pub performance_projection: PerformanceProjection,
    pub cost_projection: CostProjection,
    pub confidence_interval: (f64, f64),
}
/// Capacity growth projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityProjection {
    pub current_usage_gb: f64,
    pub projected_usage_gb: f64,
    pub growth_rate_percent: f64,
    pub days_to_full: Option<u32>,
}
/// Performance trend projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProjection {
    pub current_iops: f64,
    pub projected_iops: f64,
    pub current_latency_ms: f64,
    pub projected_latency_ms: f64,
}
/// Cost projection analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostProjection {
    pub current_monthly_cost: f64,
    pub projected_monthly_cost: f64,
    pub cost_drivers: Vec<String>,
    pub optimization_opportunities: Vec<String>,
} 