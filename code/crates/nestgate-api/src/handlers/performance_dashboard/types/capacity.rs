// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CAPACITY ANALYSIS TYPES**
//!
//! Types for system capacity analysis and planning.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **CAPACITY ANALYSIS**
///
/// Analysis of system capacity and resource utilization.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capacityanalysis
pub struct CapacityAnalysis {
    /// Total system capacity in bytes
    pub total_capacity: u64,
    /// Used capacity in bytes
    pub used_capacity: u64,
    /// Available capacity in bytes
    pub available_capacity: u64,
    /// Utilization percentage
    pub utilization_percentage: f64,
    /// Current usage percentage
    pub current_usage_percentage: f64,
    /// Daily growth rate in bytes
    pub daily_growth_rate: f64,
    /// Growth rate per day (alias for `daily_growth_rate`)
    pub growth_rate_per_day: f64,
    /// Weekly growth rate in bytes
    pub weekly_growth_rate: f64,
    /// Monthly growth rate in bytes
    pub monthly_growth_rate: f64,
    /// Growth trend coefficient
    pub growth_trend: f64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Deduplication ratio
    pub deduplication_ratio: f64,
    /// Projected date when storage will be full
    pub projected_full_date: Option<SystemTime>,
    /// Overall capacity utilization percentage (legacy field)
    pub overall_utilization_percent: f64,
    /// Detailed capacity analysis by resource type
    pub resource_analysis: HashMap<String, ResourceCapacity>,
    /// Pool-specific capacity details
    pub pool_details: Vec<PoolCapacityDetail>,
    /// Capacity recommendations
    pub recommendations: Vec<CapacityRecommendation>,
    /// Time until capacity exhaustion (in days)
    pub days_until_exhaustion: Option<f64>,
    /// Days until storage is full
    pub days_until_full: Option<u32>,
}

/// **POOL CAPACITY DETAIL**
///
/// Detailed capacity information for storage pools.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolcapacitydetail
pub struct PoolCapacityDetail {
    /// Pool name
    pub pool_name: String,
    /// Total capacity in bytes
    pub total_capacity_bytes: u64,
    /// Used capacity in bytes
    pub used_capacity_bytes: u64,
    /// Available capacity in bytes
    pub available_capacity_bytes: u64,
    /// Utilization percentage
    pub utilization_percent: f64,
    /// Health status
    pub health_status: PoolHealthStatus,
}

/// **RESOURCE CAPACITY**
///
/// Capacity information for a specific resource type.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcecapacity
pub struct ResourceCapacity {
    /// Resource type (CPU, Memory, Disk, etc.)
    pub resource_type: String,
    /// Total capacity
    pub total_capacity: u64,
    /// Used capacity
    pub used_capacity: u64,
    /// Available capacity
    pub available_capacity: u64,
    /// Utilization percentage
    pub utilization_percent: f64,
    /// Growth rate per day
    pub growth_rate_per_day: f64,
}

/// **CAPACITY RECOMMENDATION**
///
/// Recommendation for capacity planning and optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capacityrecommendation
pub struct CapacityRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Recommendation description
    pub description: String,
    /// Expected impact
    pub expected_impact: String,
    /// Implementation timeline
    pub timeline: String,
}

/// **POOL HEALTH STATUS**
///
/// Health status of storage pools.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for `PoolHealth`
pub enum PoolHealthStatus {
    /// Pool is healthy
    Healthy,
    /// Pool has warnings
    Warning,
    /// Pool is in critical state
    Critical,
    /// Pool status is unknown
    Unknown,
}

/// **RECOMMENDATION TYPE**
///
/// Type of capacity recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Recommendation
pub enum RecommendationType {
    /// Scale up resources
    ScaleUp,
    /// Scale down resources
    ScaleDown,
    /// Optimize existing resources
    Optimize,
    /// Add new resources
    AddResources,
    /// Archive old data
    ArchiveData,
}

/// **RECOMMENDATION PRIORITY**
///
/// Priority level for recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Recommendationpriority
pub enum RecommendationPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}
