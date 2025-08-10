//! Optimization types for dataset optimization and performance improvements

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of optimization operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub successful_optimizations: u32,
    pub failed_optimizations: u32,
    pub optimized_datasets: Vec<String>,
    pub errors: Vec<String>,
}

/// Property change recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyChange {
    pub property_name: String,
    pub current_value: String,
    pub recommended_value: String,
    pub reason: String,
}

/// Optimization plan for distributed processing
#[derive(Debug, Clone)]
pub enum OptimizationPlan {
    /// Distribute optimization across multiple services
    Distributed {
        squirrel_tasks: HashMap<String, Vec<String>>,
    },
    /// Use single service for all optimizations
    SingleSquirrel { squirrel_id: String },
    /// Fall back to local processing only
    LocalOnly,
}

/// Tier threshold configuration for automated storage management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierThresholds {
    /// Hot tier threshold (access frequency per day)
    pub hot_threshold: f64,
    /// Warm tier threshold (access frequency per day)
    pub warm_threshold: f64,
    /// Cold tier threshold (access frequency per day)
    pub cold_threshold: f64,
    /// File size thresholds in bytes
    pub size_thresholds: SizeThresholds,
    /// Age thresholds in days
    pub age_thresholds: AgeThresholds,
}

/// Size-based thresholds for tier assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeThresholds {
    /// Small file threshold (bytes)
    pub small_file: u64,
    /// Large file threshold (bytes)
    pub large_file: u64,
}

/// Age-based thresholds for tier assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeThresholds {
    /// Recent file threshold (days)
    pub recent: u32,
    /// Old file threshold (days)
    pub old: u32,
}

impl Default for TierThresholds {
    fn default() -> Self {
        Self {
            hot_threshold: 10.0, // 10+ accesses per day
            warm_threshold: 1.0, // 1+ access per day
            cold_threshold: 0.1, // 0.1+ access per day
            size_thresholds: SizeThresholds::default(),
            age_thresholds: AgeThresholds::default(),
        }
    }
}

impl Default for SizeThresholds {
    fn default() -> Self {
        Self {
            small_file: {
                use nestgate_core::constants::storage::sizes;
                sizes::SMALL_FILE_BYTES
            },
            large_file: {
                use nestgate_core::constants::storage::sizes;
                sizes::LARGE_FILE_BYTES
            },
        }
    }
}

impl Default for AgeThresholds {
    fn default() -> Self {
        Self {
            recent: 7, // 7 days
            old: 90,   // 90 days
        }
    }
}

/// Performance expectation for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceExpectation {
    /// Expected IOPS (Input/Output Operations Per Second)
    pub expected_iops: u32,
    /// Expected bandwidth in MB/s
    pub expected_bandwidth_mbps: f64,
    /// Expected latency in milliseconds
    pub expected_latency_ms: f64,
    /// Expected availability percentage (0.0-100.0)
    pub expected_availability: f64,
    /// Expected durability (number of 9s, e.g., 11 for 99.999999999%)
    pub expected_durability_nines: u32,
}

impl Default for PerformanceExpectation {
    fn default() -> Self {
        Self {
            expected_iops: 1000,
            expected_bandwidth_mbps: 100.0,
            expected_latency_ms: 10.0,
            expected_availability: 99.9,
            expected_durability_nines: 11,
        }
    }
}
