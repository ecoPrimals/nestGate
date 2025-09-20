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
