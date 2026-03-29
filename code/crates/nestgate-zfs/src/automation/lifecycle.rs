// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module handles the complete lifecycle of datasets including stage
// transitions, condition evaluation, and automated lifecycle rule application.

//! Lifecycle module

use std::time::Duration;
use std::time::SystemTime;
use tracing::debug;
use tracing::info;
use tracing::warn;
// Removed unused tracing import
use uuid;

use super::types::{
    AutomationEvent, AutomationEventType, AutomationPolicy, DatasetLifecycle, LifecycleStage,
};
use nestgate_core::{Result, canonical_types::StorageTier as CoreStorageTier};

/// Update lifecycle stage based on policy rules
pub fn update_lifecycle_stage(
    lifecycle: &mut DatasetLifecycle,
    _policy: &AutomationPolicy,
) -> Result<()> {
    // Simple lifecycle progression logic
    let now = SystemTime::now();
    let age_days = now
        .duration_since(lifecycle.created)
        .unwrap_or(Duration::from_secs(
            std::env::var("NESTGATE_ZFS_DEFAULT_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0), // 0 seconds default (immediate)
        ))
        .as_secs()
        / (24 * 3600);
    // Update stage based on age and access patterns
    lifecycle.lifecycle_stage = match age_days {
        0..=7 => LifecycleStage::New,
        8..=30 => LifecycleStage::Active,
        31..=90 => LifecycleStage::Aging,
        _ => {
            if lifecycle.access_count < 10 {
                LifecycleStage::Archived
            } else {
                LifecycleStage::Active
            }
        }
    };

    // Keep current stage if no progression needed
    debug!(
        "Dataset {} remains in stage {:?} (age: {} days)",
        lifecycle.dataset_name, lifecycle.lifecycle_stage, age_days
    );

    Ok(())
}

/// Evaluate lifecycle conditions to determine if actions should be taken
pub fn evaluate_lifecycle_conditions(
    dataset_name: &str,
    lifecycle: &DatasetLifecycle,
    conditions: &[String],
) -> Result<bool> {
    debug!(
        "Evaluating lifecycle conditions for dataset: {}",
        dataset_name
    );
    for condition in conditions {
        if !evaluate_single_condition(dataset_name, lifecycle, condition)? {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Evaluate a single lifecycle condition
fn evaluate_single_condition(
    _dataset_name: &str,
    lifecycle: &DatasetLifecycle,
    condition: &str,
) -> Result<bool> {
    let condition_lower = condition.to_lowercase();
    if condition_lower.starts_with("age_days_gt_") {
        if let Some(threshold_str) = condition_lower.strip_prefix("age_days_gt_")
            && let Ok(threshold) = threshold_str.parse::<u64>()
        {
            let now = SystemTime::now();
            let age_days = now
                .duration_since(lifecycle.created)
                .unwrap_or(Duration::from_secs(0))
                .as_secs()
                / (24 * 3600);
            return Ok(age_days > threshold);
        }
    } else if condition_lower.starts_with("access_count_lt_") {
        if let Some(threshold_str) = condition_lower.strip_prefix("access_count_lt_")
            && let Ok(threshold) = threshold_str.parse::<u64>()
        {
            return Ok(lifecycle.access_count < threshold);
        }
    } else if condition_lower.starts_with("days_since_access_gt_") {
        if let Some(threshold_str) = condition_lower.strip_prefix("days_since_access_gt_")
            && let Ok(threshold) = threshold_str.parse::<u64>()
        {
            if let Some(last_accessed) = lifecycle.last_accessed {
                let now = SystemTime::now();
                let days_since_access = now
                    .duration_since(last_accessed)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs()
                    / (24 * 3600);
                return Ok(days_since_access > threshold);
            }
            return Ok(true); // Never accessed counts as infinite days
        }
    } else if condition_lower == "always" || condition_lower == "true" {
        return Ok(true);
    } else if condition_lower == "never" || condition_lower == "false" {
        return Ok(false);
    }

    // Default to false for unknown conditions
    warn!("Unknown lifecycle condition: {}", condition);
    Ok(false)
}

/// Check if dataset should transition to a new stage
#[must_use]
pub const fn should_transition_to_stage(
    _dataset_name: &str,
    _current_lifecycle: &DatasetLifecycle,
) -> bool {
    // Default implementation - could be made more sophisticated
    false
}
/// Transition dataset to new lifecycle stage
pub fn transition_lifecycle_stage(
    dataset_name: &str,
    new_stage: LifecycleStage,
    lifecycle_tracker: &mut std::collections::HashMap<String, DatasetLifecycle>,
) -> Result<()> {
    if let Some(lifecycle) = lifecycle_tracker.get_mut(dataset_name) {
        let old_stage = lifecycle.lifecycle_stage.clone();
        lifecycle.lifecycle_stage = new_stage.clone();
        // Add automation event
        lifecycle.automation_history.push(AutomationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: AutomationEventType::PolicyUpdate,
            timestamp: SystemTime::now(),
            details: format!("Stage transition: {old_stage:?} → {new_stage:?}"),
            success: true,
        });

        info!(
            "✅ Transitioned dataset {} from {:?} to {:?}",
            dataset_name, old_stage, new_stage
        );
    } else {
        warn!("Dataset {} not found in lifecycle tracker", dataset_name);
    }

    Ok(())
}

/// Get or create lifecycle tracking for a dataset
#[must_use]
pub fn get_or_create_lifecycle(
    dataset_name: &str,
    lifecycle_tracker: &std::collections::HashMap<String, DatasetLifecycle>,
) -> DatasetLifecycle {
    if let Some(lifecycle) = lifecycle_tracker.get(dataset_name) {
        lifecycle.clone()
    } else {
        // Create new lifecycle tracking
        DatasetLifecycle {
            dataset_name: dataset_name.to_string(),
            current_tier: CoreStorageTier::Warm, // Default to warm
            created: SystemTime::now(),
            last_accessed: None,
            access_count: 0,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::New,
            automation_history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StorageTier;
    use std::collections::HashMap;

    #[test]
    fn test_should_transition_to_stage() {
        let lifecycle = DatasetLifecycle {
            dataset_name: "pool/ds".to_string(),
            current_tier: StorageTier::Warm,
            created: SystemTime::now(),
            last_accessed: None,
            access_count: 0,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::New,
            automation_history: vec![],
        };
        assert!(!should_transition_to_stage("pool/ds", &lifecycle));
    }

    #[test]
    fn test_get_or_create_lifecycle_new() {
        let tracker = HashMap::new();
        let lifecycle = get_or_create_lifecycle("pool/newds", &tracker);
        assert_eq!(lifecycle.dataset_name, "pool/newds");
        assert!(matches!(lifecycle.lifecycle_stage, LifecycleStage::New));
    }

    #[test]
    fn test_get_or_create_lifecycle_existing() {
        let mut tracker = HashMap::new();
        let existing = DatasetLifecycle {
            dataset_name: "pool/existing".to_string(),
            current_tier: StorageTier::Hot,
            created: SystemTime::now(),
            last_accessed: None,
            access_count: 10,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::Active,
            automation_history: vec![],
        };
        tracker.insert("pool/existing".to_string(), existing);
        let lifecycle = get_or_create_lifecycle("pool/existing", &tracker);
        assert_eq!(lifecycle.access_count, 10);
        assert!(matches!(lifecycle.lifecycle_stage, LifecycleStage::Active));
    }

    #[test]
    fn test_evaluate_lifecycle_conditions_always() {
        let lifecycle = DatasetLifecycle {
            dataset_name: "pool/ds".to_string(),
            current_tier: StorageTier::Warm,
            created: SystemTime::now(),
            last_accessed: None,
            access_count: 5,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::Active,
            automation_history: vec![],
        };
        let result = evaluate_lifecycle_conditions("pool/ds", &lifecycle, &["always".to_string()]);
        assert!(result.unwrap());
    }

    #[test]
    fn test_evaluate_lifecycle_conditions_never() {
        let lifecycle = DatasetLifecycle {
            dataset_name: "pool/ds".to_string(),
            current_tier: StorageTier::Warm,
            created: SystemTime::now(),
            last_accessed: None,
            access_count: 5,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::Active,
            automation_history: vec![],
        };
        let result = evaluate_lifecycle_conditions("pool/ds", &lifecycle, &["never".to_string()]);
        assert!(!result.unwrap());
    }
}
