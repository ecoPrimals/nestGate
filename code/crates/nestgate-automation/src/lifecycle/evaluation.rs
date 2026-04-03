// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Policy transition evaluation, condition checks, and stage updates.

use std::time::{Duration, SystemTime};

use tracing::info;

use crate::Result;

use super::ComparisonOperator;
use super::DatasetLifecycleManager;
use super::DatasetLifecycleState;
use super::LifecycleAction;
use super::LifecyclePolicy;
use super::LifecycleStage;
use super::TransitionCondition;

impl DatasetLifecycleManager {
    pub(super) fn evaluate_transitions(
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
                if self.evaluate_conditions(&transition.conditions, state) {
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

    pub(super) fn evaluate_conditions(
        &self,
        conditions: &[TransitionCondition],
        state: &DatasetLifecycleState,
    ) -> bool {
        conditions
            .iter()
            .all(|c| self.evaluate_single_condition(c, state))
    }

    pub(super) fn evaluate_single_condition(
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

    pub(super) async fn update_dataset_stage(
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

    pub(super) async fn update_evaluation_stats(&self, start_time: SystemTime) {
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
