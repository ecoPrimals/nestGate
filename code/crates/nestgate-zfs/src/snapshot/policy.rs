// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Configuration types for snapshot policies including retention rules,
// scheduling frequencies, and policy definitions.

use crate::types::StorageTier;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retentionpolicy
pub enum RetentionPolicy {
    /// Keep snapshots for a specific duration
    Duration(Duration),
    /// Keep a specific number of snapshots
    Count(u32),
    /// Custom retention rule
    Custom {
        /// Keep hourly snapshots for this many hours
        hourly_hours: u32,
        /// Keep daily snapshots for this many days
        daily_days: u32,
        /// Keep weekly snapshots for this many weeks
        weekly_weeks: u32,
        /// Keep monthly snapshots for this many months
        monthly_months: u32,
        /// Keep yearly snapshots for this many years
        yearly_years: u32,
    },
}
impl Default for RetentionPolicy {
    /// Returns the default instance
    fn default() -> Self {
        Self::Custom {
            hourly_hours: 24,   // 24 hours
            daily_days: 30,     // 30 days
            weekly_weeks: 12,   // 12 weeks
            monthly_months: 12, // 12 months
            yearly_years: 5,    // 5 years
        }
    }
}

/// Snapshot schedule frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Schedulefrequency
pub enum ScheduleFrequency {
    /// Take snapshots every N minutes
    Minutes(u32),
    /// Take snapshots every N hours
    Hours(u32),
    /// Take snapshots daily at specific hour
    Daily(u8),
    /// Take snapshots weekly on specific day and hour
    Weekly {
        /// Day of week (0-6, where 0 is Sunday)
        day: u8,
        /// Hour of day (0-23)
        hour: u8,
    },
    /// Monthly schedule
    Monthly {
        /// Day of month (1-31)
        day: u8,
        /// Hour of day (0-23)
        hour: u8,
    },
    /// Custom cron-like schedule
    Custom(String),
}
/// Snapshot policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotpolicy
pub struct SnapshotPolicy {
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Whether policy is enabled
    pub enabled: bool,
    /// Schedule frequency
    pub frequency: ScheduleFrequency,
    /// Retention policy
    pub retention: RetentionPolicy,
    /// Datasets to apply policy to (glob patterns)
    pub dataset_patterns: Vec<String>,
    /// Storage tiers to apply policy to
    pub tiers: Vec<StorageTier>,
    /// Snapshot name prefix
    pub name_prefix: String,
    /// Whether to include properties in snapshot
    pub include_properties: bool,
    /// Whether to create recursive snapshots
    pub recursive: bool,
    /// Maximum snapshots to create per run
    pub max_snapshots_per_run: u32,
    /// Priority (higher number = higher priority)
    pub priority: u32,
}
impl Default for SnapshotPolicy {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            description: "Default snapshot policy".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Hours(1),
            retention: RetentionPolicy::default(),
            dataset_patterns: vec!["*".to_string()],
            tiers: vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold],
            name_prefix: "auto".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 100,
            priority: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_policy_default() {
        let policy = RetentionPolicy::default();
        match policy {
            RetentionPolicy::Custom {
                hourly_hours,
                daily_days,
                weekly_weeks,
                monthly_months,
                yearly_years,
            } => {
                assert_eq!(hourly_hours, 24);
                assert_eq!(daily_days, 30);
                assert_eq!(weekly_weeks, 12);
                assert_eq!(monthly_months, 12);
                assert_eq!(yearly_years, 5);
            }
            _ => panic!("Expected Custom retention policy"),
        }
    }

    #[test]
    fn test_retention_policy_duration() {
        let policy = RetentionPolicy::Duration(Duration::from_secs(3600));
        assert!(matches!(policy, RetentionPolicy::Duration(_)));
    }

    #[test]
    fn test_retention_policy_count() {
        let policy = RetentionPolicy::Count(100);
        assert!(matches!(policy, RetentionPolicy::Count(100)));
    }

    #[test]
    fn test_schedule_frequency_variants() {
        let minutes = ScheduleFrequency::Minutes(15);
        let hours = ScheduleFrequency::Hours(6);
        let daily = ScheduleFrequency::Daily(12);
        let weekly = ScheduleFrequency::Weekly { day: 1, hour: 0 };
        let monthly = ScheduleFrequency::Monthly { day: 1, hour: 0 };
        let custom = ScheduleFrequency::Custom("0 0 * * *".to_string());

        assert!(matches!(minutes, ScheduleFrequency::Minutes(15)));
        assert!(matches!(hours, ScheduleFrequency::Hours(6)));
        assert!(matches!(daily, ScheduleFrequency::Daily(12)));
        assert!(matches!(
            weekly,
            ScheduleFrequency::Weekly { day: 1, hour: 0 }
        ));
        assert!(matches!(
            monthly,
            ScheduleFrequency::Monthly { day: 1, hour: 0 }
        ));
        assert!(matches!(custom, ScheduleFrequency::Custom(_)));
    }

    #[test]
    fn test_snapshot_policy_default() {
        let policy = SnapshotPolicy::default();

        assert_eq!(policy.name, "default");
        assert_eq!(policy.description, "Default snapshot policy");
        assert!(policy.enabled);
        assert!(matches!(policy.frequency, ScheduleFrequency::Hours(1)));
        assert_eq!(policy.dataset_patterns, vec!["*"]);
        assert_eq!(policy.tiers.len(), 3);
        assert_eq!(policy.name_prefix, "auto");
        assert!(policy.include_properties);
        assert!(policy.recursive);
        assert_eq!(policy.max_snapshots_per_run, 100);
        assert_eq!(policy.priority, 50);
    }

    #[test]
    fn test_snapshot_policy_custom() {
        let policy = SnapshotPolicy {
            name: "custom".to_string(),
            description: "Custom policy".to_string(),
            enabled: false,
            frequency: ScheduleFrequency::Daily(2),
            retention: RetentionPolicy::Count(50),
            dataset_patterns: vec!["tank/*".to_string()],
            tiers: vec![StorageTier::Hot],
            name_prefix: "custom".to_string(),
            include_properties: false,
            recursive: false,
            max_snapshots_per_run: 50,
            priority: 100,
        };

        assert_eq!(policy.name, "custom");
        assert!(!policy.enabled);
        assert!(matches!(policy.frequency, ScheduleFrequency::Daily(2)));
        assert!(matches!(policy.retention, RetentionPolicy::Count(50)));
    }

    #[test]
    fn test_snapshot_policy_serialization() {
        let policy = SnapshotPolicy::default();
        let serialized = serde_json::to_string(&policy).expect("Failed to serialize");
        let deserialized: SnapshotPolicy =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(policy.name, deserialized.name);
        assert_eq!(policy.enabled, deserialized.enabled);
        assert_eq!(policy.priority, deserialized.priority);
    }

    #[test]
    fn test_retention_policy_serialization() {
        let policy = RetentionPolicy::default();
        let serialized = serde_json::to_string(&policy).expect("Failed to serialize");
        let deserialized: RetentionPolicy =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        match (policy, deserialized) {
            (
                RetentionPolicy::Custom {
                    hourly_hours: h1, ..
                },
                RetentionPolicy::Custom {
                    hourly_hours: h2, ..
                },
            ) => {
                assert_eq!(h1, h2);
            }
            _ => panic!("Expected Custom retention policy"),
        }
    }

    #[test]
    fn test_schedule_frequency_serialization() {
        let freq = ScheduleFrequency::Hours(6);
        let serialized = serde_json::to_string(&freq).expect("Failed to serialize");
        let deserialized: ScheduleFrequency =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert!(matches!(deserialized, ScheduleFrequency::Hours(6)));
    }
}
