// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// ZFS snapshot analysis and optimization recommendations

use crate::types::RetentionPolicy;
use nestgate_core::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Snapshot analytics without AI recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotanalytics
pub struct SnapshotAnalytics {
    /// Current snapshot count
    pub snapshot_count: u64,
    /// Storage usage by snapshots
    pub storage_usage: u64,
    /// Basic recommendations
    pub recommendations: Vec<String>,
}
impl SnapshotAnalytics {
    /// Analyze snapshot usage
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn analyze_snapshots(
        dataset: &str,
        snapshots: &[String],
        retention_policy: &RetentionPolicy,
    ) -> Result<Self> {
        debug!("Analyzing snapshots for dataset: {}", dataset);

        let snapshot_count = snapshots.len() as u64;
        let storage_usage = snapshot_count * 1024 * 1024 * 100; // Mock 100MB per snapshot
        let mut recommendations = Vec::new();

        // Basic snapshot analysis
        if snapshot_count > 50 {
            recommendations.push("Consider cleaning up old snapshots".to_string());
        }

        if retention_policy.keep_daily > 30 {
            recommendations.push("Daily snapshot retention is very high".to_string());
        }

        if storage_usage > 10 * 1024 * 1024 * 1024 {
            // 10GB
            recommendations.push("Snapshots are using significant storage".to_string());
        }
        Ok(Self {
            snapshot_count,
            storage_usage,
            recommendations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates  Test Retention Policy
    fn create_test_retention_policy(keep_daily: u32) -> RetentionPolicy {
        RetentionPolicy {
            name: "test_policy".to_string(),
            keep_hourly: 24,
            keep_daily,
            keep_weekly: 4,
            keep_monthly: 12,
        }
    }

    #[test]
    fn test_snapshot_analytics_creation() {
        let snapshots = vec!["snap1".to_string(), "snap2".to_string()];
        let policy = create_test_retention_policy(7);
        let analytics = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy)
            .expect("Test: analyze_snapshots should succeed");

        assert_eq!(analytics.snapshot_count, 2);
        assert!(analytics.storage_usage > 0);
    }

    #[test]
    fn test_many_snapshots_recommendation() {
        let snapshots: Vec<String> = (0..60).map(|i| format!("snap{}", i)).collect();
        let policy = create_test_retention_policy(7);
        let analytics = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy)
            .expect("Test: analyze_snapshots should succeed");

        assert!(analytics
            .recommendations
            .iter()
            .any(|r| r.contains("cleaning up")));
    }

    #[test]
    fn test_high_retention_recommendation() {
        let snapshots = vec!["snap1".to_string()];
        let policy = create_test_retention_policy(40);
        let analytics = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy)
            .expect("Test: analyze_snapshots should succeed");

        assert!(analytics
            .recommendations
            .iter()
            .any(|r| r.contains("retention is very high")));
    }

    #[test]
    fn test_high_storage_usage_recommendation() {
        let snapshots: Vec<String> = (0..150).map(|i| format!("snap{}", i)).collect();
        let policy = create_test_retention_policy(7);
        let analytics =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();

        assert!(analytics
            .recommendations
            .iter()
            .any(|r| r.contains("significant storage")));
    }

    #[test]
    fn test_optimal_snapshots_no_recommendations() {
        let snapshots = vec!["snap1".to_string(), "snap2".to_string()];
        let policy = create_test_retention_policy(7);
        let analytics =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();

        assert!(analytics.recommendations.is_empty());
    }

    #[test]
    fn test_empty_snapshots() {
        let snapshots: Vec<String> = vec![];
        let policy = create_test_retention_policy(7);
        let analytics =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();

        assert_eq!(analytics.snapshot_count, 0);
        assert_eq!(analytics.storage_usage, 0);
    }

    #[test]
    fn test_storage_calculation() {
        let snapshots = vec!["snap1".to_string()];
        let policy = create_test_retention_policy(7);
        let analytics =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();

        assert_eq!(analytics.storage_usage, 1024 * 1024 * 100); // 100MB per snapshot
    }

    #[test]
    fn test_analytics_clone() {
        let snapshots = vec!["snap1".to_string()];
        let policy = create_test_retention_policy(7);
        let analytics1 =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();
        let analytics2 = analytics1.clone();

        assert_eq!(analytics1.snapshot_count, analytics2.snapshot_count);
    }

    #[test]
    fn test_analytics_serialization() {
        let snapshots = vec!["snap1".to_string()];
        let policy = create_test_retention_policy(7);
        let analytics =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();
        let serialized = serde_json::to_string(&analytics).unwrap();

        assert!(serialized.contains("snapshot_count"));
    }
}
