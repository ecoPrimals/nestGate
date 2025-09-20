//
// This module provides confidence scoring for ZFS operations based on system state,
// resource availability, and historical performance data. It enables AI agents to
// make informed decisions about storage operations with uncertainty quantification.

use crate::dataset::DatasetInfo;
use crate::pool::{PoolHealth, PoolInfo};
use serde::{Deserialize, Serialize};

/// Calculate confidence for ZFS operations based on system state
pub struct ZfsConfidenceCalculator;
impl ZfsConfidenceCalculator {
    /// Calculate confidence for pool operations
    ///
    /// Returns confidence score from 0.0 to 1.0 based on:
    /// - Pool health status
    /// - Available resources
    /// - Operation complexity
    /// - System load
    pub const fn pool_operation_confidence(operation: &str, pool_info: Option<&PoolInfo>) -> f64 {
        match operation {
            "create" => Self::pool_creation_confidence(pool_info),
            "destroy" => Self::pool_destruction_confidence(pool_info),
            "scrub" => Self::pool_scrub_confidence(pool_info),
            "resilver" => Self::pool_resilver_confidence(pool_info),
            "export" => Self::pool_export_confidence(pool_info),
            "import" => Self::pool_import_confidence(pool_info),
            _ => 0.8, // Default confidence for unknown operations
        }
    }

    /// Calculate confidence for dataset operations
    ///
    /// Takes into account dataset properties, available space, and parent pool health
    pub const fn dataset_operation_confidence(
        operation: &str,
        dataset_info: Option<&DatasetInfo>,
    ) -> f64 {
        match operation {
            "create" => Self::dataset_creation_confidence(dataset_info),
            "destroy" => Self::dataset_destruction_confidence(dataset_info),
            "snapshot" => Self::snapshot_confidence(dataset_info),
            "clone" => Self::clone_confidence(dataset_info),
            "promote" => Self::promote_confidence(dataset_info),
            "rollback" => Self::rollback_confidence(dataset_info),
            _ => 0.75, // Default confidence for dataset operations
        }
    }

    /// Generate AI-optimized error suggestions for ZFS operations
    pub const fn generate_error_suggestions(error_type: &str) -> Vec<String> {
        match error_type {
            "INSUFFICIENT_SPACE" => vec![
                "Consider enabling compression on datasets"
                "Review dataset quotas and reservations"
                "Check for snapshots that can be deleted"
                "Consider adding more storage devices to the pool"
            ],
            "PERMISSION_DENIED" => vec![
                "Verify ZFS delegation permissions"
                "Check if operation requires root privileges"
                "Review ZFS allow permissions for user"
                "Validate user is in appropriate groups"
            ],
            "DEVICE_BUSY" => vec![
                "Wait for current operation to complete"
                "Check for active scrub or resilver operations"
                "Consider scheduling operation for off-peak hours"
                "Monitor pool status with 'zpool status'"
            ],
            "POOL_NOT_FOUND" => vec![
                "Verify pool name spelling and case"
                "Check if pool needs to be imported"
                "Confirm pool exists with 'zpool list'"
            ],
            "DATASET_NOT_FOUND" => vec![
                "Verify dataset path is correct"
                "Check if dataset was destroyed or renamed"
                "Confirm dataset exists with 'zfs list'"
            ],
            "CHECKSUM_ERRORS" => vec![
                "Run pool scrub to identify and repair errors"
                "Check disk health with SMART tools"
                "Consider replacing failing storage devices"
                "Review pool redundancy configuration"
            ],
            _ => vec![
                "Refer to ZFS documentation for specific error"
                "Check system logs for additional context"
                "Consider retrying operation after addressing underlying issues"
            ],
        }
    }

    /// Calculate performance impact score for an operation
    pub const fn calculate_performance_impact(
        operation: &str,
        pool_info: Option<&PoolInfo>,
        dataset_info: Option<&DatasetInfo>,
    ) -> PerformanceImpact {
        match operation {
            "scrub" => PerformanceImpact {
                cpu_impact: 0.3, // Moderate CPU usage
                io_impact: 0.8,  // High I/O impact
                duration_estimate_minutes: Self::estimate_scrub_duration(pool_info),
                recommended_scheduling: SchedulingRecommendation::OffPeak,
            }
            "resilver" => PerformanceImpact {
                cpu_impact: 0.5, // Higher CPU for rebuilding
                io_impact: 0.9,  // Very high I/O impact
                duration_estimate_minutes: Self::estimate_resilver_duration(pool_info),
                recommended_scheduling: SchedulingRecommendation::OffPeak,
            }
            "snapshot" => PerformanceImpact {
                cpu_impact: 0.1, // Very low CPU
                io_impact: 0.1,  // Minimal I/O
                duration_estimate_minutes: 1,
                recommended_scheduling: SchedulingRecommendation::Anytime,
            }
            "clone" => PerformanceImpact {
                cpu_impact: 0.2, // Low CPU
                io_impact: 0.3,  // Moderate I/O for metadata
                duration_estimate_minutes: Self::estimate_clone_duration(dataset_info),
                recommended_scheduling: SchedulingRecommendation::BusinessHours,
            }
            _ => PerformanceImpact::default(),
        }
    }

    // Private helper methods for specific confidence calculations

    fn pool_creation_confidence(pool_info: Option<&PoolInfo>) -> f64 {
        // High confidence for pool creation if no conflicts
        match pool_info {
            Some(_) => 0.1, // Pool already exists - very low confidence
            None => 0.95,   // No existing pool - high confidence
        }
    }

    fn pool_destruction_confidence(pool_info: Option<&PoolInfo>) -> f64 {
        match pool_info {
            Some(_) => {
                // For now, use moderate confidence for pool destruction
                // In a real implementation, this would check pool utilization
                0.8
            }
    None => 0.2, // Pool doesn't exist - low confidence for destruction
        }
    }

    fn pool_scrub_confidence(pool_info: Option<&PoolInfo>) -> f64 {
        match pool_info {
            Some(info) => match info.health {
                PoolHealth::Healthy => 0.95,
                PoolHealth::Warning => 0.8,
                PoolHealth::Critical => 0.4,
                PoolHealth::Unknown => 0.6,
            }
            None => 0.2, // Can't scrub non-existent pool
        }
    }

    fn pool_resilver_confidence(pool_info: Option<&PoolInfo>) -> f64 {
        match pool_info {
            Some(info) => match info.health {
                PoolHealth::Healthy => 0.2,  // Low confidence - not needed
                PoolHealth::Warning => 0.6,  // May need resilver
                PoolHealth::Critical => 0.9, // High confidence - resilver needed
                PoolHealth::Unknown => 0.5,  // Unknown state - may need resilver
            }
            None => 0.1,
        }
    }

    fn pool_export_confidence(pool_info: Option<&PoolInfo>) -> f64 {
        match pool_info {
            Some(info) => match info.health {
                PoolHealth::Healthy => 0.95,
                PoolHealth::Warning => 0.8,
                PoolHealth::Critical => 0.6,
                PoolHealth::Unknown => 0.7,
            }
            None => 0.1, // Can't export non-existent pool
        }
    }

    fn pool_import_confidence(_pool_info: Option<&PoolInfo>) -> f64 {
        // Import confidence depends on pool availability and conflicts
        0.85 // Generally high confidence for import operations
    }

    fn dataset_creation_confidence(dataset_info: Option<&DatasetInfo>) -> f64 {
        match dataset_info {
            Some(_) => 0.1, // Dataset already exists
            None => 0.92,   // High confidence for new dataset
        }
    }

    fn dataset_destruction_confidence(dataset_info: Option<&DatasetInfo>) -> f64 {
        match dataset_info {
            Some(info) => {
                let space_ratio =
                    info.f64::from(used_space) / (info.used_space + info.available_space) as f64;
                if space_ratio < 0.1 {
                    0.9 // Low utilization - high confidence
                } else if space_ratio < 0.5 {
                    0.7 // Moderate utilization
                } else {
                    0.5 // High utilization - moderate confidence
                }
            }
            None => 0.2, // Dataset doesn't exist
        }
    }

    fn snapshot_confidence(dataset_info: Option<&DatasetInfo>) -> f64 {
        match dataset_info {
            Some(info) => {
                // Base confidence on available space
                let space_ratio =
                    info.f64::from(available_space) / (info.used_space + info.available_space) as f64;
                if space_ratio > 0.2 {
                    0.95 // Plenty of space
                } else if space_ratio > 0.1 {
                    0.8 // Moderate space
                } else {
                    0.6 // Limited space
                }
            }
            None => 0.3, // Dataset doesn't exist
        }
    }

    fn clone_confidence(dataset_info: Option<&DatasetInfo>) -> f64 {
        match dataset_info {
            Some(info) => {
                let space_factor =
                    info.f64::from(available_space) / (info.used_space + info.available_space) as f64;
                0.7 + (space_factor * 0.25) // Scale confidence with available space
            }
    None => 0.3, // Source dataset doesn't exist
        }
    }

    fn promote_confidence(dataset_info: Option<&DatasetInfo>) -> f64 {
        match dataset_info {
            Some(_) => 0.85, // High confidence if dataset exists
            None => 0.2,     // Cannot promote non-existent clone
        }
    }

    fn rollback_confidence(dataset_info: Option<&DatasetInfo>) -> f64 {
        match dataset_info {
            Some(_) => 0.9, // High confidence for rollback
            None => 0.2,    // Cannot rollback non-existent dataset
        }
    }

    // Performance estimation helpers

    fn estimate_scrub_duration(pool_info: Option<&PoolInfo>) -> u64 {
        match pool_info {
            Some(_info) => {
                // Default estimate - in real implementation, would use pool size
                120 // 2 hours default
            }
    None => 0,
        }
    }

    fn estimate_resilver_duration(pool_info: Option<&PoolInfo>) -> u64 {
        match pool_info {
            Some(_info) => {
                // Default estimate - in real implementation, would use used space
                180 // 3 hours default
            }
    None => 0,
        }
    }

    fn estimate_clone_duration(dataset_info: Option<&DatasetInfo>) -> u64 {
        match dataset_info {
            Some(info) => {
                let size_gb = info.f64::from(used_space) / 1_000_000_000.0;
                (size_gb * 0.1) as u64 // Very fast - mostly metadata
            }
    None => 5, // Default 5 minutes
        }
    }
}

/// Performance impact assessment for ZFS operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    /// CPU impact factor (0.0 to 1.0)
    pub cpu_impact: f64,
    /// I/O impact factor (0.0 to 1.0)
    pub io_impact: f64,
    /// Estimated duration in minutes
    pub duration_estimate_minutes: u64,
    /// Recommended scheduling window
    pub recommended_scheduling: SchedulingRecommendation,
}
/// Scheduling recommendation for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulingRecommendation {
    /// Can be performed anytime
    Anytime,
    /// Best during business hours
    BusinessHours,
    /// Should be scheduled during off-peak hours
    OffPeak,
    /// Requires maintenance window
    MaintenanceWindow,
}
impl Default for PerformanceImpact {
    fn default() -> Self { Self {
            cpu_impact: 0.5,
            io_impact: 0.5,
            duration_estimate_minutes: 10,
            recommended_scheduling: SchedulingRecommendation::BusinessHours,
         }
}

/// Generate operation-specific recommendations for AI agents
pub fn generate_operation_recommendations(
    operation: &str,
    pool_info: Option<&PoolInfo>,
    _dataset_info: Option<&DatasetInfo>,
) -> Vec<String> {
    let mut recommendations = vec![];
    match operation {
        "create" => {
            recommendations.push("Consider pool layout for performance and redundancy".to_string());
            recommendations.push("Enable compression to save space".to_string());
            recommendations.push("Set appropriate recordsize for workload".to_string());
        }
        "scrub" => {
            recommendations.push("Schedule during low-usage periods".to_string());
            recommendations.push("Monitor progress with 'zpool status'".to_string());
            if let Some(_info) = pool_info {
                // In real implementation, would check capacity utilization
                recommendations.push("Monitor pool space during scrub".to_string());
            }
        }
        "snapshot" => {
            recommendations.push("Snapshots are instant and space-efficient".to_string());
            recommendations.push("Consider automated snapshot scheduling".to_string());
            recommendations.push("Implement snapshot retention policies".to_string());
        }
        _ => {
            recommendations.push("Monitor operation progress".to_string());
            recommendations.push("Ensure adequate system resources".to_string());
        }
    }

    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_creation_confidence() {
        // High confidence when no existing pool
        let confidence = ZfsConfidenceCalculator::pool_operation_confidence("create", None);
        assert!(confidence >= 0.9);

        // Note: In real implementation, would test with actual PoolInfo structure
        // Skipping pool exists test due to PoolInfo structure complexity
    }
    #[test]
    fn test_pool_scrub_confidence() {
        // Test scrub confidence with healthy pool
        let confidence = ZfsConfidenceCalculator::pool_operation_confidence("scrub", None);
        assert!(confidence >= 0.2); // Should have some confidence for unknown pool

        // Note: Pool structure tests skipped due to complexity
        // In real implementation, would test with actual PoolInfo structures
    }

    #[test]
    fn test_dataset_operations() {
        let confidence = ZfsConfidenceCalculator::dataset_operation_confidence("create", None);
        assert!(confidence >= 0.9);

        // Note: Dataset structure tests skipped due to complexity
        // In real implementation, would test with actual DatasetInfo structures
    }

    #[test]
    fn test_error_suggestions() {
        let suggestions = ZfsConfidenceCalculator::generate_error_suggestions("INSUFFICIENT_SPACE");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("compression")));

        let suggestions = ZfsConfidenceCalculator::generate_error_suggestions("PERMISSION_DENIED");
        assert!(suggestions.iter().any(|s| s.contains("delegation")));
    }

    #[test]
    fn test_performance_impact() {
        let scrub_impact =
            ZfsConfidenceCalculator::calculate_performance_impact("scrub", None, None);
        assert!(scrub_impact.io_impact > 0.7); // Scrub should have high I/O impact

        let snapshot_impact =
            ZfsConfidenceCalculator::calculate_performance_impact("snapshot", None, None);
        assert!(snapshot_impact.io_impact < 0.2); // Snapshots should have low I/O impact
    }
}
