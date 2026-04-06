// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// ZFS snapshot analysis and optimization recommendations

use crate::types::RetentionPolicy;
use nestgate_core::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;
use tracing::debug;

/// Fallback when `zfs list` cannot be used (no binary, dataset missing, or parse failure).
const FALLBACK_BYTES_PER_SNAPSHOT_ESTIMATE: u64 = 1024 * 1024 * 100;

/// Sums `used` from `zfs list -r -t snapshot` for snapshots whose names match `snapshot_labels`.
///
/// Returns [`None`] if the `zfs` command fails or output cannot be parsed. Matching uses the
/// snapshot tag after `@` or the full `pool/dataset@snap` name.
fn try_sum_snapshot_used_from_zfs(dataset: &str, snapshot_labels: &[String]) -> Option<u64> {
    let output = Command::new("zfs")
        .args([
            "list",
            "-r",
            "-t",
            "snapshot",
            "-H",
            "-p",
            "-o",
            "name,used",
            dataset,
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let label_set: HashSet<&str> = snapshot_labels.iter().map(String::as_str).collect();
    let mut sum: u64 = 0;
    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }
        let (full_name, used_str) = line.split_once('\t')?;
        let used: u64 = used_str.trim().parse().ok()?;
        let snap_short = full_name.split_once('@').map_or(full_name, |(_, s)| s);
        if label_set.contains(snap_short) || label_set.contains(full_name) {
            sum = sum.saturating_add(used);
        }
    }
    Some(sum)
}

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
    /// Analyze snapshot usage.
    ///
    /// `storage_usage` prefers the sum of snapshot `used` bytes from
    /// `zfs list -r -t snapshot -H -p -o name,used <dataset>` for snapshots whose names match
    /// `snapshots`. If that sum is zero or `zfs` is unavailable, falls back to
    /// `snapshot_count × 100 MiB` as a planning estimate.
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
        let storage_usage = if snapshot_count == 0 {
            0
        } else {
            match try_sum_snapshot_used_from_zfs(dataset, snapshots) {
                Some(zfs_sum) if zfs_sum > 0 => zfs_sum,
                _ => snapshot_count.saturating_mul(FALLBACK_BYTES_PER_SNAPSHOT_ESTIMATE),
            }
        };
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
        let snapshots: Vec<String> = (0..60).map(|i| format!("snap{i}")).collect();
        let policy = create_test_retention_policy(7);
        let analytics = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy)
            .expect("Test: analyze_snapshots should succeed");

        assert!(
            analytics
                .recommendations
                .iter()
                .any(|r| r.contains("cleaning up"))
        );
    }

    #[test]
    fn test_high_retention_recommendation() {
        let snapshots = vec!["snap1".to_string()];
        let policy = create_test_retention_policy(40);
        let analytics = SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy)
            .expect("Test: analyze_snapshots should succeed");

        assert!(
            analytics
                .recommendations
                .iter()
                .any(|r| r.contains("retention is very high"))
        );
    }

    #[test]
    fn test_high_storage_usage_recommendation() {
        let snapshots: Vec<String> = (0..150).map(|i| format!("snap{i}")).collect();
        let policy = create_test_retention_policy(7);
        let analytics =
            SnapshotAnalytics::analyze_snapshots("tank/data", &snapshots, &policy).unwrap();

        assert!(
            analytics
                .recommendations
                .iter()
                .any(|r| r.contains("significant storage"))
        );
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
        let analytics = SnapshotAnalytics::analyze_snapshots(
            "zzz_nestgate_snapshot_storage_fallback_ds",
            &snapshots,
            &policy,
        )
        .unwrap();

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
