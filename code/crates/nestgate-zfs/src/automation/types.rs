//
// This module contains all the fundamental data structures used throughout
// the automation system including policies, lifecycle management, events,
// and status tracking.

use crate::types::StorageTier;
use serde::{Deserialize, Serialize};

use std::time::SystemTime;

/// Policy priority levels for automation policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyPriority {
    Low,
    Normal,
    High,
    Critical,
}
/// Policy conditions container that defines when and how automation rules apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConditions {
    pub tier_rules: Vec<TierRule>,
    pub migration_rules: Vec<MigrationRule>,
    pub lifecycle_rules: Vec<LifecycleRule>,
}
/// Simple tier rule for basic automation - defines target tier based on conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierRule {
    pub condition: String,
    pub target_tier: StorageTier,
    pub priority: u32,
}
/// Simple migration rule for automated dataset movement between tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRule {
    pub source_tier: StorageTier,
    pub target_tier: StorageTier,
    pub condition: String,
    pub bandwidth_limits: BandwidthLimits,
    pub schedule: String,
}
/// Simple lifecycle rule for dataset lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRule {
    pub stage: LifecycleStage,
    pub next_stage: Option<LifecycleStage>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
}
/// Dataset lifecycle management policy with complete configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationPolicy {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub priority: PolicyPriority,
    pub enabled: bool,
    pub conditions: PolicyConditions,
    pub created: SystemTime,
    pub last_modified: SystemTime,
}
/// Dataset lifecycle stages with automation rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LifecycleStage {
    /// Newly created, high activity expected
    New,
    /// Active usage phase
    Active,
    /// Declining usage, candidate for migration
    Aging,
    /// Low usage, moved to cold storage
    Archived,
    /// Marked for potential cleanup
    Obsolete,
}
impl std::fmt::Display for LifecycleStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifecycleStage::New => write!(f, "New"),
            LifecycleStage::Active => write!(f, "Active"),
            LifecycleStage::Aging => write!(f, "Aging"),
            LifecycleStage::Archived => write!(f, "Archived"),
            LifecycleStage::Obsolete => write!(f, "Obsolete"),
        }
    }
}

/// Dataset lifecycle tracking with comprehensive state management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetLifecycle {
    pub dataset_name: String,
    pub current_tier: StorageTier,
    pub created: SystemTime,
    pub last_accessed: Option<SystemTime>,
    pub access_count: u64,
    pub total_migrations: u32,
    pub last_optimization: Option<SystemTime>,
    pub lifecycle_stage: LifecycleStage,
    pub automation_history: Vec<AutomationEvent>,
}
/// Automation event tracking for audit and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationEvent {
    pub event_id: String,
    pub event_type: AutomationEventType,
    pub timestamp: SystemTime,
    pub details: String,
    pub success: bool,
}
/// Types of automation events for categorization and filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationEventType {
    TierAssignment,
    Migration,
    Optimization,
    Cleanup,
    Compression,
    Archival,
    PolicyUpdate,
}
/// Bandwidth limits for migrations and operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthLimits {
    /// Maximum migration bandwidth during peak hours (MB/s)
    pub peak_max_mbps: u64,
    /// Maximum migration bandwidth during off-peak hours (MB/s)
    pub off_peak_max_mbps: u64,
}
impl Default for BandwidthLimits {
    fn default() -> Self {
        Self {
            peak_max_mbps: 1000,
            off_peak_max_mbps: 2000,
        }
    }
}

/// Automation status information for monitoring and reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationStatus {
    pub enabled: bool,
    pub active_policies: u32,
    pub tracked_datasets: u32,
    pub total_migrations_performed: u32,
    pub last_automation_cycle: SystemTime,
}
/// Dataset metadata for tier evaluation and decision making
#[derive(Debug, Default)]
pub struct DatasetMetadata {
    pub size_bytes: u64,
    pub last_accessed: Option<SystemTime>,
    pub access_frequency: f64,
    pub file_types: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_priority_variants() {
        assert_eq!(PolicyPriority::Low, PolicyPriority::Low);
        assert_ne!(PolicyPriority::Low, PolicyPriority::High);
        assert_eq!(PolicyPriority::Critical, PolicyPriority::Critical);
    }

    #[test]
    fn test_lifecycle_stage_display() {
        assert_eq!(format!("{}", LifecycleStage::New), "New");
        assert_eq!(format!("{}", LifecycleStage::Active), "Active");
        assert_eq!(format!("{}", LifecycleStage::Aging), "Aging");
        assert_eq!(format!("{}", LifecycleStage::Archived), "Archived");
        assert_eq!(format!("{}", LifecycleStage::Obsolete), "Obsolete");
    }

    #[test]
    fn test_lifecycle_stage_equality() {
        assert_eq!(LifecycleStage::New, LifecycleStage::New);
        assert_ne!(LifecycleStage::New, LifecycleStage::Active);
    }

    #[test]
    fn test_bandwidth_limits_default() {
        let limits = BandwidthLimits::default();
        assert_eq!(limits.peak_max_mbps, 1000);
        assert_eq!(limits.off_peak_max_mbps, 2000);
    }

    #[test]
    fn test_bandwidth_limits_custom() {
        let limits = BandwidthLimits {
            peak_max_mbps: 500,
            off_peak_max_mbps: 1500,
        };
        assert_eq!(limits.peak_max_mbps, 500);
        assert_eq!(limits.off_peak_max_mbps, 1500);
    }

    #[test]
    fn test_tier_rule_creation() {
        let rule = TierRule {
            condition: "size > 1GB".to_string(),
            target_tier: StorageTier::Cold,
            priority: 10,
        };
        assert_eq!(rule.condition, "size > 1GB");
        assert_eq!(rule.priority, 10);
    }

    #[test]
    fn test_migration_rule_creation() {
        let rule = MigrationRule {
            source_tier: StorageTier::Hot,
            target_tier: StorageTier::Cold,
            condition: "age > 30 days".to_string(),
            bandwidth_limits: BandwidthLimits::default(),
            schedule: "nightly".to_string(),
        };
        assert_eq!(rule.schedule, "nightly");
    }

    #[test]
    fn test_lifecycle_rule_creation() {
        let rule = LifecycleRule {
            stage: LifecycleStage::Active,
            next_stage: Some(LifecycleStage::Aging),
            conditions: vec!["low_access".to_string()],
            actions: vec!["migrate".to_string()],
        };
        assert_eq!(rule.stage, LifecycleStage::Active);
        assert_eq!(rule.conditions.len(), 1);
    }

    #[test]
    fn test_automation_policy_creation() {
        let policy = AutomationPolicy {
            policy_id: "pol-001".to_string(),
            name: "Test Policy".to_string(),
            description: "A test policy".to_string(),
            priority: PolicyPriority::High,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        assert_eq!(policy.policy_id, "pol-001");
        assert!(policy.enabled);
    }

    #[test]
    fn test_dataset_lifecycle_creation() {
        let lifecycle = DatasetLifecycle {
            dataset_name: "tank/data".to_string(),
            current_tier: StorageTier::Hot,
            created: SystemTime::now(),
            last_accessed: None,
            access_count: 100,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::Active,
            automation_history: vec![],
        };
        assert_eq!(lifecycle.dataset_name, "tank/data");
        assert_eq!(lifecycle.access_count, 100);
    }

    #[test]
    fn test_automation_event_creation() {
        let event = AutomationEvent {
            event_id: "evt-001".to_string(),
            event_type: AutomationEventType::Migration,
            timestamp: SystemTime::now(),
            details: "Migrated to cold tier".to_string(),
            success: true,
        };
        assert_eq!(event.event_id, "evt-001");
        assert!(event.success);
    }

    #[test]
    fn test_automation_status_creation() {
        let status = AutomationStatus {
            enabled: true,
            active_policies: 5,
            tracked_datasets: 100,
            total_migrations_performed: 50,
            last_automation_cycle: SystemTime::now(),
        };
        assert!(status.enabled);
        assert_eq!(status.active_policies, 5);
    }

    #[test]
    fn test_dataset_metadata_default() {
        let metadata = DatasetMetadata::default();
        assert_eq!(metadata.size_bytes, 0);
        assert!(metadata.last_accessed.is_none());
        assert_eq!(metadata.access_frequency, 0.0);
    }

    #[test]
    fn test_policy_conditions_empty() {
        let conditions = PolicyConditions {
            tier_rules: vec![],
            migration_rules: vec![],
            lifecycle_rules: vec![],
        };
        assert_eq!(conditions.tier_rules.len(), 0);
        assert_eq!(conditions.migration_rules.len(), 0);
    }

    #[test]
    fn test_automation_event_type_variants() {
        let event1 = AutomationEvent {
            event_id: "1".to_string(),
            event_type: AutomationEventType::TierAssignment,
            timestamp: SystemTime::now(),
            details: "test".to_string(),
            success: true,
        };
        let event2 = AutomationEvent {
            event_id: "2".to_string(),
            event_type: AutomationEventType::Cleanup,
            timestamp: SystemTime::now(),
            details: "test".to_string(),
            success: true,
        };
        assert_eq!(event1.event_id, "1");
        assert_eq!(event2.event_id, "2");
    }

    #[test]
    fn test_policy_serialization() {
        let priority = PolicyPriority::High;
        let serialized = serde_json::to_string(&priority).unwrap();
        let deserialized: PolicyPriority = serde_json::from_str(&serialized).unwrap();
        assert_eq!(priority, deserialized);
    }
}
