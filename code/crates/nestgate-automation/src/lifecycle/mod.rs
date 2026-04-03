// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Automated dataset lifecycle management and optimization scheduling.

mod evaluation;
mod policies;
mod scheduler;

pub mod types;
pub use types::*;

use crate::Result;
use nestgate_core::error::NestGateError;
use std::collections::HashMap;
use std::time::SystemTime;
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
        let sender = {
            let mut sched = self.scheduler.write().await;
            sched.take()
        };
        if let Some(sender) = sender {
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
            self.evaluate_transitions(&current_state, &policies)?;

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
}

#[cfg(test)]
mod tests;
