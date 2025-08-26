//
// This module handles the complete lifecycle of datasets including stage
// transitions, condition evaluation, and automated lifecycle rule application.

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
use nestgate_core::{types::StorageTier as CoreStorageTier, Result};

/// Update lifecycle stage based on policy rules
pub async fn update_lifecycle_stage(
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
pub async fn evaluate_lifecycle_conditions(
    dataset_name: &str,
    lifecycle: &DatasetLifecycle,
    conditions: &[String],
) -> Result<bool> {
    debug!(
        "Evaluating lifecycle conditions for dataset: {}",
        dataset_name
    );

    for condition in conditions {
        if !evaluate_single_condition(dataset_name, lifecycle, condition).await? {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Evaluate a single lifecycle condition
async fn evaluate_single_condition(
    _dataset_name: &str,
    lifecycle: &DatasetLifecycle,
    condition: &str,
) -> Result<bool> {
    let condition_lower = condition.to_lowercase();

    if condition_lower.starts_with("age_days_gt_") {
        if let Some(threshold_str) = condition_lower.strip_prefix("age_days_gt_") {
            if let Ok(threshold) = threshold_str.parse::<u64>() {
                let now = SystemTime::now();
                let age_days = now
                    .duration_since(lifecycle.created)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs()
                    / (24 * 3600);
                return Ok(age_days > threshold);
            }
        }
    } else if condition_lower.starts_with("access_count_lt_") {
        if let Some(threshold_str) = condition_lower.strip_prefix("access_count_lt_") {
            if let Ok(threshold) = threshold_str.parse::<u64>() {
                return Ok(lifecycle.access_count < threshold);
            }
        }
    } else if condition_lower.starts_with("days_since_access_gt_") {
        if let Some(threshold_str) = condition_lower.strip_prefix("days_since_access_gt_") {
            if let Ok(threshold) = threshold_str.parse::<u64>() {
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
pub async fn should_transition_to_stage(
    _dataset_name: &str,
    _current_lifecycle: &DatasetLifecycle,
) -> bool {
    // Default implementation - could be made more sophisticated
    false
}

/// Transition dataset to new lifecycle stage
pub async fn transition_lifecycle_stage(
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
