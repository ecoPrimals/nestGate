// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Snapshot retention engine.
//!
//! Determines which snapshots should be deleted based on a [`RetentionPolicy`].
//! Called from the [`super::scheduler::PolicyScheduler`] after policy execution.

use super::policy::RetentionPolicy;
use super::types::SnapshotInfo;
use std::time::{Duration, SystemTime};

/// Select snapshots that should be deleted according to the given retention policy.
pub fn select_for_deletion(
    snapshots: Vec<SnapshotInfo>,
    retention: &RetentionPolicy,
) -> Vec<SnapshotInfo> {
    match retention {
        RetentionPolicy::Duration(duration) => {
            let cutoff = SystemTime::now() - *duration;
            snapshots
                .into_iter()
                .filter(|s| s.created_at < cutoff)
                .collect()
        }
        RetentionPolicy::Count(count) => {
            if snapshots.len() > *count as usize {
                let mut sorted = snapshots;
                sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                sorted.into_iter().skip(*count as usize).collect()
            } else {
                Vec::new()
            }
        }
        RetentionPolicy::Custom {
            hourly_hours,
            daily_days: _,
            weekly_weeks: _,
            monthly_months: _,
            yearly_years,
        } => custom_retention(
            snapshots,
            *hourly_hours,
            *yearly_years,
        ),
    }
}

/// Apply custom GFS-style retention.
///
/// Simplified: keeps everything newer than the yearly cutoff.
/// A full implementation would bucket snapshots by time period and keep
/// only the required count from each bucket.
fn custom_retention(
    snapshots: Vec<SnapshotInfo>,
    _hourly_hours: u32,
    yearly_years: u32,
) -> Vec<SnapshotInfo> {
    let year_cutoff =
        SystemTime::now() - Duration::from_secs(u64::from(yearly_years) * 31_556_952);
    snapshots
        .into_iter()
        .filter(|s| s.created_at < year_cutoff)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StorageTier;
    use std::collections::HashMap;
    use std::time::UNIX_EPOCH;

    fn test_snap(name: &str, created_at: SystemTime) -> SnapshotInfo {
        SnapshotInfo {
            name: name.into(),
            full_name: format!("ds@{name}"),
            dataset: "ds".into(),
            created_at,
            size: 1,
            referenced_size: 1,
            written_size: 1,
            compression_ratio: 1.0,
            properties: HashMap::new(),
            policy: None,
            tier: StorageTier::Warm,
            protected: false,
            tags: Vec::new(),
        }
    }

    #[test]
    fn duration_retention_keeps_recent() {
        let recent = test_snap("new", SystemTime::now());
        let old = test_snap("old", UNIX_EPOCH);
        let out = select_for_deletion(
            vec![recent, old],
            &RetentionPolicy::Duration(Duration::from_secs(3600)),
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "old");
    }

    #[test]
    fn count_retention_keeps_newest() {
        let a = test_snap("a", UNIX_EPOCH + Duration::from_secs(10));
        let b = test_snap("b", UNIX_EPOCH + Duration::from_secs(20));
        let out = select_for_deletion(vec![a, b], &RetentionPolicy::Count(1));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "a");
    }

    #[test]
    fn count_retention_empty_when_within_limit() {
        let snap = test_snap("only", UNIX_EPOCH);
        let out = select_for_deletion(vec![snap], &RetentionPolicy::Count(5));
        assert!(out.is_empty());
    }

    #[test]
    fn custom_retention_marks_old_snapshots() {
        let old = test_snap("s1", UNIX_EPOCH);
        let to_delete = select_for_deletion(
            vec![old.clone()],
            &RetentionPolicy::Custom {
                hourly_hours: 0,
                daily_days: 0,
                weekly_weeks: 0,
                monthly_months: 0,
                yearly_years: 1,
            },
        );
        assert!(to_delete.iter().any(|x| x.full_name == old.full_name));
    }

    #[test]
    fn custom_retention_empty_input() {
        let out = select_for_deletion(
            Vec::new(),
            &RetentionPolicy::Custom {
                hourly_hours: 1,
                daily_days: 1,
                weekly_weeks: 1,
                monthly_months: 1,
                yearly_years: 1,
            },
        );
        assert!(out.is_empty());
    }
}
