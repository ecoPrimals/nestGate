//
// Configuration types for snapshot policies including retention rules,
// scheduling frequencies, and policy definitions.

use crate::types::StorageTier;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub enum ScheduleFrequency {
    /// Take snapshots every N minutes
    Minutes(u32),
    /// Take snapshots every N hours
    Hours(u32),
    /// Take snapshots daily at specific hour
    Daily(u8),
    /// Take snapshots weekly on specific day and hour
    Weekly { day: u8, hour: u8 },
    /// Monthly schedule
    Monthly { day: u8, hour: u8 },
    /// Custom cron-like schedule
    Custom(String),
}
/// Snapshot policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
