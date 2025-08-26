//
// Handles scheduling and execution of snapshot policies including
// retention management and automated snapshot creation.

use chrono::{Datelike, Timelike};
// Removed unused HashMap import
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::dataset::ZfsDatasetManager;
use crate::performance::types::SnapshotPolicyMap;
use nestgate_core::{NestGateError, Result as CoreResult};

use super::operations::SnapshotOperationType;
use super::policy::{RetentionPolicy, ScheduleFrequency, SnapshotPolicy};
use super::types::{SnapshotInfo, SnapshotOperation, SnapshotOperationStatus};
use nestgate_core::error::UnifiedConfigSource;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

/// Policy scheduler for managing automated snapshot creation
#[derive(Debug)]
pub struct PolicyScheduler {
    dataset_manager: Arc<ZfsDatasetManager>,
    policies: SnapshotPolicyMap,
    operation_queue: Arc<RwLock<Vec<SnapshotOperation>>>,
}

impl PolicyScheduler {
    /// Create a new policy scheduler
    pub fn new(
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
        let policies = self.policies.read().await;

        for policy in policies.values() {
            if !policy.enabled {
                continue;
            }

            if Self::should_execute_policy(policy).await {
                info!("Executing policy: {}", policy.name);
                if let Err(e) = self.execute_policy(policy).await {
                    error!("Failed to execute policy {}: {}", policy.name, e);
                }
            }
        }
        Ok(())
    }

    /// Check if a policy should be executed based on its schedule
    async fn should_execute_policy(policy: &SnapshotPolicy) -> bool {
        let now = SystemTime::now();
        let now_duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();

        match &policy.frequency {
            ScheduleFrequency::Minutes(minutes) => {
                let interval = Duration::from_secs(*minutes as u64 * 60);
                now_duration.as_secs() % interval.as_secs() < 60 // Within 1 minute of schedule
            }
            ScheduleFrequency::Hours(hours) => {
                let interval = Duration::from_secs(*hours as u64 * 3600);
                now_duration.as_secs() % interval.as_secs() < 300 // Within 5 minutes of schedule
            }
            ScheduleFrequency::Daily(hour) => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                datetime.hour() == *hour as u32 && datetime.minute() < 5
            }
            ScheduleFrequency::Weekly { day, hour } => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                datetime.weekday().num_days_from_monday() == *day as u32
                    && datetime.hour() == *hour as u32
                    && datetime.minute() < 5
            }
            ScheduleFrequency::Monthly { day, hour } => {
                let datetime = chrono::DateTime::<chrono::Utc>::from(now);
                datetime.day() == *day as u32
                    && datetime.hour() == *hour as u32
                    && datetime.minute() < 5
            }
            ScheduleFrequency::Cron(_) => {
                // Simplified cron check - would need proper cron parser
                warn!("Cron schedule not fully implemented");
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
            let snapshot_name = self.generate_snapshot_name(policy, &dataset).await;

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
            let mut queue = self.operation_queue.write().await;
            queue.push(operation);

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
    async fn generate_snapshot_name(&self, policy: &SnapshotPolicy, dataset: &str) -> String {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        match &policy.frequency {
            ScheduleFrequency::Minutes(_) => {
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
            ScheduleFrequency::Cron(_) => {
                format!(
                    "{}_{}_{}",
                    policy.name_prefix,
                    dataset.replace('/', "_"),
                    timestamp
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
        retention: &RetentionPolicy,
    ) -> CoreResult<()> {
        debug!("Applying retention policy for dataset: {}", dataset);

        let snapshots = self.dataset_manager.list_snapshots(dataset).await?;
        let mut snapshots_to_delete = Vec::new();

        match retention {
            RetentionPolicy::Duration(duration) => {
                let cutoff = SystemTime::now() - *duration;
                snapshots_to_delete = snapshots
                    .into_iter()
                    .filter(|s| s.created_at < cutoff)
                    .collect();
            }
            RetentionPolicy::Count(count) => {
                if snapshots.len() > *count as usize {
                    let mut sorted_snapshots = snapshots;
                    sorted_snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                    snapshots_to_delete =
                        sorted_snapshots.into_iter().skip(*count as usize).collect();
                }
            }
            RetentionPolicy::Custom {
                hourly_hours,
                daily_days,
                weekly_weeks,
                monthly_months,
                yearly_years,
            } => {
                snapshots_to_delete = self
                    .apply_custom_retention(
                        snapshots,
                        *hourly_hours,
                        *daily_days,
                        *weekly_weeks,
                        *monthly_months,
                        *yearly_years,
                    )
                    .await;
            }
        }

        // Queue deletion operations
        let mut queue = self.operation_queue.write().await;
        for snapshot in snapshots_to_delete {
            let operation = SnapshotOperation {
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
            };
            queue.push(operation);
        }
        Ok(())
    }

    /// Apply custom retention policy
    #[allow(clippy::too_many_arguments)] // Retention policy requires multiple time periods
    async fn apply_custom_retention(
        &self,
        snapshots: Vec<SnapshotInfo>,
        hourly_hours: u32,
        daily_days: u32,
        weekly_weeks: u32,
        monthly_months: u32,
        yearly_years: u32,
    ) -> Vec<SnapshotInfo> {
        // This is a simplified implementation
        // A full implementation would categorize snapshots by time period
        // and keep only the required number from each category

        let now = SystemTime::now();
        let _hour_cutoff = now - Duration::from_secs(hourly_hours as u64 * 3600);
        let _day_cutoff = now - Duration::from_secs(daily_days as u64 * 86400);
        let _week_cutoff = now - Duration::from_secs(weekly_weeks as u64 * 604800);
        let _month_cutoff = now - Duration::from_secs(monthly_months as u64 * 2629746); // ~30.44 days
        let year_cutoff = now - Duration::from_secs(yearly_years as u64 * 31556952); // ~365.25 days

        snapshots
            .into_iter()
            .filter(|s| s.created_at < year_cutoff)
            .collect()
    }

    /// Parse schedule frequency to duration for next execution
    pub fn parse_schedule(&self, schedule: &ScheduleFrequency) -> CoreResult<Duration> {
        match schedule {
            ScheduleFrequency::Minutes(minutes) => Ok(Duration::from_secs(*minutes as u64 * 60)),
            ScheduleFrequency::Hours(hours) => Ok(Duration::from_secs(*hours as u64 * 3600)),
            ScheduleFrequency::Daily(_) => Ok(Duration::from_secs(86400)), // 24 hours
            ScheduleFrequency::Weekly { .. } => Ok(Duration::from_secs(604800)), // 7 days
            ScheduleFrequency::Monthly { .. } => Ok(Duration::from_secs(2629746)), // ~30.44 days
            ScheduleFrequency::Cron(_) => Err(NestGateError::Configuration {
                message: "Cron parsing not implemented".to_string(),
                config_source: UnifiedConfigSource::File("zfs.conf".to_string()),
                field: None,
                suggested_fix: None,
            }),
        }
    }
}
