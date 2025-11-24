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
    pub currentvalue: String,
    pub recommendedvalue: String,
    pub reason: String,
}
/// Optimization plan for distributed processing
#[derive(Debug, Clone)]
pub enum OptimizationPlan {
    /// Distribute optimization across multiple services
    Distributed {
        intelligence_tasks: HashMap<String, Vec<String>>,
    },
    /// Use single service for all optimizations
    SingleIntelligence { intelligence_id: String },
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
                use nestgate_core::canonical_modernization::canonical_constants::storage::SMALL_FILE_BYTES;
                SMALL_FILE_BYTES
            },
            large_file: {
                use nestgate_core::canonical_modernization::canonical_constants::storage::LARGE_FILE_BYTES;
                LARGE_FILE_BYTES
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_result_default() {
        let result = OptimizationResult::default();
        assert_eq!(result.successful_optimizations, 0);
        assert_eq!(result.failed_optimizations, 0);
        assert!(result.optimized_datasets.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_optimization_result_with_data() {
        let result = OptimizationResult {
            successful_optimizations: 5,
            failed_optimizations: 2,
            optimized_datasets: vec!["dataset1".to_string(), "dataset2".to_string()],
            errors: vec!["Error 1".to_string()],
        };

        assert_eq!(result.successful_optimizations, 5);
        assert_eq!(result.failed_optimizations, 2);
        assert_eq!(result.optimized_datasets.len(), 2);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_property_change_creation() {
        let change = PropertyChange {
            property_name: "compression".to_string(),
            currentvalue: "off".to_string(),
            recommendedvalue: "lz4".to_string(),
            reason: "Improve compression ratio".to_string(),
        };

        assert_eq!(change.property_name, "compression");
        assert_eq!(change.currentvalue, "off");
        assert_eq!(change.recommendedvalue, "lz4");
        assert!(change.reason.contains("compression"));
    }

    #[test]
    fn test_tier_thresholds_default() {
        let thresholds = TierThresholds::default();
        assert_eq!(thresholds.hot_threshold, 10.0);
        assert_eq!(thresholds.warm_threshold, 1.0);
        assert_eq!(thresholds.cold_threshold, 0.1);
    }

    #[test]
    fn test_size_thresholds_default() {
        let thresholds = SizeThresholds::default();
        assert!(thresholds.small_file > 0);
        assert!(thresholds.large_file > thresholds.small_file);
    }

    #[test]
    fn test_age_thresholds_default() {
        let thresholds = AgeThresholds::default();
        assert_eq!(thresholds.recent, 7);
        assert_eq!(thresholds.old, 90);
        assert!(thresholds.old > thresholds.recent);
    }

    #[test]
    fn test_performance_expectation_default() {
        let perf = PerformanceExpectation::default();
        assert_eq!(perf.expected_iops, 1000);
        assert_eq!(perf.expected_bandwidth_mbps, 100.0);
        assert_eq!(perf.expected_latency_ms, 10.0);
        assert_eq!(perf.expected_availability, 99.9);
        assert_eq!(perf.expected_durability_nines, 11);
    }

    #[test]
    fn test_performance_expectation_custom() {
        let perf = PerformanceExpectation {
            expected_iops: 5000,
            expected_bandwidth_mbps: 500.0,
            expected_latency_ms: 5.0,
            expected_availability: 99.99,
            expected_durability_nines: 12,
        };

        assert_eq!(perf.expected_iops, 5000);
        assert_eq!(perf.expected_bandwidth_mbps, 500.0);
        assert_eq!(perf.expected_latency_ms, 5.0);
        assert_eq!(perf.expected_availability, 99.99);
    }

    #[test]
    fn test_optimization_plan_distributed() {
        let mut tasks = HashMap::new();
        tasks.insert(
            "intel-001".to_string(),
            vec!["task1".to_string(), "task2".to_string()],
        );

        let plan = OptimizationPlan::Distributed {
            intelligence_tasks: tasks,
        };

        match plan {
            OptimizationPlan::Distributed { intelligence_tasks } => {
                assert_eq!(intelligence_tasks.len(), 1);
            }
            _ => panic!("Expected Distributed plan"),
        }
    }

    #[test]
    fn test_optimization_plan_single_intelligence() {
        let plan = OptimizationPlan::SingleIntelligence {
            intelligence_id: "intel-001".to_string(),
        };

        match plan {
            OptimizationPlan::SingleIntelligence { intelligence_id } => {
                assert_eq!(intelligence_id, "intel-001");
            }
            _ => panic!("Expected SingleIntelligence plan"),
        }
    }

    #[test]
    fn test_optimization_plan_local_only() {
        let plan = OptimizationPlan::LocalOnly;

        match plan {
            OptimizationPlan::LocalOnly => {}
            _ => panic!("Expected LocalOnly plan"),
        }
    }

    #[test]
    fn test_tier_thresholds_serialization() {
        let thresholds = TierThresholds::default();
        let json = serde_json::to_string(&thresholds).expect("Failed to serialize");
        let deserialized: TierThresholds =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(thresholds.hot_threshold, deserialized.hot_threshold);
        assert_eq!(thresholds.warm_threshold, deserialized.warm_threshold);
    }

    #[test]
    fn test_property_change_serialization() {
        let change = PropertyChange {
            property_name: "compression".to_string(),
            currentvalue: "off".to_string(),
            recommendedvalue: "lz4".to_string(),
            reason: "Better performance".to_string(),
        };

        let json = serde_json::to_string(&change).expect("Failed to serialize");
        let deserialized: PropertyChange =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(change.property_name, deserialized.property_name);
        assert_eq!(change.recommendedvalue, deserialized.recommendedvalue);
    }
}
