//! Lifecycle management type definitions for dataset automation.

use nestgate_core::unified_enums::storage_types::StorageTier;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Lifecycle stage for datasets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LifecycleStage {
    /// Newly created dataset
    Created,
    /// Active dataset with regular access
    Active,
    /// Aging dataset with reduced access
    Aging,
    /// Archived dataset with minimal access
    Archived,
    /// Deprecated dataset scheduled for deletion
    Deprecated,
    /// Deleted dataset
    Deleted,
}

/// Lifecycle policy for dataset management
#[derive(Debug, Clone)]
pub struct LifecyclePolicy {
    /// Policy identifier
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Transition rules between stages
    pub transitions: Vec<LifecycleTransition>,
    /// Actions to perform at each stage
    pub stage_actions: HashMap<LifecycleStage, Vec<LifecycleAction>>,
    /// Policy priority (higher = more important)
    pub priority: u32,
    /// Whether this policy is active
    pub enabled: bool,
}

/// Transition rule between lifecycle stages
#[derive(Debug, Clone)]
pub struct LifecycleTransition {
    /// Source stage
    pub from_stage: LifecycleStage,
    /// Target stage
    pub to_stage: LifecycleStage,
    /// All conditions must be met for transition
    pub conditions: Vec<TransitionCondition>,
    /// Minimum time in current stage before transition
    pub min_stage_duration: Duration,
    /// Whether this transition requires manual approval
    pub requires_approval: bool,
}

/// Condition for stage transitions
#[derive(Debug, Clone)]
pub enum TransitionCondition {
    /// Age of dataset exceeds threshold
    AgeExceeds(Duration),
    /// Access frequency below threshold
    AccessBelowThreshold(u32),
    /// Dataset size exceeds threshold
    SizeExceeds(u64),
    /// Storage tier matches condition
    TierMatches(StorageTier),
    /// Custom condition based on metrics
    CustomMetric(String, f64, ComparisonOperator),
}

/// Comparison operators for metric conditions
#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Actions to perform during lifecycle management
#[derive(Debug, Clone)]
pub enum LifecycleAction {
    /// Move dataset to different storage tier
    ChangeTier(StorageTier),
    /// Enable compression on dataset
    EnableCompression,
    /// Enable deduplication on dataset
    EnableDeduplication,
    /// Create point-in-time snapshot
    CreateSnapshot,
    /// Send notification to operators
    SendNotification(String),
    /// Execute custom automation script
    ExecuteScript(String),
    /// Schedule dataset for future deletion
    ScheduleDeletion(Duration),
    /// Update dataset properties
    UpdateProperties(HashMap<String, String>),
}

/// Tracked lifecycle state for a dataset
#[derive(Debug, Clone)]
pub struct DatasetLifecycleState {
    /// Dataset name
    pub dataset_name: String,
    /// Current lifecycle stage
    pub current_stage: LifecycleStage,
    /// When the dataset entered the current stage
    pub stage_entered_at: SystemTime,
    /// Last time this dataset was evaluated
    pub last_evaluated_at: SystemTime,
    /// Names of policies applied to this dataset
    pub applied_policies: Vec<String>,
    /// Actions queued for execution
    pub pending_actions: Vec<LifecycleAction>,
    /// Runtime metrics for condition evaluation
    pub metrics: HashMap<String, f64>,
}

/// Result of a lifecycle evaluation
#[derive(Debug, Clone)]
pub struct LifecycleEvaluation {
    /// Dataset name
    pub dataset_name: String,
    /// Stage at time of evaluation
    pub current_stage: LifecycleStage,
    /// Recommended new stage (if transition warranted)
    pub recommended_stage: Option<LifecycleStage>,
    /// Recommended actions for the dataset
    pub recommended_actions: Vec<LifecycleAction>,
    /// Policies that contributed to evaluation
    pub applied_policies: Vec<String>,
    /// When this evaluation was performed
    pub evaluation_timestamp: SystemTime,
    /// Scheduled time for next evaluation
    pub next_evaluation: SystemTime,
}

/// Configuration for lifecycle management
#[derive(Debug, Clone)]
pub struct LifecycleConfig {
    /// How often to evaluate dataset lifecycles
    pub evaluation_interval: Duration,
    /// Maximum number of concurrent actions
    pub max_concurrent_actions: usize,
    /// Whether to require approval for destructive actions
    pub require_approval_for_destructive: bool,
    /// Default policies to apply to new datasets
    pub default_policies: Vec<String>,
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            evaluation_interval: Duration::from_secs(
                std::env::var("NESTGATE_LIFECYCLE_EVALUATION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            ),
            max_concurrent_actions: 5,
            require_approval_for_destructive: true,
            default_policies: vec!["standard".to_string()],
        }
    }
}

/// Scheduled task for the lifecycle background worker
#[derive(Debug)]
pub enum ScheduledTask {
    EvaluateDataset(String),
    ExecuteAction(String, LifecycleAction),
    PolicyUpdate,
    StatsCollection,
}

/// Lifecycle management statistics
#[derive(Debug, Clone, Default)]
pub struct LifecycleStats {
    /// Total managed datasets
    pub total_datasets: u64,
    /// Breakdown of datasets by stage
    pub datasets_by_stage: HashMap<LifecycleStage, u64>,
    /// Total stage transitions performed
    pub total_transitions: u64,
    /// Total lifecycle actions executed
    pub total_actions_executed: u64,
    /// When the last evaluation cycle ran
    pub last_evaluation_time: Option<SystemTime>,
    /// Running average of evaluation duration
    pub average_evaluation_duration: Duration,
}
