// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Built-in lifecycle policies and policy selection.

use std::collections::HashMap;
use std::time::Duration;

use crate::Result;
use nestgate_core::unified_enums::storage_types::StorageTier;

use super::DatasetLifecycleManager;
use super::DatasetLifecycleState;
use super::LifecycleAction;
use super::LifecyclePolicy;
use super::LifecycleStage;
use super::LifecycleTransition;
use super::TransitionCondition;

impl DatasetLifecycleManager {
    pub(super) async fn add_default_policies(&self) -> Result<()> {
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

    pub(super) async fn get_applicable_policies(
        &self,
        state: &DatasetLifecycleState,
    ) -> Vec<LifecyclePolicy> {
        let policies = self.policies.read().await;
        policies
            .iter()
            .filter(|p| p.enabled && state.applied_policies.contains(&p.name))
            .cloned()
            .collect()
    }
}
