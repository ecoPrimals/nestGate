//
// Tests for snapshot policy and retention

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use nestgate_core::StorageTier as CoreStorageTier;
use nestgate_zfs::performance::TierMetrics;
use nestgate_zfs::performance::{AlertCondition, AlertMetric, AlertOperator, AlertSeverity};
use nestgate_zfs::{
    automation::{DatasetLifecycle, LifecycleRule, LifecycleStage},
    config::ZfsConfig,
    migration::{MigrationJob, MigrationPriority, MigrationStatus},
    snapshot::*,
    types::StorageTier,
};

#[cfg(test)]
mod snapshot_unit_tests {
    use super::*;

    #[test]
    fn test_snapshot_policy_validation() {
        let policy = SnapshotPolicy::default();

        assert_eq!(policy.name, "default");
        assert!(policy.enabled);
        assert!(matches!(policy.frequency, ScheduleFrequency::Hours(1)));
        assert!(!policy.dataset_patterns.is_empty());
        assert!(policy.max_snapshots_per_run > 0);
    }

    #[test]
    fn test_retention_policy_custom() {
        let policy = RetentionPolicy::Custom {
            hourly_hours: 24,
            daily_days: 30,
            weekly_weeks: 12,
            monthly_months: 12,
            yearly_years: 5,
        };

        if let RetentionPolicy::Custom {
            hourly_hours,
            daily_days,
            weekly_weeks,
            monthly_months,
            yearly_years,
        } = policy
        {
            assert_eq!(hourly_hours, 24);
            assert_eq!(daily_days, 30);
            assert_eq!(weekly_weeks, 12);
            assert_eq!(monthly_months, 12);
            assert_eq!(yearly_years, 5);
        } else {
            return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Expected Custom retention policy".to_string()
).into());
        }
    }

    #[test]
    fn test_snapshot_schedule_creation() {
        let schedule = ScheduleFrequency::Hours(4);
        assert!(matches!(schedule, ScheduleFrequency::Hours(4)));
        
        let schedule = ScheduleFrequency::Daily;
        assert!(matches!(schedule, ScheduleFrequency::Daily));
    }
} 