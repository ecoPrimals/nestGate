// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Snapshot policy scheduler.
//!
//! Handles automated snapshot creation based on time-based policies.
//! Retention logic lives in [`super::retention`].

use chrono::{Datelike, Timelike};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::dataset::ZfsDatasetManager;
use crate::performance::types::SnapshotPolicyMap;
use nestgate_core::{NestGateError, Result as CoreResult};

use super::operations::SnapshotOperationType;
use super::policy::{ScheduleFrequency, SnapshotPolicy};
use super::retention;
use super::types::{SnapshotOperation, SnapshotOperationStatus};
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Policy scheduler for managing automated snapshot creation.
#[derive(Debug)]
pub struct PolicyScheduler {
    dataset_manager: Arc<ZfsDatasetManager>,
    policies: SnapshotPolicyMap,
    operation_queue: Arc<RwLock<Vec<SnapshotOperation>>>,
}
impl PolicyScheduler {
    /// Create a new policy scheduler
    pub const fn new(
        dataset_manager: Arc<ZfsDatasetManager>,
        policies: SnapshotPolicyMap,
        operation_queue: Arc<RwLock<Vec<SnapshotOperation>>>,
    ) -> Self {
        Self {
            dataset_manager,
            policies,
            operation_queue,
        }
    }

    /// Process all policies and create snapshots as needed
    pub async fn process_policies(&self) -> CoreResult<()> {
        let policies: Vec<SnapshotPolicy> = self.policies.read().await.values().cloned().collect();

        for policy in &policies {
            if !policy.enabled {
                continue;
            }

            if Self::should_execute_policy(policy) {
                info!("Executing policy: {}", policy.name);
                if let Err(e) = self.execute_policy(policy).await {
                    error!("Failed to execute policy {}: {}", policy.name, e);
                }
            }
        }
        Ok(())
    }

    /// Check if a policy should be executed based on its schedule
    fn should_execute_policy(policy: &SnapshotPolicy) -> bool {
        let now = SystemTime::now();
        let now_duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();

        match &policy.frequency {
            ScheduleFrequency::Minutes(minutes) => {
                let interval = Duration::from_secs(u64::from(*minutes) * 60);
                now_duration.as_secs() % interval.as_secs() < 60 // Within 1 minute of schedule
            }
            ScheduleFrequency::Hours(hours) => {
                let interval = Duration::from_secs(u64::from(*hours) * 3600);
                now_duration.as_secs() % interval.as_secs() < 300 // Within 5 minutes of schedule
            }
            ScheduleFrequency::Daily(hour) => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                datetime.hour() == u32::from(*hour) && datetime.minute() < 5
            }
            ScheduleFrequency::Weekly { day, hour } => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                datetime.weekday().num_days_from_monday() == u32::from(*day)
                    && datetime.hour() == u32::from(*hour)
                    && datetime.minute() < 5
            }
            ScheduleFrequency::Monthly { day, hour } => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                datetime.day() == u32::from(*day)
                    && datetime.hour() == u32::from(*hour)
                    && datetime.minute() < 5
            }
            ScheduleFrequency::Custom(_) => {
                // Simplified custom schedule check - would need proper cron parser
                warn!("Custom schedule not fully implemented");
                false
            }
        }
    }

    /// Execute a snapshot policy
    async fn execute_policy(&self, policy: &SnapshotPolicy) -> CoreResult<()> {
        debug!("Executing policy: {}", policy.name);

        // Get matching datasets
        let datasets = self.get_matching_datasets(policy).await?;

        for (created_snapshots, dataset) in datasets.into_iter().enumerate() {
            if created_snapshots >= policy.max_snapshots_per_run as usize {
                warn!(
                    "Reached max snapshots per run ({}) for policy {}",
                    policy.max_snapshots_per_run, policy.name
                );
                break;
            }

            // Generate snapshot name
            let snapshot_name = self.generate_snapshot_name(policy, &dataset);

            // Create snapshot operation
            let operation = SnapshotOperation {
                id: format!(
                    "{}_{}_{}_{}",
                    policy.name,
                    dataset,
                    snapshot_name,
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_else(|_| Duration::from_secs(0))
                        .as_secs()
                ),
                operation_type: SnapshotOperationType::Create,
                dataset: dataset.clone(),
                snapshot_name: Some(snapshot_name.clone()),
                status: SnapshotOperationStatus::Queued,
                created_at: SystemTime::now(),
                started_at: None,
                completed_at: None,
                error_message: None,
                policy: Some(policy.name.clone()),
            };

            // Add to operation queue
            {
                let mut queue = self.operation_queue.write().await;
                queue.push(operation);
            }

            info!("Queued snapshot creation for {}@{}", dataset, snapshot_name);
        }

        // Apply retention policy
        self.apply_retention_policy_for_policy(policy).await?;
        Ok(())
    }

    /// Get datasets matching policy patterns
    async fn get_matching_datasets(&self, policy: &SnapshotPolicy) -> CoreResult<Vec<String>> {
        let all_datasets = self.dataset_manager.list_datasets().await?;
        let mut matching_datasets = Vec::new();

        for dataset in all_datasets {
            for pattern in &policy.dataset_patterns {
                if Self::matches_pattern(&dataset.name, pattern) {
                    matching_datasets.push(dataset.name.clone());
                    break;
                }
            }
        }

        Ok(matching_datasets)
    }

    /// Check if dataset name matches pattern
    fn matches_pattern(dataset: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        // Simple glob matching - would need proper glob library for full support
        if let Some(prefix) = pattern.strip_suffix('*') {
            dataset.starts_with(prefix)
        } else if let Some(suffix) = pattern.strip_prefix('*') {
            dataset.ends_with(suffix)
        } else {
            dataset == pattern
        }
    }

    /// Generate snapshot name for policy
    fn generate_snapshot_name(&self, policy: &SnapshotPolicy, dataset: &str) -> String {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        match &policy.frequency {
            ScheduleFrequency::Minutes(_) | ScheduleFrequency::Custom(_) => {
                format!(
                    "{}_{}_{}",
                    policy.name_prefix,
                    dataset.replace('/', "_"),
                    timestamp
                )
            }
            ScheduleFrequency::Hours(_) => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                format!(
                    "{}_{}_{}_{:02}{:02}",
                    policy.name_prefix,
                    dataset.replace('/', "_"),
                    datetime.format("%Y%m%d"),
                    datetime.hour(),
                    datetime.minute()
                )
            }
            ScheduleFrequency::Daily(_) => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                format!(
                    "{}_{}_daily_{}",
                    policy.name_prefix,
                    dataset.replace('/', "_"),
                    datetime.format("%Y%m%d")
                )
            }
            ScheduleFrequency::Weekly { .. } => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                format!(
                    "{}_{}_weekly_{}",
                    policy.name_prefix,
                    dataset.replace('/', "_"),
                    datetime.format("%Y_W%U")
                )
            }
            ScheduleFrequency::Monthly { .. } => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                format!(
                    "{}_{}_monthly_{}",
                    policy.name_prefix,
                    dataset.replace('/', "_"),
                    datetime.format("%Y%m")
                )
            }
        }
    }

    /// Apply retention policy for all datasets affected by a policy
    async fn apply_retention_policy_for_policy(&self, policy: &SnapshotPolicy) -> CoreResult<()> {
        let datasets = self.get_matching_datasets(policy).await?;

        for dataset in datasets {
            if let Err(e) = self
                .apply_retention_policy_for_dataset(&dataset, &policy.retention)
                .await
            {
                error!(
                    "Failed to apply retention policy for dataset {}: {}",
                    dataset, e
                );
            }
        }
        Ok(())
    }

    /// Apply retention policy for a specific dataset
    async fn apply_retention_policy_for_dataset(
        &self,
        dataset: &str,
        retention: &super::policy::RetentionPolicy,
    ) -> CoreResult<()> {
        debug!("Applying retention policy for dataset: {}", dataset);

        let snapshots = self.dataset_manager.list_snapshots(dataset).await?;
        let snapshots_to_delete = retention::select_for_deletion(snapshots, retention);

        // Queue deletion operations
        let delete_operations: Vec<SnapshotOperation> = snapshots_to_delete
            .into_iter()
            .map(|snapshot| SnapshotOperation {
                id: format!(
                    "delete_{}_{}",
                    snapshot.dataset.replace('/', "_"),
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_else(|_| Duration::from_secs(0))
                        .as_secs()
                ),
                operation_type: SnapshotOperationType::Delete,
                dataset: snapshot.dataset,
                snapshot_name: Some(snapshot.name),
                status: SnapshotOperationStatus::Queued,
                created_at: SystemTime::now(),
                started_at: None,
                completed_at: None,
                error_message: None,
                policy: None,
            })
            .collect();

        self.operation_queue.write().await.extend(delete_operations);
        Ok(())
    }

    /// Parse schedule frequency to duration for next execution
    pub fn parse_schedule(&self, schedule: &ScheduleFrequency) -> CoreResult<Duration> {
        match schedule {
            ScheduleFrequency::Minutes(minutes) => {
                Ok(Duration::from_secs(u64::from(*minutes) * 60))
            }
            ScheduleFrequency::Hours(hours) => Ok(Duration::from_secs(u64::from(*hours) * 3600)),
            ScheduleFrequency::Daily(_) => Ok(Duration::from_secs(86400)), // 24 hours
            ScheduleFrequency::Weekly { .. } => Ok(Duration::from_secs(604800)), // 7 days
            ScheduleFrequency::Monthly { .. } => Ok(Duration::from_secs(2629746)), // ~30.44 days
            ScheduleFrequency::Custom(_) => Err(NestGateError::configuration_error(
                "schedule.frequency",
                "Custom schedule frequencies not yet implemented",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::ZfsDatasetManager;
    use crate::performance::types::SnapshotPolicyMap;
    use crate::snapshot::policy::{ScheduleFrequency, SnapshotPolicy};
    use std::collections::HashMap;

    fn test_scheduler() -> PolicyScheduler {
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let policies: SnapshotPolicyMap = Arc::new(RwLock::new(HashMap::new()));
        let queue = Arc::new(RwLock::new(Vec::new()));
        PolicyScheduler::new(dm, policies, queue)
    }

    #[test]
    fn matches_pattern_wildcard_and_prefix_suffix() {
        assert!(PolicyScheduler::matches_pattern("anything", "*"));
        assert!(PolicyScheduler::matches_pattern("prefixrest", "prefix*"));
        assert!(PolicyScheduler::matches_pattern("xsuffix", "*suffix"));
        assert!(PolicyScheduler::matches_pattern("exact", "exact"));
        assert!(!PolicyScheduler::matches_pattern("no", "yes"));
    }

    #[test]
    fn matches_pattern_literal_no_glob() {
        assert!(PolicyScheduler::matches_pattern("tank/data", "tank/data"));
        assert!(!PolicyScheduler::matches_pattern("tank/data2", "tank/data"));
    }

    #[test]
    fn should_execute_custom_schedule_is_false() {
        let policy = SnapshotPolicy {
            frequency: ScheduleFrequency::Custom("x".into()),
            ..SnapshotPolicy::default()
        };
        assert!(!PolicyScheduler::should_execute_policy(&policy));
    }

    #[tokio::test]
    async fn parse_schedule_variants() {
        let s = test_scheduler();
        assert_eq!(
            s.parse_schedule(&ScheduleFrequency::Minutes(10))
                .expect("test: minutes"),
            Duration::from_secs(600)
        );
        assert_eq!(
            s.parse_schedule(&ScheduleFrequency::Hours(3))
                .expect("test: hours"),
            Duration::from_secs(10800)
        );
        assert_eq!(
            s.parse_schedule(&ScheduleFrequency::Daily(4))
                .expect("test: daily"),
            Duration::from_secs(86400)
        );
        assert_eq!(
            s.parse_schedule(&ScheduleFrequency::Weekly { day: 1, hour: 2 })
                .expect("test: weekly"),
            Duration::from_secs(604800)
        );
        assert_eq!(
            s.parse_schedule(&ScheduleFrequency::Monthly { day: 1, hour: 0 })
                .expect("test: monthly"),
            Duration::from_secs(2629746)
        );
        s.parse_schedule(&ScheduleFrequency::Custom("c".into()))
            .expect_err("test: custom parse_schedule");
    }

    #[tokio::test]
    async fn generate_snapshot_name_formats_by_frequency() {
        let s = test_scheduler();
        let mut p = SnapshotPolicy {
            name_prefix: "pre".into(),
            ..SnapshotPolicy::default()
        };
        p.frequency = ScheduleFrequency::Minutes(5);
        let n1 = s.generate_snapshot_name(&p, "z/a");
        assert!(n1.contains("pre"));
        assert!(n1.contains("z_a"));

        p.frequency = ScheduleFrequency::Hours(1);
        let n2 = s.generate_snapshot_name(&p, "z/a");
        assert!(n2.starts_with("pre_z_a_"));

        p.frequency = ScheduleFrequency::Daily(0);
        let n3 = s.generate_snapshot_name(&p, "z/a");
        assert!(n3.contains("_daily_"));

        p.frequency = ScheduleFrequency::Weekly { day: 0, hour: 0 };
        let n4 = s.generate_snapshot_name(&p, "z/a");
        assert!(n4.contains("_weekly_"));

        p.frequency = ScheduleFrequency::Monthly { day: 1, hour: 0 };
        let n5 = s.generate_snapshot_name(&p, "z/a");
        assert!(n5.contains("_monthly_"));

        p.frequency = ScheduleFrequency::Custom("x".into());
        let n6 = s.generate_snapshot_name(&p, "z/a");
        assert!(n6.contains("pre"));
    }

    #[tokio::test]
    async fn process_policies_empty_map_ok() {
        let s = test_scheduler();
        s.process_policies().await.expect("process");
    }

    #[tokio::test]
    async fn process_policies_disabled_policy_skipped() {
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let mut map = HashMap::new();
        map.insert(
            "p1".into(),
            SnapshotPolicy {
                name: "disabled".into(),
                enabled: false,
                ..SnapshotPolicy::default()
            },
        );
        let policies: SnapshotPolicyMap = Arc::new(RwLock::new(map));
        let queue = Arc::new(RwLock::new(Vec::new()));
        let s = PolicyScheduler::new(dm, policies, queue);
        s.process_policies().await.expect("process");
    }

    /// `Minutes(1)` uses a 60s window where `secs % 60 < 60` is always true, so execution is deterministic.
    #[tokio::test]
    async fn process_policies_enabled_minutes_policy_runs_execute_and_retention_paths() {
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let mut map = HashMap::new();
        map.insert(
            "every_minute".into(),
            SnapshotPolicy {
                name: "min".into(),
                enabled: true,
                frequency: ScheduleFrequency::Minutes(1),
                dataset_patterns: vec!["*".into()],
                ..SnapshotPolicy::default()
            },
        );
        let policies: SnapshotPolicyMap = Arc::new(RwLock::new(map));
        let queue = Arc::new(RwLock::new(Vec::new()));
        let s = PolicyScheduler::new(dm, policies, queue);
        s.process_policies().await.expect("process");
    }

    #[test]
    fn should_execute_minutes_one_is_always_within_window() {
        let policy = SnapshotPolicy {
            frequency: ScheduleFrequency::Minutes(1),
            ..SnapshotPolicy::default()
        };
        assert!(PolicyScheduler::should_execute_policy(&policy));
    }

    #[test]
    fn matches_pattern_bare_star_prefix_suffix_edge_cases() {
        assert!(PolicyScheduler::matches_pattern("ab", "a*"));
        assert!(PolicyScheduler::matches_pattern("ba", "*a"));
        assert!(!PolicyScheduler::matches_pattern("x", "a*"));
    }

    #[tokio::test]
    async fn process_policies_zero_max_snapshots_per_run_skips_queue() {
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let mut map = HashMap::new();
        map.insert(
            "zero_cap".into(),
            SnapshotPolicy {
                name: "zero_cap".into(),
                enabled: true,
                frequency: ScheduleFrequency::Minutes(1),
                max_snapshots_per_run: 0,
                dataset_patterns: vec!["*".into()],
                ..SnapshotPolicy::default()
            },
        );
        let policies: SnapshotPolicyMap = Arc::new(RwLock::new(map));
        let queue = Arc::new(RwLock::new(Vec::new()));
        let s = PolicyScheduler::new(dm, policies, queue.clone());
        s.process_policies().await.expect("process");
        assert!(queue.read().await.is_empty());
    }

    #[test]
    fn should_execute_daily_branch_smoke() {
        let policy = SnapshotPolicy {
            frequency: ScheduleFrequency::Daily(12),
            ..SnapshotPolicy::default()
        };
        let _ = PolicyScheduler::should_execute_policy(&policy);
    }

    #[test]
    fn should_execute_weekly_and_monthly_smoke() {
        let w = SnapshotPolicy {
            frequency: ScheduleFrequency::Weekly { day: 2, hour: 3 },
            ..SnapshotPolicy::default()
        };
        let _ = PolicyScheduler::should_execute_policy(&w);
        let m = SnapshotPolicy {
            frequency: ScheduleFrequency::Monthly { day: 15, hour: 4 },
            ..SnapshotPolicy::default()
        };
        let _ = PolicyScheduler::should_execute_policy(&m);
    }

    #[test]
    fn should_execute_hours_branch_smoke() {
        let policy = SnapshotPolicy {
            frequency: ScheduleFrequency::Hours(6),
            ..SnapshotPolicy::default()
        };
        let _ = PolicyScheduler::should_execute_policy(&policy);
    }
}
