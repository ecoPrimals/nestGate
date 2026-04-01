// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **PERFORMANCE FORECASTING TYPES**
//!
//! Types for performance forecasting and predictions.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// **PERFORMANCE FORECAST**
///
/// Forecasting of future performance metrics and trends.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceforecast
pub struct PerformanceForecast {
    /// Forecast horizon in days
    pub forecast_horizon_days: u32,
    /// Predicted performance trend
    pub predicted_performance_trend: f64,
    /// Predicted metrics data points
    pub predicted_metrics: Vec<PredictedMetricsPoint>,
    /// Confidence intervals for predictions
    pub confidence_intervals: Vec<ConfidenceInterval>,
    /// Risk factors affecting performance
    pub risk_factors: Vec<String>,
    /// Potential issues that may arise
    pub potential_issues: Vec<String>,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Capacity forecast details
    pub capacity_forecast: CapacityForecast,
}

/// **PREDICTED METRICS POINT**
///
/// Individual predicted performance metrics data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Predictedmetricspoint
pub struct PredictedMetricsPoint {
    /// Timestamp for this prediction
    pub timestamp: SystemTime,
    /// Predicted CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Predicted memory usage percentage
    pub memory_usage_percent: f64,
    /// Predicted disk usage percentage
    pub disk_usage_percent: f64,
    /// Predicted network throughput in bytes/second
    pub network_throughput_bps: u64,
    /// Predicted response time in milliseconds
    pub response_time_ms: f64,
}

/// **CONFIDENCE INTERVAL**
///
/// Confidence interval for predictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Confidenceinterval
pub struct ConfidenceInterval {
    /// Metric name
    pub metric: String,
    /// Lower bound
    pub lower_bound: f64,
    /// Upper bound
    pub upper_bound: f64,
    /// Confidence level (0.0 to 1.0)
    pub confidence_level: f64,
}

/// **CAPACITY FORECAST**
///
/// Forecast of capacity utilization.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capacityforecast
pub struct CapacityForecast {
    /// Current usage percentage
    pub current_usage_percentage: f64,
    /// Projected usage in 30 days
    pub projected_usage_in_30_days: f64,
    /// Projected usage in 90 days
    pub projected_usage_in_90_days: f64,
    /// Growth data points
    pub growth_points: Vec<GrowthPoint>,
    /// Capacity recommendations
    pub recommendations: Vec<String>,
}

/// **GROWTH POINT**
///
/// Data point showing capacity growth over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Growthpoint
pub struct GrowthPoint {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Usage percentage at this point
    pub usage_percentage: f64,
}

/// **FORECAST METHODOLOGY**
///
/// The methodology used for performance forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Forecastmethodology
pub enum ForecastMethodology {
    /// Linear regression based forecasting
    LinearRegression,
    /// Moving average based forecasting
    MovingAverage,
    /// Exponential smoothing
    ExponentialSmoothing,
    /// Machine learning based prediction
    MachineLearning,
    /// Historical pattern analysis
    HistoricalPattern,
}
