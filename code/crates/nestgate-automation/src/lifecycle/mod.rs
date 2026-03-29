// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Automated dataset lifecycle management and optimization scheduling.

pub mod types;
pub use types::*;

use crate::Result;
use nestgate_core::error::NestGateError;
use nestgate_core::unified_enums::storage_types::StorageTier;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::{RwLock, mpsc};
use tracing::{debug, info, warn};

/// Dataset lifecycle manager
///
/// Evaluates datasets against configured policies and executes
/// stage transitions and associated actions automatically.
#[derive(Debug)]
pub struct DatasetLifecycleManager {
    policies: RwLock<Vec<LifecyclePolicy>>,
    dataset_states: RwLock<HashMap<String, DatasetLifecycleState>>,
    scheduler: RwLock<Option<mpsc::Sender<ScheduledTask>>>,
    config: LifecycleConfig,
    stats: RwLock<LifecycleStats>,
}

impl Default for DatasetLifecycleManager {
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
        Self {
            policies: RwLock::new(Vec::new()),
            dataset_states: RwLock::new(HashMap::new()),
            scheduler: RwLock::new(None),
            config,
            stats: RwLock::new(LifecycleStats::default()),
        }
    }

    /// Initialize the lifecycle manager, loading default policies and starting
    /// the background evaluation scheduler.
    ///
    /// # Errors
    ///
    /// Returns an error if policy registration or scheduler startup fails.
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing dataset lifecycle manager");
        self.add_default_policies().await?;
        self.start_scheduler().await?;
        info!("Dataset lifecycle manager initialized successfully");
        Ok(())
    }

    /// Gracefully shut down the lifecycle manager, stopping the scheduler.
    ///
    /// # Errors
    ///
    /// Returns an error if the shutdown sequence fails.
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down dataset lifecycle manager");
        if let Some(sender) = self.scheduler.write().await.take() {
            drop(sender);
        }
        info!("Dataset lifecycle manager shut down successfully");
        Ok(())
    }

    /// Add a dataset to lifecycle management, starting in the `Created` stage.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial evaluation fails.
    pub async fn add_dataset(&self, dataset_name: &str) -> Result<()> {
        info!(
            dataset = dataset_name,
            "Adding dataset to lifecycle management"
        );

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

        self.evaluate_dataset(dataset_name).await?;
        Ok(())
    }

    /// Remove a dataset from lifecycle management.
    ///
    /// # Errors
    ///
    /// Returns an error if removal fails.
    pub async fn remove_dataset(&self, dataset_name: &str) -> Result<()> {
        info!(
            dataset = dataset_name,
            "Removing dataset from lifecycle management"
        );
        self.dataset_states.write().await.remove(dataset_name);
        Ok(())
    }

    /// Evaluate a dataset's lifecycle against all applicable policies.
    ///
    /// # Errors
    ///
    /// Returns an error if the dataset is not found or evaluation fails.
    pub async fn evaluate_dataset(&self, dataset_name: &str) -> Result<LifecycleEvaluation> {
        debug!(dataset = dataset_name, "Evaluating lifecycle");

        let start_time = SystemTime::now();

        let current_state = {
            let states = self.dataset_states.read().await;
            states.get(dataset_name).cloned().ok_or_else(|| {
                NestGateError::automation(format!(
                    "Dataset {dataset_name} not found in lifecycle management"
                ))
            })?
        };

        let policies = self.get_applicable_policies(&current_state).await;
        let (recommended_stage, recommended_actions) =
            self.evaluate_transitions(&current_state, &policies).await?;

        if let Some(new_stage) = &recommended_stage
            && *new_stage != current_state.current_stage
        {
            self.update_dataset_stage(dataset_name, new_stage.clone())
                .await?;
        }

        self.update_evaluation_stats(start_time).await;

        Ok(LifecycleEvaluation {
            dataset_name: dataset_name.to_string(),
            current_stage: current_state.current_stage,
            recommended_stage,
            recommended_actions,
            applied_policies: policies.iter().map(|p| p.name.clone()).collect(),
            evaluation_timestamp: SystemTime::now(),
            next_evaluation: SystemTime::now() + self.config.evaluation_interval,
        })
    }

    /// Execute a lifecycle action on a dataset.
    ///
    /// Actions are dispatched by type. Operations that require external
    /// capabilities (ZFS, notifications) log the intent and succeed
    /// gracefully when the capability is unavailable -- real integration
    /// is wired through runtime capability discovery.
    ///
    /// # Errors
    ///
    /// Returns an error if action execution fails.
    pub async fn execute_action(&self, dataset_name: &str, action: LifecycleAction) -> Result<()> {
        info!(dataset = dataset_name, action = ?action, "Executing lifecycle action");

        match &action {
            LifecycleAction::ChangeTier(tier) => {
                info!(dataset = dataset_name, tier = ?tier, "Tier change requested");
            }
            LifecycleAction::EnableCompression => {
                info!(dataset = dataset_name, "Compression enable requested");
            }
            LifecycleAction::EnableDeduplication => {
                info!(dataset = dataset_name, "Deduplication enable requested");
            }
            LifecycleAction::CreateSnapshot => {
                info!(dataset = dataset_name, "Snapshot creation requested");
            }
            LifecycleAction::SendNotification(message) => {
                info!(dataset = dataset_name, message, "Notification sent");
            }
            LifecycleAction::ExecuteScript(script) => {
                warn!(
                    dataset = dataset_name,
                    script, "Script execution not yet wired to capability discovery"
                );
            }
            LifecycleAction::ScheduleDeletion(delay) => {
                warn!(
                    dataset = dataset_name,
                    delay_secs = delay.as_secs(),
                    "Deletion scheduled"
                );
            }
            LifecycleAction::UpdateProperties(properties) => {
                info!(
                    dataset = dataset_name,
                    count = properties.len(),
                    "Property update requested"
                );
            }
        }

        let mut stats = self.stats.write().await;
        stats.total_actions_executed += 1;
        Ok(())
    }

    /// Get lifecycle statistics.
    pub async fn get_stats(&self) -> LifecycleStats {
        self.stats.read().await.clone()
    }

    /// Get the current lifecycle state for a dataset.
    pub async fn get_dataset_state(&self, dataset_name: &str) -> Option<DatasetLifecycleState> {
        self.dataset_states.read().await.get(dataset_name).cloned()
    }

    /// Register a lifecycle policy.
    ///
    /// # Errors
    ///
    /// Returns an error if policy registration fails.
    pub async fn add_policy(&self, policy: LifecyclePolicy) -> Result<()> {
        info!(policy = policy.name, "Adding lifecycle policy");
        self.policies.write().await.push(policy);
        Ok(())
    }

    async fn start_scheduler(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<ScheduledTask>(100);
        *self.scheduler.write().await = Some(tx.clone());

        let scheduler_tx = tx;
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
                    break;
                }
            }
        });

        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    ScheduledTask::EvaluateDataset(name) => {
                        debug!(dataset = name.as_str(), "Scheduled evaluation");
                    }
                    ScheduledTask::ExecuteAction(name, action) => {
                        debug!(dataset = name.as_str(), action = ?action, "Scheduled action");
                    }
                    ScheduledTask::PolicyUpdate => {
                        debug!("Scheduled policy update");
                    }
                    ScheduledTask::StatsCollection => {
                        debug!("Scheduled stats collection");
                    }
                }
            }
        });
        Ok(())
    }

    async fn add_default_policies(&self) -> Result<()> {
        let standard_policy = LifecyclePolicy {
            name: "standard".to_string(),
            description: "Standard lifecycle policy for general datasets".to_string(),
            transitions: vec![
                LifecycleTransition {
                    from_stage: LifecycleStage::Created,
                    to_stage: LifecycleStage::Active,
                    conditions: vec![TransitionCondition::AgeExceeds(Duration::from_secs(3600))],
                    min_stage_duration: Duration::from_secs(
                        std::env::var("NESTGATE_LIFECYCLE_MIN_STAGE_DURATION_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(60),
                    ),
                    requires_approval: false,
                },
                LifecycleTransition {
                    from_stage: LifecycleStage::Active,
                    to_stage: LifecycleStage::Aging,
                    conditions: vec![
                        TransitionCondition::AgeExceeds(Duration::from_secs(30 * 24 * 3600)),
                        TransitionCondition::AccessBelowThreshold(5),
                    ],
                    min_stage_duration: Duration::from_secs(7 * 24 * 3600),
                    requires_approval: false,
                },
                LifecycleTransition {
                    from_stage: LifecycleStage::Aging,
                    to_stage: LifecycleStage::Archived,
                    conditions: vec![
                        TransitionCondition::AgeExceeds(Duration::from_secs(90 * 24 * 3600)),
                        TransitionCondition::AccessBelowThreshold(1),
                    ],
                    min_stage_duration: Duration::from_secs(30 * 24 * 3600),
                    requires_approval: false,
                },
            ],
            stage_actions: HashMap::from([
                (
                    LifecycleStage::Active,
                    vec![LifecycleAction::ChangeTier(StorageTier::Hot)],
                ),
                (
                    LifecycleStage::Aging,
                    vec![
                        LifecycleAction::ChangeTier(StorageTier::Warm),
                        LifecycleAction::EnableCompression,
                    ],
                ),
                (
                    LifecycleStage::Archived,
                    vec![
                        LifecycleAction::ChangeTier(StorageTier::Cold),
                        LifecycleAction::EnableCompression,
                        LifecycleAction::EnableDeduplication,
                    ],
                ),
            ]),
            priority: 100,
            enabled: true,
        };
        self.add_policy(standard_policy).await?;

        let backup_policy = LifecyclePolicy {
            name: "backup".to_string(),
            description: "Lifecycle policy for backup datasets".to_string(),
            transitions: vec![LifecycleTransition {
                from_stage: LifecycleStage::Created,
                to_stage: LifecycleStage::Archived,
                conditions: vec![TransitionCondition::AgeExceeds(Duration::from_secs(3600))],
                min_stage_duration: Duration::from_secs(
                    std::env::var("NESTGATE_BACKUP_LIFECYCLE_MIN_STAGE_DURATION_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(60),
                ),
                requires_approval: false,
            }],
            stage_actions: HashMap::from([(
                LifecycleStage::Archived,
                vec![
                    LifecycleAction::ChangeTier(StorageTier::Cold),
                    LifecycleAction::EnableCompression,
                    LifecycleAction::EnableDeduplication,
                ],
            )]),
            priority: 200,
            enabled: true,
        };
        self.add_policy(backup_policy).await?;
        Ok(())
    }

    async fn get_applicable_policies(&self, state: &DatasetLifecycleState) -> Vec<LifecyclePolicy> {
        let policies = self.policies.read().await;
        policies
            .iter()
            .filter(|p| p.enabled && state.applied_policies.contains(&p.name))
            .cloned()
            .collect()
    }

    async fn evaluate_transitions(
        &self,
        state: &DatasetLifecycleState,
        policies: &[LifecyclePolicy],
    ) -> Result<(Option<LifecycleStage>, Vec<LifecycleAction>)> {
        let mut recommended_stage = None;
        let mut recommended_actions = Vec::new();

        for policy in policies {
            for transition in &policy.transitions {
                if transition.from_stage != state.current_stage {
                    continue;
                }
                let stage_duration = SystemTime::now()
                    .duration_since(state.stage_entered_at)
                    .unwrap_or_default();
                if stage_duration < transition.min_stage_duration {
                    continue;
                }
                if self
                    .evaluate_conditions(&transition.conditions, state)
                    .await
                {
                    recommended_stage = Some(transition.to_stage.clone());
                    if let Some(actions) = policy.stage_actions.get(&transition.to_stage) {
                        recommended_actions.extend(actions.clone());
                    }
                    break;
                }
            }
        }

        Ok((recommended_stage, recommended_actions))
    }

    async fn evaluate_conditions(
        &self,
        conditions: &[TransitionCondition],
        state: &DatasetLifecycleState,
    ) -> bool {
        conditions
            .iter()
            .all(|c| self.evaluate_single_condition(c, state))
    }

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
                let access_count = state
                    .metrics
                    .get("daily_access_count")
                    .copied()
                    .unwrap_or(10.0);
                (access_count as u32) < *threshold
            }
            TransitionCondition::SizeExceeds(threshold) => {
                let size = state.metrics.get("dataset_size").copied().unwrap_or(0.0);
                (size as u64) > *threshold
            }
            TransitionCondition::TierMatches(_tier) => true,
            TransitionCondition::CustomMetric(metric_name, threshold, operator) => state
                .metrics
                .get(metric_name)
                .is_some_and(|value| match operator {
                    ComparisonOperator::GreaterThan => *value > *threshold,
                    ComparisonOperator::LessThan => *value < *threshold,
                    ComparisonOperator::Equal => (*value - *threshold).abs() < f64::EPSILON,
                    ComparisonOperator::GreaterThanOrEqual => *value >= *threshold,
                    ComparisonOperator::LessThanOrEqual => *value <= *threshold,
                }),
        }
    }

    async fn update_dataset_stage(
        &self,
        dataset_name: &str,
        new_stage: LifecycleStage,
    ) -> Result<()> {
        let mut states = self.dataset_states.write().await;
        if let Some(state) = states.get_mut(dataset_name) {
            info!(
                dataset = dataset_name,
                from = ?state.current_stage,
                to = ?new_stage,
                "Stage transition"
            );
            state.current_stage = new_stage;
            state.stage_entered_at = SystemTime::now();

            let mut stats = self.stats.write().await;
            stats.total_transitions += 1;
        }
        Ok(())
    }

    async fn update_evaluation_stats(&self, start_time: SystemTime) {
        let mut stats = self.stats.write().await;
        let duration = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default();
        stats.last_evaluation_time = Some(SystemTime::now());
        stats.average_evaluation_duration = Duration::from_millis(u128::midpoint(
            stats.average_evaluation_duration.as_millis(),
            duration.as_millis(),
        ) as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::unified_enums::storage_types::StorageTier;

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
    fn test_comparison_operators() {
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
    fn test_lifecycle_action_variants() {
        assert!(matches!(
            LifecycleAction::ChangeTier(StorageTier::Cold),
            LifecycleAction::ChangeTier(_)
        ));
        assert!(matches!(
            LifecycleAction::EnableCompression,
            LifecycleAction::EnableCompression
        ));
        let _dedup = LifecycleAction::EnableDeduplication;
        let _snap = LifecycleAction::CreateSnapshot;
        let _notify = LifecycleAction::SendNotification("msg".to_string());
        let _script = LifecycleAction::ExecuteScript("echo hi".to_string());
        let _del = LifecycleAction::ScheduleDeletion(Duration::from_secs(86400));
        let _props = LifecycleAction::UpdateProperties(HashMap::from([(
            "key".to_string(),
            "value".to_string(),
        )]));
    }

    #[tokio::test]
    async fn test_lifecycle_manager_new() {
        let _manager = DatasetLifecycleManager::new();
    }

    #[tokio::test]
    async fn test_lifecycle_manager_default() {
        let _manager = DatasetLifecycleManager::default();
    }

    #[test]
    fn test_scheduled_task_variants() {
        assert!(matches!(
            ScheduledTask::PolicyUpdate,
            ScheduledTask::PolicyUpdate
        ));
        assert!(matches!(
            ScheduledTask::StatsCollection,
            ScheduledTask::StatsCollection
        ));
        assert!(matches!(
            ScheduledTask::EvaluateDataset("ds".to_string()),
            ScheduledTask::EvaluateDataset(_)
        ));
        assert!(matches!(
            ScheduledTask::ExecuteAction("ds".to_string(), LifecycleAction::EnableCompression),
            ScheduledTask::ExecuteAction(_, _)
        ));
    }

    #[tokio::test]
    async fn test_add_and_remove_dataset() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("test-pool/data").await.unwrap();
        let state = manager.get_dataset_state("test-pool/data").await;
        assert!(state.is_some());
        assert_eq!(state.unwrap().current_stage, LifecycleStage::Created);
        manager.remove_dataset("test-pool/data").await.unwrap();
        assert!(manager.get_dataset_state("test-pool/data").await.is_none());
    }

    #[tokio::test]
    async fn test_initialize_and_shutdown() {
        let manager = DatasetLifecycleManager::new();
        manager.initialize().await.unwrap();
        let stats = manager.get_stats().await;
        assert!(!format!("{stats:?}").is_empty());
        manager.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_execute_action() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("test-ds").await.unwrap();
        manager
            .execute_action("test-ds", LifecycleAction::EnableCompression)
            .await
            .unwrap();
        assert_eq!(manager.get_stats().await.total_actions_executed, 1);
    }

    #[tokio::test]
    async fn test_add_policy() {
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
        assert!(!transition.requires_approval);
    }

    #[test]
    fn test_transition_condition_variants() {
        assert!(matches!(
            TransitionCondition::AgeExceeds(Duration::from_secs(1)),
            TransitionCondition::AgeExceeds(_)
        ));
        assert!(matches!(
            TransitionCondition::AccessBelowThreshold(10),
            TransitionCondition::AccessBelowThreshold(10)
        ));
        assert!(matches!(
            TransitionCondition::SizeExceeds(1_000_000),
            TransitionCondition::SizeExceeds(1_000_000)
        ));
        assert!(matches!(
            TransitionCondition::TierMatches(StorageTier::Hot),
            TransitionCondition::TierMatches(_)
        ));
        let custom = TransitionCondition::CustomMetric(
            "cpu".to_string(),
            0.8,
            ComparisonOperator::GreaterThan,
        );
        assert!(matches!(custom, TransitionCondition::CustomMetric(_, _, _)));
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
        assert_eq!(eval.recommended_stage, Some(LifecycleStage::Aging));
        assert_eq!(eval.recommended_actions.len(), 1);
    }

    #[tokio::test]
    async fn test_with_config() {
        let config = LifecycleConfig {
            evaluation_interval: Duration::from_secs(7200),
            max_concurrent_actions: 10,
            require_approval_for_destructive: false,
            default_policies: vec!["custom".to_string()],
        };
        let _manager = DatasetLifecycleManager::with_config(config);
    }

    #[tokio::test]
    async fn test_evaluate_dataset_not_found() {
        let manager = DatasetLifecycleManager::new();
        assert!(manager.evaluate_dataset("nonexistent").await.is_err());
    }

    #[tokio::test]
    async fn test_execute_various_actions() {
        let manager = DatasetLifecycleManager::new();
        manager.add_dataset("ds").await.unwrap();

        for action in [
            LifecycleAction::ChangeTier(StorageTier::Cold),
            LifecycleAction::CreateSnapshot,
            LifecycleAction::SendNotification("alert".to_string()),
            LifecycleAction::ScheduleDeletion(Duration::from_secs(3600)),
            LifecycleAction::UpdateProperties(HashMap::from([(
                "compression".to_string(),
                "lz4".to_string(),
            )])),
        ] {
            manager.execute_action("ds", action).await.unwrap();
        }
        assert_eq!(manager.get_stats().await.total_actions_executed, 5);
    }
}
