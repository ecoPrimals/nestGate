//
// Automated dataset lifecycle management and optimization scheduling

//! Lifecycle module

use crate::Result;
use nestgate_core::error::NestGateError;
use nestgate_core::unified_enums::storage_types::StorageTier;
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::{mpsc, RwLock};
use tracing::debug;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

/// Lifecycle stage for datasets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Lifecyclestage
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
/// Lifecyclepolicy
pub struct LifecyclePolicy {
    /// Name
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
/// Lifecycletransition
pub struct LifecycleTransition {
    /// From Stage
    pub from_stage: LifecycleStage,
    /// To Stage
    pub to_stage: LifecycleStage,
    /// Conditions
    pub conditions: Vec<TransitionCondition>,
    /// Minimum time in current stage before transition
    pub min_stage_duration: Duration,
    /// Whether this transition requires manual approval
    pub requires_approval: bool,
}
/// Condition for stage transitions
#[derive(Debug, Clone)]
/// Transitioncondition
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
/// Comparison operators for conditions
#[derive(Debug, Clone)]
/// Comparisonoperator
pub enum ComparisonOperator {
    /// Greaterthan
    GreaterThan,
    /// Lessthan
    LessThan,
    /// Equal
    Equal,
    /// Greaterthanorequal
    GreaterThanOrEqual,
    /// Lessthanorequal
    LessThanOrEqual,
}
/// Actions to perform during lifecycle management
#[derive(Debug, Clone)]
/// Lifecycleaction
pub enum LifecycleAction {
    /// Move dataset to different tier
    ChangeTier(StorageTier),
    /// Enable compression
    EnableCompression,
    /// Enable deduplication
    EnableDeduplication,
    /// Create snapshot
    CreateSnapshot,
    /// Send notification
    SendNotification(String),
    /// Execute custom script
    ExecuteScript(String),
    /// Schedule deletion
    ScheduleDeletion(Duration),
    /// Update dataset properties
    UpdateProperties(HashMap<String, String>),
}
/// Dataset lifecycle state
#[derive(Debug, Clone)]
/// Datasetlifecyclestate
pub struct DatasetLifecycleState {
    /// Dataset name
    pub dataset_name: String,
    /// Current Stage
    pub current_stage: LifecycleStage,
    /// Stage Entered At
    pub stage_entered_at: SystemTime,
    /// Last Evaluated At
    pub last_evaluated_at: SystemTime,
    /// Applied Policies
    pub applied_policies: Vec<String>,
    /// Pending Actions
    pub pending_actions: Vec<LifecycleAction>,
    /// Metrics
    pub metrics: HashMap<String, f64>,
}
/// Lifecycle evaluation result
#[derive(Debug, Clone)]
/// Lifecycleevaluation
pub struct LifecycleEvaluation {
    /// Dataset name
    pub dataset_name: String,
    /// Current Stage
    pub current_stage: LifecycleStage,
    /// Recommended Stage
    pub recommended_stage: Option<LifecycleStage>,
    /// Recommended Actions
    pub recommended_actions: Vec<LifecycleAction>,
    /// Applied Policies
    pub applied_policies: Vec<String>,
    /// Evaluation Timestamp
    pub evaluation_timestamp: SystemTime,
    /// Next Evaluation
    pub next_evaluation: SystemTime,
}
/// Dataset lifecycle manager
#[derive(Debug)]
/// Manager for DatasetLifecycle operations
pub struct DatasetLifecycleManager {
    /// Active lifecycle policies
    policies: RwLock<Vec<LifecyclePolicy>>,
    /// Current state of all datasets
    dataset_states: RwLock<HashMap<String, DatasetLifecycleState>>,
    /// Scheduler for periodic evaluations
    scheduler: RwLock<Option<mpsc::Sender<ScheduledTask>>>,
    /// Configuration
    config: LifecycleConfig,
    /// Statistics
    stats: RwLock<LifecycleStats>,
}
/// Configuration for lifecycle management
#[derive(Debug, Clone)]
/// Configuration for Lifecycle
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            evaluation_interval: Duration::from_secs(
                std::env::var("NESTGATE_LIFECYCLE_EVALUATION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600), // 1 hour default
            ), // Lifecycle evaluation interval
            max_concurrent_actions: 5,
            require_approval_for_destructive: true,
            default_policies: vec!["standard".to_string()],
        }
    }
}

/// Scheduled task for lifecycle management
#[derive(Debug)]
/// Scheduledtask
pub enum ScheduledTask {
    EvaluateDataset(String),
    ExecuteAction(String, LifecycleAction),
    /// Policyupdate
    PolicyUpdate,
    /// Statscollection
    StatsCollection,
}
/// Lifecycle management statistics
#[derive(Debug, Clone, Default)]
/// Lifecyclestats
pub struct LifecycleStats {
    /// Total Datasets
    pub total_datasets: u64,
    /// Datasets By Stage
    pub datasets_by_stage: HashMap<LifecycleStage, u64>,
    /// Total Transitions
    pub total_transitions: u64,
    /// Total Actions Executed
    pub total_actions_executed: u64,
    /// Last Evaluation Time
    pub last_evaluation_time: Option<SystemTime>,
    /// Average Evaluation Duration
    pub average_evaluation_duration: Duration,
}
impl Default for DatasetLifecycleManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl DatasetLifecycleManager {
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(LifecycleConfig::default())
    }

    #[must_use]
    pub fn with_config(config: LifecycleConfig) -> Self {
        let manager = Self {
            policies: RwLock::new(Vec::new()),
            dataset_states: RwLock::new(HashMap::new()),
            scheduler: RwLock::new(None),
            config,
            stats: RwLock::new(LifecycleStats::default()),
        };

        // Add default policies
        tokio::spawn(async move {
            // This would normally be done in an async context
            // manager.add_default_policies().await;
        });

        manager
    }

    /// Initialize the lifecycle manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing dataset lifecycle manager");

        // Add default policies
        self.add_default_policies().await?;

        // Start scheduler
        self.start_scheduler().await?;

        info!("Dataset lifecycle manager initialized successfully");
        Ok(())
    }

    /// Shutdown the lifecycle manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down dataset lifecycle manager");

        // Stop scheduler
        if let Some(sender) = self.scheduler.write().await.take() {
            drop(sender); // Close the channel
        }

        info!("Dataset lifecycle manager shut down successfully");
        Ok(())
    }

    /// Add a dataset to lifecycle management
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn add_dataset(&self, dataset_name: &str) -> Result<()> {
        info!("Adding dataset {} to lifecycle management", dataset_name);

        let state = DatasetLifecycleState {
            dataset_name: dataset_name.to_string(),
            current_stage: LifecycleStage::Created,
            stage_entered_at: SystemTime::now(),
            last_evaluated_at: SystemTime::now(),
            applied_policies: self.config.default_policies.clone(),
            pending_actions: Vec::new(),
            metrics: HashMap::new(),
        };

        self.dataset_states
            .write()
            .await
            .insert(dataset_name.to_string(), state);

        // Trigger initial evaluation
        self.evaluate_dataset(dataset_name).await?;
        Ok(())
    }

    /// Remove a dataset from lifecycle management
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn remove_dataset(&self, dataset_name: &str) -> Result<()> {
        info!(
            "Removing dataset {} from lifecycle management",
            dataset_name
        );
        self.dataset_states.write().await.remove(dataset_name);
        Ok(())
    }

    /// Evaluate a specific dataset's lifecycle
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn evaluate_dataset(&self, dataset_name: &str) -> Result<LifecycleEvaluation> {
        debug!("Evaluating lifecycle for dataset: {}", dataset_name);

        let start_time = SystemTime::now();

        // Get current state
        let current_state = {
            let states = self.dataset_states.read().await;
            states.get(dataset_name).cloned().ok_or_else(|| {
                NestGateError::automation(format!(
                    "Dataset {dataset_name} not found in lifecycle management"
                ))
            })?
        };

        // Get applicable policies
        let policies = self.get_applicable_policies(&current_state).await;

        // Evaluate transitions
        let (recommended_stage, recommended_actions) =
            self.evaluate_transitions(&current_state, &policies).await?;

        // Update state if stage changed
        if let Some(new_stage) = &recommended_stage {
            if *new_stage != current_state.current_stage {
                self.update_dataset_stage(dataset_name, new_stage.clone())
                    .await?;
            }
        }

        // Update statistics
        self.update_evaluation_stats(start_time).await;

        let evaluation = LifecycleEvaluation {
            dataset_name: dataset_name.to_string(),
            current_stage: current_state.current_stage,
            recommended_stage,
            recommended_actions,
            applied_policies: policies.iter().map(|p| p.name.clone()).collect(),
            evaluation_timestamp: SystemTime::now(),
            next_evaluation: SystemTime::now() + self.config.evaluation_interval,
        };

        debug!(
            "Lifecycle evaluation completed for {}: {:?}",
            dataset_name, evaluation
        );
        Ok(evaluation)
    }

    /// Execute a lifecycle action
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn execute_action(&self, dataset_name: &str, action: LifecycleAction) -> Result<()> {
        info!(
            "Executing lifecycle action for {}: {:?}",
            dataset_name, action
        );

        match action {
            LifecycleAction::ChangeTier(tier) => {
                info!("Changing tier for {} to {:?}", dataset_name, tier);
                // In a real implementation, this would integrate with ZFS manager
            }
            LifecycleAction::EnableCompression => {
                info!("Enabling compression for {}", dataset_name);
                // Integrate with ZFS compression
            }
            LifecycleAction::EnableDeduplication => {
                info!("Enabling deduplication for {}", dataset_name);
                // Integrate with ZFS deduplication
            }
            LifecycleAction::CreateSnapshot => {
                info!("Creating snapshot for {}", dataset_name);
                // Integrate with ZFS snapshots
            }
            LifecycleAction::SendNotification(message) => {
                info!("Sending notification for {}: {}", dataset_name, message);
                // Integrate with notification system
            }
            LifecycleAction::ExecuteScript(script) => {
                info!("Executing script for {}: {}", dataset_name, script);
                // Execute custom script safely
            }
            LifecycleAction::ScheduleDeletion(delay) => {
                warn!("Scheduling deletion for {} in {:?}", dataset_name, delay);
                // Schedule for future deletion
            }
            LifecycleAction::UpdateProperties(properties) => {
                info!("Updating properties for {}: {:?}", dataset_name, properties);
                // Update dataset properties
            }
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_actions_executed += 1;
        Ok(())
    }

    /// Get lifecycle statistics
    pub async fn get_stats(&self) -> LifecycleStats {
        self.stats.read().await.clone()
    }

    /// Get dataset lifecycle state
    pub async fn get_dataset_state(&self, dataset_name: &str) -> Option<DatasetLifecycleState> {
        self.dataset_states.read().await.get(dataset_name).cloned()
    }

    /// Add a lifecycle policy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn add_policy(&self, policy: LifecyclePolicy) -> Result<()> {
        info!("Adding lifecycle policy: {}", policy.name);
        self.policies.write().await.push(policy);
        Ok(())
    }

    /// Start the background scheduler
    async fn start_scheduler(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<ScheduledTask>(100);
        *self.scheduler.write().await = Some(tx.clone());

        // Spawn scheduler task
        let scheduler_tx = tx.clone();
        let evaluation_interval = self.config.evaluation_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(evaluation_interval);

            loop {
                interval.tick().await;
                if scheduler_tx
                    .send(ScheduledTask::PolicyUpdate)
                    .await
                    .is_err()
                {
                    break; // Channel closed
                }
            }
        });

        // Spawn task processor
        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    ScheduledTask::EvaluateDataset(dataset_name) => {
                        debug!("Scheduled evaluation for dataset: {}", dataset_name);
                        // Process dataset evaluation
                    }
                    ScheduledTask::ExecuteAction(dataset_name, action) => {
                        debug!("Scheduled action for {}: {:?}", dataset_name, action);
                        // Process action execution
                    }
                    ScheduledTask::PolicyUpdate => {
                        debug!("Scheduled policy update");
                        // Process policy updates
                    }
                    ScheduledTask::StatsCollection => {
                        debug!("Scheduled stats collection");
                        // Collect statistics
                    }
                }
            }
        });
        Ok(())
    }

    /// Add default lifecycle policies
    async fn add_default_policies(&self) -> Result<()> {
        // Standard policy for general datasets
        let standard_policy = LifecyclePolicy {
            name: "standard".to_string(),
            description: "Standard lifecycle policy for general datasets".to_string(),
            transitions: vec![
                LifecycleTransition {
                    from_stage: LifecycleStage::Created,
                    to_stage: LifecycleStage::Active,
                    conditions: vec![TransitionCondition::AgeExceeds(
                        Duration::from_secs(3600), // 1 hour
                    )],
                    min_stage_duration: Duration::from_secs(
                        std::env::var("NESTGATE_LIFECYCLE_MIN_STAGE_DURATION_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(60), // 1 minute default
                    ),
                    requires_approval: false,
                },
                LifecycleTransition {
                    from_stage: LifecycleStage::Active,
                    to_stage: LifecycleStage::Aging,
                    conditions: vec![
                        TransitionCondition::AgeExceeds(Duration::from_secs(30 * 24 * 3600)), // 30 days
                        TransitionCondition::AccessBelowThreshold(5),
                    ],
                    min_stage_duration: Duration::from_secs(7 * 24 * 3600), // 7 days
                    requires_approval: false,
                },
                LifecycleTransition {
                    from_stage: LifecycleStage::Aging,
                    to_stage: LifecycleStage::Archived,
                    conditions: vec![
                        TransitionCondition::AgeExceeds(Duration::from_secs(90 * 24 * 3600)), // 90 days
                        TransitionCondition::AccessBelowThreshold(1),
                    ],
                    min_stage_duration: Duration::from_secs(30 * 24 * 3600), // 30 days
                    requires_approval: false,
                },
            ],
            stage_actions: {
                let mut actions = HashMap::new();
                actions.insert(
                    LifecycleStage::Active,
                    vec![LifecycleAction::ChangeTier(StorageTier::Hot)],
                );
                actions.insert(
                    LifecycleStage::Aging,
                    vec![
                        LifecycleAction::ChangeTier(StorageTier::Warm),
                        LifecycleAction::EnableCompression,
                    ],
                );
                actions.insert(
                    LifecycleStage::Archived,
                    vec![
                        LifecycleAction::ChangeTier(StorageTier::Cold),
                        LifecycleAction::EnableCompression,
                        LifecycleAction::EnableDeduplication,
                    ],
                );
                actions
            },
            priority: 100,
            enabled: true,
        };

        self.add_policy(standard_policy).await?;

        // Backup policy for backup datasets
        let backup_policy = LifecyclePolicy {
            name: "backup".to_string(),
            description: "Lifecycle policy for backup datasets".to_string(),
            transitions: vec![LifecycleTransition {
                from_stage: LifecycleStage::Created,
                to_stage: LifecycleStage::Archived,
                conditions: vec![TransitionCondition::AgeExceeds(
                    Duration::from_secs(3600), // 1 hour
                )],
                min_stage_duration: Duration::from_secs(
                    std::env::var("NESTGATE_BACKUP_LIFECYCLE_MIN_STAGE_DURATION_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(60), // 1 minute default
                ),
                requires_approval: false,
            }],
            stage_actions: {
                let mut actions = HashMap::new();
                actions.insert(
                    LifecycleStage::Archived,
                    vec![
                        LifecycleAction::ChangeTier(StorageTier::Cold),
                        LifecycleAction::EnableCompression,
                        LifecycleAction::EnableDeduplication,
                    ],
                );
                actions
            },
            priority: 200,
            enabled: true,
        };

        self.add_policy(backup_policy).await?;
        Ok(())
    }

    /// Get applicable policies for a dataset
    async fn get_applicable_policies(&self, state: &DatasetLifecycleState) -> Vec<LifecyclePolicy> {
        let policies = self.policies.read().await;
        policies
            .iter()
            .filter(|p| p.enabled && state.applied_policies.contains(&p.name))
            .cloned()
            .collect()
    }

    /// Evaluate stage transitions for a dataset
    async fn evaluate_transitions(
        &self,
        state: &DatasetLifecycleState,
        policies: &[LifecyclePolicy],
    ) -> Result<(Option<LifecycleStage>, Vec<LifecycleAction>)> {
        let mut recommended_stage = None;
        let mut recommended_actions = Vec::new();

        for policy in policies {
            for transition in &policy.transitions {
                if transition.from_stage == state.current_stage {
                    // Check if minimum stage duration has passed
                    let stage_duration = SystemTime::now()
                        .duration_since(state.stage_entered_at)
                        .unwrap_or_default();
                    if stage_duration < transition.min_stage_duration {
                        continue;
                    }

                    // Check all conditions
                    if self
                        .evaluate_conditions(&transition.conditions, state)
                        .await
                    {
                        recommended_stage = Some(transition.to_stage.clone());

                        // Get actions for the new stage
                        if let Some(actions) = policy.stage_actions.get(&transition.to_stage) {
                            recommended_actions.extend(actions.clone());
                        }

                        break; // Use first matching transition
                    }
                }
            }
        }

        Ok((recommended_stage, recommended_actions))
    }

    /// Evaluate transition conditions
    async fn evaluate_conditions(
        &self,
        conditions: &[TransitionCondition],
        state: &DatasetLifecycleState,
    ) -> bool {
        for condition in conditions {
            if !self.evaluate_single_condition(condition, state) {
                return false; // All conditions must be true
            }
        }
        true
    }

    /// Evaluate a single transition condition
    fn evaluate_single_condition(
        &self,
        condition: &TransitionCondition,
        state: &DatasetLifecycleState,
    ) -> bool {
        match condition {
            TransitionCondition::AgeExceeds(threshold) => {
                let age = SystemTime::now()
                    .duration_since(state.stage_entered_at)
                    .unwrap_or_default();
                age > *threshold
            }
            TransitionCondition::AccessBelowThreshold(threshold) => {
                // In a real implementation, this would check actual access patterns
                let access_count = state.metrics.get("daily_access_count").unwrap_or(&10.0);
                (*access_count as u32) < *threshold
            }
            TransitionCondition::SizeExceeds(threshold) => {
                let size = state.metrics.get("dataset_size").unwrap_or(&0.0);
                (*size as u64) > *threshold
            }
            TransitionCondition::TierMatches(_tier) => {
                // In a real implementation, this would check current tier
                true
            }
            TransitionCondition::CustomMetric(metric_name, threshold, operator) => {
                if let Some(value) = state.metrics.get(metric_name) {
                    match operator {
                        ComparisonOperator::GreaterThan => *value > *threshold,
                        ComparisonOperator::LessThan => *value < *threshold,
                        ComparisonOperator::Equal => (*value - *threshold).abs() < f64::EPSILON,
                        ComparisonOperator::GreaterThanOrEqual => *value >= *threshold,
                        ComparisonOperator::LessThanOrEqual => *value <= *threshold,
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Update dataset stage
    async fn update_dataset_stage(
        &self,
        dataset_name: &str,
        new_stage: LifecycleStage,
    ) -> Result<()> {
        let mut states = self.dataset_states.write().await;
        if let Some(state) = states.get_mut(dataset_name) {
            info!(
                "Transitioning dataset {} from {:?} to {:?}",
                dataset_name, state.current_stage, new_stage
            );
            state.current_stage = new_stage;
            state.stage_entered_at = SystemTime::now();

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.total_transitions += 1;
        }
        Ok(())
    }

    /// Update evaluation statistics
    async fn update_evaluation_stats(&self, start_time: SystemTime) {
        let mut stats = self.stats.write().await;
        let duration = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default();

        stats.last_evaluation_time = Some(SystemTime::now());
        stats.average_evaluation_duration = Duration::from_millis(
            ((stats.average_evaluation_duration.as_millis() + duration.as_millis()) / 2) as u64,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_stage_variants() {
        assert_eq!(LifecycleStage::Created, LifecycleStage::Created);
        assert_ne!(LifecycleStage::Active, LifecycleStage::Aging);
    }

    #[test]
    fn test_lifecycle_config_default() {
        let config = LifecycleConfig::default();
        assert_eq!(config.max_concurrent_actions, 5);
        assert!(config.require_approval_for_destructive);
        assert_eq!(config.default_policies[0], "standard");
    }

    #[test]
    fn test_lifecycle_stats_default() {
        let stats = LifecycleStats::default();
        assert_eq!(stats.total_datasets, 0);
        assert!(stats.last_evaluation_time.is_none());
    }

    #[test]
    fn test_comparison_operator_variants() {
        let gt = ComparisonOperator::GreaterThan;
        let lt = ComparisonOperator::LessThan;
        assert!(matches!(gt, ComparisonOperator::GreaterThan));
        assert!(matches!(lt, ComparisonOperator::LessThan));
    }

    #[test]
    fn test_lifecycle_action_variants() {
        let action1 = LifecycleAction::ChangeTier(StorageTier::Cold);
        let action2 = LifecycleAction::EnableCompression;
        assert!(matches!(action1, LifecycleAction::ChangeTier(_)));
        assert!(matches!(action2, LifecycleAction::EnableCompression));
    }

    #[tokio::test]
    async fn test_lifecycle_manager_new() {
        let _manager = DatasetLifecycleManager::new();
        // Manager spawns async tasks, just verify creation doesn't panic
    }

    #[tokio::test]
    async fn test_lifecycle_manager_default() {
        let _manager = DatasetLifecycleManager::default();
        // Manager spawns async tasks, just verify creation doesn't panic
    }

    #[test]
    fn test_scheduled_task_variants() {
        let task1 = ScheduledTask::PolicyUpdate;
        let task2 = ScheduledTask::StatsCollection;
        assert!(matches!(task1, ScheduledTask::PolicyUpdate));
        assert!(matches!(task2, ScheduledTask::StatsCollection));
    }

    #[tokio::test]
    async fn test_lifecycle_manager_add_and_remove_dataset() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("test-pool/data").await.unwrap();
        let state = manager.get_dataset_state("test-pool/data").await;
        assert!(state.is_some());
        assert_eq!(state.unwrap().current_stage, LifecycleStage::Created);
        manager.remove_dataset("test-pool/data").await.unwrap();
        let state = manager.get_dataset_state("test-pool/data").await;
        assert!(state.is_none());
    }

    #[tokio::test]
    async fn test_lifecycle_manager_initialize_and_shutdown() {
        let manager = DatasetLifecycleManager::new();
        manager.initialize().await.unwrap();
        let stats = manager.get_stats().await;
        assert!(format!("{:?}", stats).len() > 0);
        manager.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_lifecycle_manager_execute_action() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("test-ds").await.unwrap();
        manager
            .execute_action("test-ds", LifecycleAction::EnableCompression)
            .await
            .unwrap();
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_actions_executed, 1);
    }

    #[tokio::test]
    async fn test_lifecycle_manager_add_policy() {
        let manager = DatasetLifecycleManager::new();
        let policy = LifecyclePolicy {
            name: "custom".to_string(),
            description: "Custom policy".to_string(),
            transitions: vec![],
            stage_actions: HashMap::new(),
            priority: 50,
            enabled: true,
        };
        manager.add_policy(policy).await.unwrap();
    }

    #[test]
    fn test_lifecycle_transition_creation() {
        let transition = LifecycleTransition {
            from_stage: LifecycleStage::Active,
            to_stage: LifecycleStage::Aging,
            conditions: vec![TransitionCondition::AgeExceeds(Duration::from_secs(86400))],
            min_stage_duration: Duration::from_secs(3600),
            requires_approval: false,
        };
        assert_eq!(transition.from_stage, LifecycleStage::Active);
        assert_eq!(transition.to_stage, LifecycleStage::Aging);
        assert!(!transition.requires_approval);
    }

    #[test]
    fn test_transition_condition_variants() {
        let age = TransitionCondition::AgeExceeds(Duration::from_secs(3600));
        assert!(matches!(age, TransitionCondition::AgeExceeds(_)));

        let access = TransitionCondition::AccessBelowThreshold(10);
        assert!(matches!(
            access,
            TransitionCondition::AccessBelowThreshold(10)
        ));

        let size = TransitionCondition::SizeExceeds(1_000_000);
        assert!(matches!(size, TransitionCondition::SizeExceeds(1_000_000)));

        let tier = TransitionCondition::TierMatches(StorageTier::Cold);
        assert!(matches!(tier, TransitionCondition::TierMatches(_)));

        let custom = TransitionCondition::CustomMetric(
            "cpu".to_string(),
            0.8,
            ComparisonOperator::GreaterThan,
        );
        assert!(matches!(custom, TransitionCondition::CustomMetric(_, _, _)));
    }

    #[test]
    fn test_comparison_operator_all_variants() {
        assert!(matches!(
            ComparisonOperator::GreaterThan,
            ComparisonOperator::GreaterThan
        ));
        assert!(matches!(
            ComparisonOperator::LessThan,
            ComparisonOperator::LessThan
        ));
        assert!(matches!(
            ComparisonOperator::Equal,
            ComparisonOperator::Equal
        ));
        assert!(matches!(
            ComparisonOperator::GreaterThanOrEqual,
            ComparisonOperator::GreaterThanOrEqual
        ));
        assert!(matches!(
            ComparisonOperator::LessThanOrEqual,
            ComparisonOperator::LessThanOrEqual
        ));
    }

    #[test]
    fn test_lifecycle_action_all_variants() {
        let _tier = LifecycleAction::ChangeTier(StorageTier::Hot);
        let _comp = LifecycleAction::EnableCompression;
        let _dedup = LifecycleAction::EnableDeduplication;
        let _snap = LifecycleAction::CreateSnapshot;
        let _notify = LifecycleAction::SendNotification("msg".to_string());
        let _script = LifecycleAction::ExecuteScript("echo hi".to_string());
        let _del = LifecycleAction::ScheduleDeletion(Duration::from_secs(86400));
        let mut props = HashMap::new();
        props.insert("key".to_string(), "value".to_string());
        let _props = LifecycleAction::UpdateProperties(props);
    }

    #[test]
    fn test_dataset_lifecycle_state_creation() {
        let state = DatasetLifecycleState {
            dataset_name: "pool/ds".to_string(),
            current_stage: LifecycleStage::Created,
            stage_entered_at: SystemTime::now(),
            last_evaluated_at: SystemTime::now(),
            applied_policies: vec!["standard".to_string()],
            pending_actions: vec![],
            metrics: HashMap::new(),
        };
        assert_eq!(state.dataset_name, "pool/ds");
        assert_eq!(state.current_stage, LifecycleStage::Created);
        assert_eq!(state.applied_policies.len(), 1);
    }

    #[test]
    fn test_lifecycle_evaluation_creation() {
        let eval = LifecycleEvaluation {
            dataset_name: "pool/ds".to_string(),
            current_stage: LifecycleStage::Active,
            recommended_stage: Some(LifecycleStage::Aging),
            recommended_actions: vec![LifecycleAction::EnableCompression],
            applied_policies: vec!["standard".to_string()],
            evaluation_timestamp: SystemTime::now(),
            next_evaluation: SystemTime::now() + Duration::from_secs(3600),
        };
        assert_eq!(eval.dataset_name, "pool/ds");
        assert_eq!(eval.recommended_stage, Some(LifecycleStage::Aging));
        assert_eq!(eval.recommended_actions.len(), 1);
    }

    #[tokio::test]
    async fn test_lifecycle_manager_with_config() {
        let config = LifecycleConfig {
            evaluation_interval: Duration::from_secs(7200),
            max_concurrent_actions: 10,
            require_approval_for_destructive: false,
            default_policies: vec!["custom".to_string()],
        };
        let manager = DatasetLifecycleManager::with_config(config);
        let _ = manager;
    }

    #[tokio::test]
    async fn test_evaluate_dataset_not_found() {
        let manager = DatasetLifecycleManager::new();
        let result = manager.evaluate_dataset("nonexistent-dataset").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_action_change_tier() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("ds1").await.unwrap();
        manager
            .execute_action("ds1", LifecycleAction::ChangeTier(StorageTier::Cold))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_execute_action_create_snapshot() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("ds2").await.unwrap();
        manager
            .execute_action("ds2", LifecycleAction::CreateSnapshot)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_execute_action_send_notification() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("ds3").await.unwrap();
        manager
            .execute_action(
                "ds3",
                LifecycleAction::SendNotification("alert".to_string()),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_execute_action_schedule_deletion() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("ds4").await.unwrap();
        manager
            .execute_action(
                "ds4",
                LifecycleAction::ScheduleDeletion(Duration::from_secs(3600)),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_execute_action_update_properties() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("ds5").await.unwrap();
        let mut props = HashMap::new();
        props.insert("compression".to_string(), "lz4".to_string());
        manager
            .execute_action("ds5", LifecycleAction::UpdateProperties(props))
            .await
            .unwrap();
    }

    #[test]
    fn test_scheduled_task_evaluate_dataset() {
        let task = ScheduledTask::EvaluateDataset("pool/ds".to_string());
        assert!(matches!(task, ScheduledTask::EvaluateDataset(_)));
    }

    #[test]
    fn test_scheduled_task_execute_action() {
        let task =
            ScheduledTask::ExecuteAction("pool/ds".to_string(), LifecycleAction::EnableCompression);
        assert!(matches!(task, ScheduledTask::ExecuteAction(_, _)));
    }
}
